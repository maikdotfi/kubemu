# 0003: Pod networking default: netns+veth vs loopback+seccomp

- Status: Accepted (signed off by the project owner, 2026-11-28)
- Date: 2026-11-28

## Context

See README §6 and §3.3. M2 needs pods that get their own IP, reach each other and Services, resolve `*.svc.cluster.local`, and are reached by the built-in ingress. Every Service hop must cross kubemu's userspace proxy so network faults can be injected there (no eBPF, no kube-proxy). Several clusters run on one host, each with its own CIDRs (§6 multi-cluster).

Forces:
- Workloads are real, unmodified images (Postgres, ingress controllers, operators). They bind `0.0.0.0:<port>`, expect their own loopback, and read their IP from the downward API or `hostname -i`.
- ADR 0004 makes kubemu root when real pods run, so creating namespaces, veths and a bridge is allowed.
- Privileged syscalls belong in shell crates behind `kubemu-env-real`; IPAM and Service/endpoint bookkeeping should be deterministic core logic so DST covers them.
- Two pods in one cluster may listen on the same port; that must work.

## Options

1. **One netns per pod, veth to a per-cluster bridge, kubemu IPAM.** What CNI's `bridge` plugin does. Each pod has a real IP and its own loopback, so port clashes and `0.0.0.0` binds behave as on Kubernetes.
2. **Loopback IPs plus seccomp.** Pods share the host netns; each gets an address in `127.0.0.0/8`, and seccomp-unotify rewrites `bind`/`connect` to it. No namespaces or links, so it's lighter and could run rootless. But it intercepts every socket call, breaks software that checks its own interfaces or uses raw sockets, netlink or `SO_BINDTODEVICE`, and a missed syscall leaks onto the host network.
3. **Delegate to CNI plugins.** Mature, but adds external binaries and config, against the single-binary rule (§1.1), and gives us no more than option 1.

## Decision

**Option 1 as the default. Option 2 stays an experimental executor idea (README §6), unscheduled.**

- **Per cluster:** a bridge `kmu<N>` with the first address of the pod CIDR as its gateway, a pod CIDR and a Service CIDR from kubemu's config (defaults distinct per cluster, e.g. `10.<64+N>.0.0/16` pods and `10.<128+N>.0.0/16` Services), and a DNS domain.
- **Per pod:** a netns created by the executor, a veth pair with one end in the pod (`eth0`, default route via the bridge) and one end attached to the bridge, plus `lo`. The pod IP is written to `status.podIP(s)`. Containers of a pod share the netns, as on Kubernetes.
- **IPAM in the core** (`kubemu-net`'s deterministic part): allocate and free pod IPs and Service `clusterIP`s from the cluster's CIDRs, recorded in the store so snapshots and forks carry them. This also closes the M1 gap \"Services get no `clusterIP`\".
- **Services:** each `clusterIP` is added as an address on the cluster bridge, and the userspace proxy listens on `clusterIP:port` for each Service port, forwarding to ready endpoints. NodePorts listen on the host. Headless Services get DNS records only. Faults hit the proxy.
- **DNS:** the built-in server listens on the bridge gateway address; pods get it in `resolv.conf` with the usual search path and `ndots:5`.
- **Egress:** one masquerade rule per pod CIDR so pods can reach the outside (package installs, external APIs), switchable off per cluster. This is the only nftables use; there is no DNAT path.
- **Cross-cluster:** separate bridges, no routing between them until M5 adds optional routing and partitions.
- **Netlink** through a Rust crate (`rtnetlink` or similar) in the shell, not the `ip` binary.

## Consequences

- Pod networking behaves like a real cluster from the workload's point of view, so a broad corpus works without per-app fixes.
- Root and `CAP_NET_ADMIN` are needed (ADR 0004), and the host gets one bridge per cluster plus one veth per pod. Cleanup must be robust: kubemu deletes its links and netns on shutdown, and on startup removes leftovers named `kmu*`.
- Pod-to-pod traffic within a cluster goes over the bridge directly and is not fault-injectable; only Service, ingress and DNS hops are. That matches README §3.3, which puts network faults at the proxies. Direct pod-IP faults would need nftables or tc later.
- Many clusters on one host means many bridge addresses (one per Service IP). Fine at test scale; revisit with nftables if benchmarks show a cost.
- Orbs can't run any of this (ADR 0004); the core IPAM is tested under DST, the netlink code in privileged CI jobs.
