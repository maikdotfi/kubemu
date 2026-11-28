# kubemu: foundation plan

> **kubemu** (pronounced *kube emu*) is a single Rust binary that emulates enough of Kubernetes to deploy and test real workloads (app, database, ingress, operators) across multiple clusters, in CI or locally. It uses a fraction of the resources of kind/k3s, and seeded fault injection is a first-class feature.

> **Status: M1 in progress (§12.1 steps 1–3 done).** Done: M0, plus vendored upstream OpenAPI (v1.37.3) with the generated T0 resource table and schema arena (`kubemu-api-types`); JSON patch and strategic merge patch ported from the apiserver's Go code and differentially fuzzed (merge, JSON patch, SMP) against the Go oracle; and the synchronous `kubemu-api` frontend (discovery, OpenAPI, CRUD/patch/watch with resourceVersion semantics, selectors, pagination, `/status`, finalizers, namespace lifecycle). **Next agent: continue at §12.1 step 4, DST through the frontend.** See §14 for ADR status.

**Developing**
```sh
mise install            # Go (oracle), kubectl, helm, kind. Rust comes from rust-toolchain.toml via rustup.
mise run ci             # fmt, clippy (incl. determinism lints), tests, k8s pins, Go oracle, differential fuzz, 1000-seed DST sweep
mise run sim -- --seed 42                 # replay a single seed
mise run fuzz -- --op smp --seed 7 --cases 1   # replay one differential-fuzz case (merge|jsonpatch|smp)
cargo run -p kubemu-cli -- help           # the kubemu binary
```
Design decisions go in [`docs/adr/`](docs/adr/).

---

## 0. Vision, scope, non-goals

**Goals**
- Run real `kubectl apply` / `helm install` payloads against one or more emulated clusters, then run high-level tests (BDD or anything else) against them.
- Use far fewer resources than kind/k3s. Many clusters per CI runner should be normal.
- Make determinism and fault injection *possible everywhere*: fully deterministic for kubemu's own logic, seeded and replayable for faults hitting workloads.
- Provide Postgres and SQLite out of the box via Turso, running in-process.
- Snapshot and fork whole clusters, so every test scenario can start from a known state in milliseconds.

**Non-goals (explicit, written in the README)**
- Linux only, period. No macOS or Windows host support. On a Mac you run it in a Linux VM.
- No production use. It's not a distribution.
- No etcd, no HA, no leader election, no separate control-plane processes.
- Resource requests and limits are accepted and stored but **not enforced**. Scheduling ignores capacity.
- No kubelet fidelity beyond what tests observe: probes, restarts, logs, exec, port-forward.
- No full conformance. Conformance is a tracked metric, not a target.

---

## 1. Core principles (hard constraints)

1. **One binary, one store.** Every cluster is a tenant in one in-process store. A "node" is a logical object plus, optionally, a forked worker process. It is never a separate control plane.
2. **The determinism boundary is enforced by the compiler and linters, not by discipline.** All nondeterminism goes through an injected `Env`: clock, RNG, network, filesystem, process spawning, task scheduling. Code outside `kubemu-env-real` can't touch the real world.
3. **Faults are data.** Every fault is a typed, serializable event in a seeded schedule. The same seed and inputs produce the same fault schedule.
4. **Compatibility is proven against oracles.** Wherever possible we compare against real Kubernetes Go code or a real apiserver instead of guessing at semantics.
5. **Efficiency is measured on every merge.** Resource budgets are CI gates. A regression fails the build.
6. **Fidelity is opt-out, never silently degraded.** Wherever kubemu substitutes or emulates something (Turso for Postgres, ignored limits), it says so loudly in logs, events and test reports.

---

## 2. Architecture

### 2.1 Process model
- **Zygote first.** At startup, before any async runtime or thread exists, the main binary forks a single-threaded zygote. Every later fork (node workers, pod init helpers) comes from the zygote, never from the multi-threaded main process.
- **Main process.** Hosts the API server, store, controllers, scheduler, service proxy, DNS, ingress and the Turso engines.
- **Pod processes.** Created through the `WorkloadExecutor` trait (see 4.1).
- **Node workers are optional.** Start with nodes as purely logical objects. Add per-node worker processes only if fault isolation needs them (for example, "kill node" means killing a worker and its pods).

