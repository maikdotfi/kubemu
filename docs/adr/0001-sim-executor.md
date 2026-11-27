# 0001: Determinism model: synchronous deterministic core, tokio shell

- Status: Accepted (signed off by the project owner, 2026-11-27)
- Date: 2026-11-27

## Context

See README §1.2, §3.1. The original question was which deterministic executor the simulation should run on: madsim, turmoil or our own. Behind it is a bigger question: **where does determinism come from?**

Constraints and forces:
- **kubemu uses tokio** (owner decision). We don't build our own async runtime, and we don't write I/O from scratch.
- Principle 1.2: nondeterminism goes through `Env`, and the compiler and linters enforce that, not discipline.
- L0 (README §3.2): the kubemu control plane must be fully deterministic under simulation. Real workloads only need seeded, replayable *faults* (L1).
- kubemu is mostly control-plane logic: storage, watches, controllers, scheduling, patching, fault schedules. The real I/O sits at a few edges: HTTP, proxies, DNS, process spawning and files.
- Any of the executor-based approaches makes the async runtime the thing that guarantees determinism:
  - madsim swaps tokio out at build time;
  - turmoil and paused tokio depend on tokio internals (`rng_seed` behind `tokio_unstable`, the current-thread scheduler order) and can't explore interleavings;
  - a custom executor means writing our own runtime and I/O, which we've ruled out.

### Prior art: Turso

