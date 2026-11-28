# 0004: Rootful-first privilege model

- Status: Accepted (signed off by the project owner, 2026-11-28)
- Date: 2026-11-28

## Context

See README §4.1 and §11. M2 starts real pods: mount namespaces and overlay mounts for root filesystems (ADR 0005), network namespaces, veths and a bridge (ADR 0003), and PID, IPC and UTS namespaces. Each of these needs `CAP_SYS_ADMIN` or `CAP_NET_ADMIN` in the initial namespaces, or a user namespace that grants them.

Forces:
- CI is the main user, and CI jobs run on throwaway VMs. People will run kubemu as root, and kubemu exists to make tests realistic, not to protect the host from the workloads.
- **GitHub-hosted runners give passwordless `sudo`** (`ubuntu-24.04`, kernel 7.0 azure; `ubuntu-latest` moves to 26.04 this month). Each job is a fresh VM. Ubuntu 24.04+ restricts *unprivileged* user namespaces through AppArmor (`kernel.apparmor_restrict_unprivileged_userns=1`), and GitHub keeps that restriction. Root is not affected.
- Rootless needs a user namespace with subordinate ID ranges (`newuidmap`), and its networking has to go through slirp4netns or pasta, because it can't create veths in the host namespace. Every piece (mounts, networking, cgroups) would get a second code path.
- The current Amp orbs can't create namespaces or mounts even as root (no `CAP_SYS_ADMIN`, seccomp filter). Future orbs will be privileged Docker containers, which can.
- The M1 surface (`kubemu up`, the API, DST) needs no privileges today.

## Options

1. **Rootful.** The executor runs as root. Pods get their own namespaces but no user namespace, like Docker's default.
2. **Rootless first.** The executor runs in a user namespace with subordinate IDs, and networking goes through pasta. Not simpler in CI (AppArmor), and the slower, more complex path for every M2 component.
3. **Rootful with a user namespace per pod** (idmapped mounts, container root mapped to an unprivileged host range). Protects the host, which isn't a goal, and adds ID mapping to the image store and volumes.

## Decision

**Option 1: rootful. Running as root is the expected way to use kubemu, and protecting the host from workloads is not a goal.**

- **Root is needed only for real pods.** The API, controllers and `SimExecutor` paths from M1 keep working unprivileged. When the real executor is enabled without root, kubemu fails at startup with a clear error rather than failing per pod.
- **The executor runs as root** with full capabilities. All privileged syscalls (namespace setup, mounts, netlink) live in shell crates behind `kubemu-env-real` and the `WorkloadExecutor` trait, so DST never touches them.
- **Pods behave as on a real node:** no user namespace, the default OCI capability set, and the default seccomp profile, because that's what workloads see on Kubernetes. These are chosen for realism, not as a security boundary.
- **CI runs privileged jobs on GitHub-hosted runners with `sudo`.** This amends ADR 0008: needing root no longer by itself requires self-hosted runners. Self-hosted runners stay planned for quota, runtime and nightly needs.
- **No rootless mode is planned.** It would need its own ADR if a use case appears.

## Consequences

- One code path for mounts, networking and cgroups, which keeps M2 small.
- Running real pods locally needs `sudo kubemu up` (or a Linux VM on a Mac, per the non-goals).
- A workload that escapes its container is root on the host. That's accepted: the host is expected to be a throwaway VM or container.
- Until the privileged orbs arrive, agents can build and unit-test executor code but not run it; privileged integration tests run in a CI job with `sudo -E`. Most logic (pod lifecycle, IPAM, image manifests) stays in the deterministic core where it runs anywhere.
- ADR 0008 gets a note pointing here.