### 2.2 Components
| Component | Responsibility |
|---|---|
| **Store** | Versioned in-memory object store per cluster: resourceVersion, watch fan-out, bookmarks, compaction, field/label indexes, finalizers, ownerReferences |
| **API server** | HTTP/2 + TLS, discovery, OpenAPI, CRUD, list/watch, SMP, JSON merge patch, JSON patch, server-side apply, validation, defaulting, admission (built-in + webhooks), streaming (logs/exec/attach/port-forward) |
| **Controllers** | Deployment, ReplicaSet, StatefulSet, DaemonSet, Job, CronJob, EndpointSlice, GC, Namespace lifecycle, ServiceAccount/token |
| **Scheduler** | Trivial placement: node selectors, affinity/taints at a basic level, no capacity |
| **Pod runtime** | Image fetch, rootfs assembly, namespaces, process lifecycle, probes, restart policy, logs |
| **Network** | Pod IPs, Service VIPs, userspace service proxy, cluster DNS, NetworkPolicy (later) |
| **Ingress** | Built-in Ingress controller and proxy (Gateway API later) |
| **Data** | Turso engines: Postgres wire frontend, SQLite files, snapshots |
| **Fault engine** | Seeded fault schedules applied across API, network, process, clock and DNS |
| **Snapshot engine** | Snapshot and fork of store state, databases and volumes |
| **Control API** | kubemu's own API (CRDs plus CLI) for clusters, snapshots, faults and reports |

### 2.3 Cargo workspace (initial)
```
crates/
  kubemu-env            # Env traits: Clock, Rng, plus narrow effect traits as needed
  kubemu-env-real       # real implementations (tokio, syscalls)
  kubemu-env-sim        # deterministic simulation implementations
  kubemu-store          # versioned store + watch
  kubemu-api-types      # wraps k8s-openapi, schema metadata, patch strategies
  kubemu-patch          # SMP, JSON patch, merge patch, SSA (managed fields)
  kubemu-api            # synchronous API frontend: request pipeline, discovery, admission (ADR 0011)
  kubemu-apiserver      # thin tokio HTTP shell over kubemu-api, streaming
  kubemu-controllers
  kubemu-scheduler
  kubemu-runtime        # WorkloadExecutor trait + youki-based executor
  kubemu-image          # registry client, content-addressed layer store, EROFS/overlay
  kubemu-net            # IPAM, service proxy, DNS, NetworkPolicy
  kubemu-ingress
  kubemu-data           # Turso Postgres + SQLite integration
  kubemu-faults         # fault schedule model + injectors
  kubemu-snapshot
  kubemu-cli            # the single binary
testing/
  sim/                  # DST harness, seed runners, invariants
  oracle-go/            # Go harness: SMP/SSA/validation oracle, trace recorder
  oracle-fuzz/          # Rust side of differential fuzzing against oracle-go
  corpus/               # Helm charts, operators, golden tests
  conformance/          # skip lists, result tracking
  bench/                # resource and latency benchmarks
```

---

## 3. Determinism and fault injection

### 3.1 The `Env` abstraction
Design: a **synchronous deterministic core plus a tokio shell**, modelled on Turso ([ADR 0001](docs/adr/0001-sim-executor.md)).
- **Core** crates (store, patch, controllers, scheduler, faults, snapshot) are plain synchronous state machines. They have no async runtime, take time and randomness only through `Env` (`Clock`, `Rng`), and return actions for a driver to carry out.
- **Shell** crates (API server HTTP layer, proxies, DNS, pod runtime, images, data) run on **tokio**. They stay thin: parse, call the core, carry out its actions.
- Narrow effect traits (files, a Turso `IO` adapter, ...) are added to `Env` only when a core crate needs them. There is no `Tasks`/executor trait.
- **Sim mode** is the DST harness driving the core directly: a seeded loop picks which actor steps next, which faults fire and how far virtual time moves. No simulated runtime is needed.
- **Enforcement:**
  - `clippy.toml` `disallowed-methods`/`disallowed-types` bans `Instant::now`, `SystemTime::now`, `std::thread::spawn`, `std::net::*`, `std::fs::*`, `std::process::Command` and `HashMap` with random state outside `kubemu-env-real`. When tokio arrives, creating sockets, files and processes through tokio is banned the same way.
  - Core crates must not depend on tokio or any async runtime (checked in CI).
  - Use deterministic hashers and ordered collections where iteration order leaks into behaviour.
  - CI runs a **determinism check**: the same seed run twice must produce byte-identical event traces.