This decision copies the architecture of [Turso](https://github.com/tursodatabase/turso), the Rust rewrite of SQLite. We read it at commit [`e07cee7`](https://github.com/tursodatabase/turso/tree/e07cee7fd3bb8a6d9bc94a20f85fba09c3bc2626) and credit its authors for the design.

- **The engine (`core/`) has no async runtime.** Running a statement is a synchronous state machine: `step()` returns `Row`, `Done`, or `IO` when it's waiting on I/O ([`core/vdbe/mod.rs`](https://github.com/tursodatabase/turso/blob/e07cee7fd3bb8a6d9bc94a20f85fba09c3bc2626/core/vdbe/mod.rs)). The caller drives progress by calling `io.step()` and stepping again.
- **All effects go through one narrow, injected trait.** The `IO` trait ([`core/io/mod.rs`](https://github.com/tursodatabase/turso/blob/e07cee7fd3bb8a6d9bc94a20f85fba09c3bc2626/core/io/mod.rs)) has about ten methods:
  - opening and removing files;
  - `pread`, `pwrite` and `sync` with completion callbacks;
  - monotonic and wall clock;
  - random numbers;
  - yield and sleep.

  There are backends for unix, io_uring, Windows and in-memory.
- **Deterministic simulation wraps that trait, not a runtime.** The simulator ([`testing/simulator/runner/io.rs`](https://github.com/tursodatabase/turso/blob/e07cee7fd3bb8a6d9bc94a20f85fba09c3bc2626/testing/simulator/runner/io.rs)) wraps the real `IO`. From a seed, it injects read, write and sync faults and latency, and drives a simulated clock. Workloads and properties are generated from the same seed and replayed with `--seed`.
- **Network-facing code is plain tokio.** Servers, sync and client bindings use tokio directly and aren't simulated. For whole-system determinism, Turso uses Antithesis (a deterministic hypervisor) instead of its own code.

The lesson: own a **narrow boundary**, not a runtime. Keep the code that has to be correct deterministic by construction, free of any executor, and let an ordinary async runtime run the edges.

## Options

1. **madsim.** Rejected: it replaces tokio at build time.
2. **tokio everywhere, with paused time and `rng_seed` as the sim executor.** Determinism then depends on `tokio_unstable` and tokio's scheduler internals, which we can't seed or explore. It's fragile, and the guarantee is only as strong as tokio's undocumented behaviour.
3. **A custom executor behind `Env`.** Rejected: it means writing our own runtime and I/O.
4. **Synchronous deterministic core plus a tokio shell, following Turso.** Chosen.

## Decision

### Two layers

**Core: deterministic by construction.** Synchronous Rust with no async runtime and no direct effects. It sees time and randomness only through `Env` (`Clock`, `Rng`), and it never blocks. The core:
- takes inputs (API requests, watch events, the current time, fault decisions);
- mutates its own state;
- returns outputs and *actions* for someone else to carry out.

Core crates:
- `kubemu-env` (traits only)
- `kubemu-store`
- `kubemu-api-types`
- `kubemu-patch`
- `kubemu-controllers`
- `kubemu-scheduler`
- `kubemu-faults`
- `kubemu-snapshot` (store and metadata parts)

**Shell: tokio.** It runs the real world around the core: HTTP/2 and TLS serving, the service and ingress proxies, DNS, the pod runtime, image pulls, and the database engines. Shell code should be thin: it parses, calls the core, and carries out the actions the core returns. Any decision worth testing belongs in the core.

Shell crates:
- `kubemu-env-real`
- `kubemu-apiserver` (HTTP layer)
- `kubemu-net`
- `kubemu-ingress`
- `kubemu-runtime`
- `kubemu-image`
- `kubemu-data`
- `kubemu-cli`

### How the core is shaped

- **Step and action style.** Long-running logic is a state machine with an explicit step function, not an `async fn`. Controllers are `reconcile(observed, desired, now) -> Vec<Action>`. Actions include create, update, delete and patch on the store, and "requeue after D". Whoever drives the core (the DST harness or the tokio shell) carries out the actions and feeds the results back.
- **Time is an input.** The core reads `Env::clock` or receives `now`. Timers are "requeue at T" actions that the driver schedules, not sleeps.
- **Randomness only via `Env::rng`.** That includes randomised backoff and jitter, as with Turso's `generate_random_number`.
- **Watches are pull-based** (ADR 0002): a watcher is a cursor, and the driver decides when it reads. In the shell, a tokio `Notify` wakes waiting HTTP watch streams. In the sim, the seeded scheduler decides.
- **Narrow effect traits, added only when needed.** As in Turso, each trait is small and owned by us:
  - a `Fs`-style file trait (snapshots, dir-backed PVCs) when the first core crate needs one;
  - a database `IO` adapter implementing Turso's own `IO` trait over `Env`, so Turso gets seeded disk faults the same way Turso's own simulator does.

### How determinism is checked

- **The DST harness (`testing/sim`) drives the core directly.** In a seeded loop it chooses which client, controller or watcher steps next, which actions apply, which faults fire, and how far virtual time advances. Interleavings are explored by the seed: something tokio can't give us, and an executor-based sim only gives at much higher cost.
- The determinism check stays as it is: the same seed run twice must produce byte-identical traces. This already works today, with no executor, because `kubemu-store` is core.
- **The shell isn't part of L0.** It's covered by:
  - integration and trace-differential tests (README §10);
  - fault injection at its own points (API middleware, proxies, DNS);
  - optionally, later: running the shell on a paused current-thread tokio runtime for end-to-end sim tests, or Antithesis for whole-system determinism. Neither is a dependency of this decision.

### Enforcement

- **Core crates must not depend on tokio (or any async runtime) at all.** CI checks this with cargo-deny `bans` (`tokio` allowed only via the shell crates as `wrappers`) or an equivalent `cargo tree` check. That's added with the first tokio dependency.
- The existing clippy bans (std clock, threads, fs, net, process, `HashMap`) apply to every crate except `kubemu-env-real`, as now.
- **Shell crates can use tokio freely** for tasks, timers, sync and I/O on handles they're given. But *creating* real-world handles still goes through `kubemu-env-real`: binding listeners, connecting, opening files, spawning processes. Matching bans for `tokio::net::*::bind`/`connect`, `tokio::fs::*` and `tokio::process::*` are added to `clippy.toml` when tokio arrives. That keeps principle 1.2 true and leaves room for proxies and fault injection to wrap those handles.

## Consequences

- **No `Tasks` or `Executor` trait in `kubemu-env`.** `Env` stays `Clock` + `Rng`, plus narrow effect traits as they're needed. The README's "Net, Fs, Spawn, Executor" list is revised to match.
- **No custom runtime and no `tokio_unstable` in M0.** M0 work is the store fork (ADR 0002), the seeded fault schedule in the core, and the Go oracle. Tokio first appears with the API server in M1.
- **Controllers are written as reconcile functions that return actions,** not free-form async code. It's more discipline up front. In return:
  - every controller is testable in DST;
  - interleavings are explored per seed;
  - failures replay from the seed.
- **Logic that drifts into the shell isn't covered by DST.** Code review and the "shell stays thin" rule guard against this. When a shell bug is found, the fix is usually to move the decision into the core.
- **Code that runs inside workloads is outside this boundary.** Workloads are covered by L1 fault schedules, not by core determinism.