### 3.2 The determinism ladder for workloads
| Level | What | When |
|---|---|---|
| **L0** | kubemu control plane fully deterministic under sim (fake pods) | From M0 |
| **L1** | Seeded **fault schedule** against real workloads: pod/node kill, network partition, latency, packet loss, connection resets, DNS failure, watch delay/drop, API errors (409/429/500/timeouts), database errors and latency, disk full | M4 |
| **L2** | Controlled inputs for workloads: seeded `getrandom` and `/dev/urandom`, virtual time where feasible (LD_PRELOAD/ptrace experiments; vDSO is the known obstacle), deterministic DNS and connection ordering through the proxy | Research track |
| **L3** | Fully deterministic execution (thread interleavings), Hermit-style | Kept *possible* via `WorkloadExecutor`; not planned |

### 3.3 Fault model
- `FaultSchedule` is a CRD in the kubemu API group: a seed, a set of fault generators (with probabilities, targets and time windows) and explicit scripted faults.
- Every injected fault is logged as a structured event with its seed, logical time and target, and attached to the test report.
- **Replay:** `kubemu replay --seed X --trace run.json` reproduces the same fault schedule against the same manifests.
- **Injection points:**
  - API server middleware
  - service proxy and ingress proxy (network faults happen here, in userspace, so no eBPF is needed)
  - DNS server
  - pod runtime (signals, OOM-kill emulation, restarts)
  - Turso wrapper (errors, latency, connection drops)
  - store (watch disconnects, resourceVersion-too-old)

---

## 4. Resource efficiency

### 4.1 Workload runtime
- **`WorkloadExecutor` trait** with three implementations:
  - `YoukiExecutor`: the default, using real OCI containers via youki's `libcontainer` with user/mount/pid/net/ipc/uts namespaces.
  - `SimExecutor`: fake pods inside DST.
  - Experimental executors later (ptrace/seccomp-controlled, WASM).
- Use seccomp-unotify for targeted syscall interception, with fault hooks and L2 experiments built on top.
- **Privilege modes:** rootful (the CI default; simplest and fastest) and rootless (later, for local use). Write an ADR up front.
- No cgroup limits by default. Use one cgroup per cluster only for accounting and cleanup.

### 4.2 Images: where the CoW/DAX ideas pay off
- A **content-addressed layer store** shared across all clusters, all runs and all pods on the host.
- Convert layers to **EROFS** (composefs-style) so identical files share page cache across every pod. Use an overlay upper dir per container for writes.
- **Lazy pulling** (nydus/stargz-style) as an optimisation track.
- A pull-through registry cache on the CI server, plus pre-seeded images for the corpus.

### 4.3 Snapshot and fork
- **Store snapshot:** a copy-on-write persistent data structure, so a fork is O(1) and diverging is cheap.
- **Database snapshot:** copy the Turso files via reflink (btrfs/XFS) or plain copy.
- **Volume snapshot:** reflink or overlay layers.
- **Live processes are not checkpointed.** A forked cluster restarts pods from their specs. CRIU is an explicit non-goal for now.
- Snapshots can be exported to object storage (S3-compatible) for warm CI starts. This is the later home for SlateDB and the NBD-over-S3 ideas.

### 4.4 Resource budgets (initial targets, validated in M1 and gated in CI after)
| Metric | Initial target |
|---|---|
| Idle empty cluster, control-plane RSS | < 20 MB |
| Empty cluster ready | < 100 ms |
| Fork from snapshot | < 50 ms |
| `helm install` reference app (app + Postgres + ingress) to Ready, warm image cache | < 3 s |
| 10 clusters with the reference app on a 4 vCPU / 8 GB runner | Stable, no OOM |
| Binary size | Tracked, no hard target |

Every merge runs a benchmark suite that compares results with the previous run. A regression above the threshold fails the build.

---

## 5. Kubernetes API scope (compatibility tiers)

| Tier | Contents | Milestone |
|---|---|---|
| **T0** | Discovery, OpenAPI v2/v3, Namespace, Pod, Service, Endpoints/EndpointSlice, ConfigMap, Secret, Event, ServiceAccount, Node (logical), list/watch with full resourceVersion semantics, SMP, JSON/merge patch, finalizers, ownerRefs + GC | M1 |
| **T1** | Deployment, ReplicaSet, StatefulSet, DaemonSet, Job, CronJob, Ingress + IngressClass, PVC (dir-backed), logs/exec/port-forward, probes, init containers | M2 |
| **T2** | CRDs (structural schemas, validation, defaulting, status subresource), server-side apply with managed fields, admission webhooks (validating and mutating), Leases, PodDisruptionBudget | M3 |
| **T3** | RBAC (Roles, ClusterRoles, Bindings, real authz on requests), NetworkPolicy, PV/StorageClass with dynamic provisioning, HPA (no-op or fake metrics), Gateway API | M5+ |

Unsupported resources return clear errors and appear in a compatibility report. They never fail silently.

---

## 6. Networking, Services, DNS, Ingress
- **Pod networking:** one netns per pod with a veth to a per-cluster bridge, and kubemu doing IPAM. The loopback-IP plus seccomp mode stays as an experimental, lighter executor option, decided in an ADR.
- **Services:** a userspace proxy owned by kubemu covering ClusterIP, NodePort and headless Services, so all traffic crosses a fault-injectable hop. No kube-proxy or iptables complexity, with nftables as an optional fast path.
- **DNS:** built-in (hickory-dns) for `*.svc.cluster.local`, pod DNS and upstream forwarding. DNS faults are injectable.
- **Ingress:** built in (Pingora or hyper). Supports host/path routing and TLS from Secrets, and exposes the cluster ingress on a host port per cluster for tests. If users deploy their own ingress controller (ingress-nginx and the like), it runs as a normal workload and the built-in one stands aside per IngressClass.
- **Multi-cluster:** each cluster gets its own CIDRs and DNS domain. Optional cross-cluster routing allows testing failover and mesh scenarios. Clusters can be partitioned from each other by faults.

---

## 7. Data: Turso for Postgres and SQLite
- **Postgres:** Turso's experimental Postgres frontend (dialect + wire protocol) runs in-process, one logical server per database.
- **SQLite:** Turso-backed database files provisioned into a volume and mounted into the pod. Snapshotted with the cluster.
- **Three ways in:**
  1. **`Database` CRD** (kubemu API group): `engine: postgres|sqlite`. Creates a Service plus a credentials Secret, or a mounted file.
  2. **Image substitution** (opt-in per cluster or per workload): a pod or StatefulSet using `postgres:*` is satisfied by Turso instead of running the container. Readiness, the Service and Secrets behave as expected.
  3. **Real Postgres image** always works as a normal workload. That's the fidelity escape hatch.
- **Fidelity guardrails:**
  - Keep a compatibility matrix of supported SQL, types, extensions, LISTEN/NOTIFY, advisory locks and so on.
  - Log unsupported statements loudly and include them in the test report.
  - Corpus tests run the same migrations (Laravel, Rails, Django, Prisma, sqlx) on Turso and on real Postgres, and diff the results.
- Faults: injectable connection drops, latency, serialization failures and disk-full errors.
- Track Turso upstream closely. Contribute fixes upstream rather than forking where possible.

---

## 8. Storage
- **M2:** PVCs bound to per-cluster directories (reflink-friendly filesystem recommended). Supports access modes in metadata only.
- **M5+:** StorageClass/PV dynamic provisioning, volume snapshots, then the object-storage-backed block/volume track (SlateDB or NBD over S3) for warm state across runs.

---

## 9. User-facing surface
- **CLI:**
  - `kubemu up` (clusters from a config file)
  - `kubemu down`
  - `kubemu snapshot save|fork|export|import`
  - `kubemu kubeconfig`
  - `kubemu faults apply`
  - `kubemu replay`
  - `kubemu report`
- **Config file** (`kubemu.yaml`): clusters, images to pre-seed, substitutions, fault defaults, seed.
- **Language-agnostic test integration:** everything is Kubernetes-style API objects (CRDs) plus a kubeconfig, so any test framework works. Ship thin helpers:
  - a Rust crate
  - a Go package
  - a GitHub Action
  - an example BDD suite (cucumber-style)
- **Test report:** JSON plus JUnit plus a human summary. Includes the seed, injected faults, compatibility warnings (substitutions, unsupported fields) and resource usage.
- **Observability:** structured logs, an event trace (the basis for replay and determinism checks), and an optional Prometheus endpoint.

---

## 10. Testing strategy

| Layer | What | Runs |
|---|---|---|
| **Unit + property tests** | proptest on store, patch and controller logic | Every PR |
| **DST simulation** | Whole multi-cluster scenarios with `SimExecutor`: random API clients, controllers, faults. Invariants are checked continuously: no lost watch events, resourceVersion monotonic, GC eventually consistent, desired replicas reached, no orphaned pods | PR: N seeds (minutes); nightly: large seed sweeps; failing seeds saved to a regression corpus |
| **Determinism check** | Same seed twice gives identical trace hashes | Every PR |
| **Differential fuzzing vs Go oracle** | Go harness (`testing/oracle-go`) wrapping `apimachinery` SMP, `structured-merge-diff` SSA, validation and defaulting. Random objects and patches fed to both, outputs diffed | PR: short run; nightly: long run |
| **Trace differential** | Recorded request sequences (kubectl, Helm, client-go informers, operators) replayed against a real apiserver (kind) and kubemu. Responses and watch streams compared after normalization | Nightly |
| **client-go informer harness** | Go test client hammering list/watch/relist/bookmarks under faults | Nightly |
| **Go codebase as a test source** | Mine `k8s.io/kubernetes` tests for cases (table-driven validation, patch and SSA fixtures) and convert them to data-driven fixtures. No direct porting of Go test code | Ongoing |
| **Conformance** | Upstream e2e conformance with a maintained skip list. Pass rate tracked over time and published | Nightly |
| **Corpus (golden)** | kubectl examples, top Helm charts, operators (cert-manager, CloudNativePG and others). Install, wait for Ready, assert via ingress/DB, uninstall cleanly | PR: small subset; nightly: full |
| **Turso fidelity** | Migration and ORM suites on Turso vs real Postgres, diffed | Nightly |
| **Fault-injection self-tests** | Reference apps with known failure behaviour. Verify faults fire and replay identically | Nightly |
| **Benchmarks** | Resource and latency budgets from 4.4, compared with the previous run | Every merge (gating) + nightly trend |
| **Soak** | Many clusters, hours of churn, leak detection (RSS, fds, mounts, netns, zombies) | Weekly |

Every failure found anywhere (DST, fuzzing, corpus) becomes a permanent regression fixture.

---

## 11. CI and nightly infrastructure (your server)

- **Forge and runners:** GitHub Actions with self-hosted runners on your server, or Forgejo if you want everything self-hosted. Hosted runners can handle cheap jobs like lint and unit tests.
- **Job isolation:** kubemu tests need root, namespaces, mounts and network setup, so every CI job runs in a throwaway systemd-nspawn container or a microVM on the server, never directly on the host. Each job gets fresh state and a guaranteed cleanup of netns, mounts and cgroups.
- **Reproducible toolchain:** a Nix flake pins the Rust toolchain, the Go oracle toolchain, kind (for the oracle apiserver), Helm, kubectl and the conformance binaries. The server is ideally NixOS-managed.
- **Shared caches on the server:**
  - sccache/cargo cache
  - a pull-through OCI registry for the corpus images
  - pre-converted EROFS layers
  - a seed and regression corpus
  - artifact storage for traces and reports
- **Pipelines:**
  - **PR:** fmt, clippy (including the determinism lints), unit/property tests, short DST sweep, determinism check, short differential fuzz, corpus subset, benchmarks (gating).
  - **Nightly:** long DST sweeps, long fuzzing, trace differential, informer harness, conformance, full corpus, Turso fidelity, fault self-tests. Failing seeds are auto-filed as issues with replay commands.
  - **Weekly:** soak tests, dependency updates (Turso, youki, k8s-openapi, the Go oracle's k8s version), performance trend report.
- **Dashboard:** a static page with the conformance pass rate, the corpus matrix, Turso compatibility, resource trends and open failing seeds.
- **Security:** the server runs untrusted chart images, so it needs network egress controls and must not hold secrets inside job sandboxes.

---

## 12. Milestones

| Milestone | Scope | Exit criteria |
|---|---|---|
| **M0: Skeleton** | Workspace, `Env` traits + real/sim impls, determinism lints, store with watch, DST harness with fake clients, CI on the server, Go oracle harness for SMP | Determinism check green; DST runs 1k seeds; first differential fuzz green |
| **M1: API core (T0)** | API server, discovery/OpenAPI, T0 resources, patching, GC, `SimExecutor` | `kubectl` get/apply/delete/watch work; informer harness green; budget baselines recorded |
| **M2: Real workloads (T1)** | youki executor, image store (EROFS + shared cache), pod networking, Services, DNS, Ingress, logs/exec/port-forward, workload controllers, dir PVCs | Reference app (app + real Postgres + ingress) installs via Helm and serves traffic; within budgets |
| **M3: Turso + operators (T2)** | Turso Postgres + SQLite, `Database` CRD, image substitution, CRDs, SSA, webhooks | Reference app on Turso; cert-manager + CloudNativePG corpus green; SSA fuzz green |
| **M4: Snapshots + faults (L1)** | Snapshot/fork, `FaultSchedule`, all injection points, replay, reports | Scenario fork < budget; fault replay identical across runs; example BDD suite with chaos |
| **M5: Multi-cluster + T3** | Cross-cluster networking/partitions, RBAC, NetworkPolicy, StorageClass | Multi-cluster failover example; RBAC conformance subset |
| **M6: Public alpha** | Docs, GitHub Action, helpers, published compatibility dashboard | External users run it in CI |
| **Research tracks** | L2 workload determinism, rootless mode, lazy pulling, object-storage-backed snapshots and volumes | Spikes with ADRs |

### 12.1 M1 work plan

M1 follows ADRs [0010](docs/adr/0010-object-model-and-wire-format.md) (JSON objects, JSON-only wire), [0011](docs/adr/0011-api-server-shell.md) (synchronous `kubemu-api` frontend, thin tokio shell, plain HTTP, no auth, one cluster per listener), [0012](docs/adr/0012-schema-source.md) (vendored upstream OpenAPI as the single schema source) and [0013](docs/adr/0013-validation-and-defaulting.md) (validate and default only what users observe, oracle-checked). Read them first. Suggested order, each step landing green on `mise run ci`:

1. **Schemas (`kubemu-api-types`).** A `mise` task fetches upstream `api/openapi-spec` at the v1.37.x tag and commits it under `crates/kubemu-api-types/openapi/`; extend `scripts/check-k8s-pins.sh` to cover the tag. Expose the T0 resource table (GVK, plural, scope, verbs, short names), the served v2/v3 documents filtered to T0, per-path SMP metadata, and schemas for pruning. Add `k8s-openapi` (feature `v1_37`) only when code uses it.
2. **Patching (`kubemu-patch`).** JSON patch and SMP in Rust, SMP metadata read from step 1. The oracle already speaks `jsonpatch` and `smp`; extend `testing/oracle-fuzz` to fuzz both.
3. **Frontend (`kubemu-api`, new core crate).** The store becomes `Store<serde_json::Value>`. Typed request and response, `Status` errors, and the pipeline: decode, built-in admission, defaulting and validation, pruning with `fieldValidation`, patch, store, encode. Plus: discovery (legacy and aggregated v2) and OpenAPI responses, list with label and field selectors, full resourceVersion semantics (`resourceVersion`/`resourceVersionMatch`, `limit`/`continue`), watch cursors with bookmarks and 410, `/status` subresource, finalizers and `deletionTimestamp`, `propagationPolicy`, `generateName`. UIDs (UUIDv4) and timestamps come from `Env`. Every request goes into the event trace.
4. **DST through the frontend.** `testing/sim` clients call `kubemu-api` instead of the store. Keep the existing invariants and determinism check; add API-level ones (resourceVersion monotonic, no lost watch events, GC eventually consistent).
5. **Controllers (`kubemu-controllers`)** as `reconcile -> Vec<Action>` (ADR 0001): garbage collector (ownerRefs; foreground, background, orphan), namespace lifecycle, default ServiceAccount per namespace, Endpoints/EndpointSlice from Services and Pods, a logical Node, and **`SimExecutor`**: the fake kubelet that moves scheduled pods to Running/Ready after a seeded delay. All run under DST.
6. **HTTP shell (`kubemu-apiserver`) and the binary.** Tokio, hyper and axum arrive here and in `kubemu-cli` only. In the same commit: the tokio bans in `clippy.toml` and the CI check that core crates don't depend on tokio (ADR 0001); listener binding goes through `kubemu-env-real`. Core behind a `Mutex`, called inline; watches wait on a `Notify` and stream chunked JSON. Shell tests drive real HTTP against an in-memory core. CLI: `kubemu up` (one cluster) and `kubemu kubeconfig` (`server: http://127.0.0.1:<port>`).
7. **Validation and defaulting (ADR 0013).** Add `validate`/`default` ops to `testing/oracle-go` for the T0 types; port only observable rules, each fuzzed against the oracle; list the rest in a compatibility report.
8. **Exit criteria.**
   - A kubectl smoke test in CI: `get`, `apply`, `delete`, `watch` against `kubemu up`.
   - An informer harness (client-go 1.37) for list/watch/relist/bookmarks. It also settles ADR 0010's open check that client-go, kubectl and Helm work with JSON only.
   - Budget baselines recorded in `testing/bench` (§4.4: idle RSS, empty-cluster-ready time, fork time).

---

## 13. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Long tail of API semantics (SSA, watch edge cases) | Oracle-driven fuzzing and trace differential from M0; explicit tiers; compatibility report |
| Turso Postgres immaturity | Opt-in substitution, real Postgres escape hatch, fidelity suite, upstream contributions |
| Determinism erosion over time | Lints + determinism check on every PR; any nondeterminism counts as a bug |
| youki/libcontainer API churn | Pin versions; keep the executor trait thin; weekly update job |
| Runtime privilege and security on CI host | Per-job nspawn/microVM sandboxes; rootful only inside sandboxes |
| Scope creep toward "real Kubernetes" | Non-goals list in the README; every new resource needs a corpus case justifying it |
| Resource budgets drifting | Gating benchmarks per merge |

---

## 14. Decisions to write as ADRs first
ADRs live in [`docs/adr/`](docs/adr/). **Every ADR that M0 and M1 need is accepted, so M1 can start.** The others aren't written yet and need sign-off before their milestone starts.

| # | Decision | Status | Needed by |
|---|---|---|---|
| [0001](docs/adr/0001-sim-executor.md) | Determinism model: synchronous deterministic core + tokio shell (Turso-style) | **Accepted** | M0 |
| [0002](docs/adr/0002-store-snapshots-and-watch-cache.md) | Store: `imbl::OrdMap` for O(1) fork, per-cluster watch log | **Accepted** | M0 |
| [0003](docs/adr/0003-pod-networking.md) | Pod networking default: netns+veth vs the loopback+seccomp experimental mode | Proposed, not written | M2 |
| [0004](docs/adr/0004-rootful-first.md) | Rootful-first privilege model | Proposed, not written | M2 |
| [0005](docs/adr/0005-image-format.md) | Image format: EROFS/composefs vs plain overlay for v1 | Proposed, not written | M2 |
| [0006](docs/adr/0006-turso-boundary.md) | Turso integration boundary: in-process API, connection model, how substitution is declared | Proposed, not written | M3 |
| [0007](docs/adr/0007-k8s-version-policy.md) | Kubernetes version policy: single target minor, **1.37** | **Accepted** | M0 |
| [0008](docs/adr/0008-ci-forge.md) | CI forge: GitHub Actions; hosted runners now, self-hosted later | **Accepted** | M0 |
| [0009](docs/adr/0009-license-and-naming.md) | License and naming (check that "kubemu" is free on crates.io and GitHub) | Proposed, not written | M6 |
| [0010](docs/adr/0010-object-model-and-wire-format.md) | Object model: JSON values in the store, JSON-only wire format | **Accepted** | M1 |
| [0011](docs/adr/0011-api-server-shell.md) | API server: synchronous `kubemu-api` frontend + thin tokio HTTP shell (Turso-style), plain HTTP, multi-cluster deferred | **Accepted** | M1 |
| [0012](docs/adr/0012-schema-source.md) | Schemas, discovery and OpenAPI from vendored upstream `api/openapi-spec` | **Accepted** | M1 |
| [0013](docs/adr/0013-validation-and-defaulting.md) | Validation and defaulting: port only what users observe, oracle-checked | **Accepted** | M1 |

---

## 15. Definition of success for v1
- A typical app (web + Postgres + ingress + one operator) goes from `helm install` to passing BDD tests in seconds, on a small runner, alongside other clusters.
- Any failing chaos run reproduces from its seed.
- Conformance pass rate and corpus coverage are published and rising.
- Resource budgets hold on every merge.
