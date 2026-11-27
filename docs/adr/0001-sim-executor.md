# 0001: Simulation executor: madsim vs turmoil vs custom

- Status: Proposed
- Date: 2026-11-27

## Context

See README §3.1. The sim Env needs a single-threaded deterministic executor with virtual time, seeded RNG and simulated net/fs. This also decides the shape of the Tasks, Net and Fs traits in kubemu-env.

Forces:
- Principle 1.2: nondeterminism goes through an injected `Env`, enforced by lints. Today `Env` is a struct of `Arc<dyn Trait>` handles (`Clock`, `Rng`).
- One binary hosts many clusters, so real mode must also run on a normal multi-threaded runtime (tokio).
- The API server needs HTTP/2 + TLS (hyper, rustls). hyper 1.x is runtime-agnostic: it needs `hyper::rt::{Read, Write, Executor, Timer}`, which we can implement on top of `Env`.
- Faults are injected in our own code (API middleware, proxies, DNS, store), not by intercepting syscalls. The sim only has to control what kubemu itself does.
- Add deps only when code uses them.

## Options

1. **madsim.** Swaps tokio and friends for simulated versions at build time (`--cfg madsim`, `madsim-tokio` and patched crates), and intercepts libc `clock_gettime`/`getrandom`.
   - + Most complete out of the box: sim net, fs, time, multi-node, determinism self-check.
   - − The whole dependency tree builds twice in two configs, and every networking crate we use needs a madsim-patched build or `[patch]` entries.
   - − Determinism comes from the build config, not from `Env`, so it fights principle 1.2 rather than building on it. The lints and `Env` would become redundant in one mode and leaky in the other.
   - − Small maintainer base; we'd be pinned to its tokio API subset.
2. **turmoil.** Runs on tokio's current-thread runtime with paused time and simulates hosts plus TCP/UDP between them.
   - + Simple and actively maintained (tokio-rs). Good network partition and latency tools.
   - − Determinism is partial: tokio internals (`select!` fairness, the task scheduler) aren't seeded by us, and there's no sim fs or process model.
   - − Code still calls tokio directly, which is what our lints ban. We'd wrap turmoil behind `Env` anyway.
3. **Custom executor behind `Env`.** Add `Tasks`, `Net` and `Fs` traits to kubemu-env. Real mode implements them with tokio. Sim mode is a small single-threaded executor built on `async-task`: a seeded run queue (the RNG picks the next ready task), a timer wheel on `SimClock`, and in-memory net/fs that the fault engine can reach directly.
   - + Matches the existing design: one seam (`Env`), enforced by lints, no build-config split.
   - + We own scheduling order, so we can randomise it per seed (finds ordering bugs) and record it in the trace.
   - + Fault injection for net/fs/time lives in the same code as the sim.
   - − We write and maintain roughly 1–2k lines: executor, timers, in-memory streams, fs.
   - − Every async dependency must go through our traits. hyper allows this; crates that hard-code tokio (some DNS and DB clients) need adapters or must stay in real-only code.

## Decision

Proposed: **option 3, custom executor behind `Env`.** Steal ideas from turmoil (host and link model) and madsim (determinism self-check) without depending on them.

Trait shape:
- `Tasks`: `spawn(name, fut) -> JoinHandle`, `sleep(Duration)`, `timeout(Duration, fut)`. Tasks are named, so traces stay readable.
- `Net`: `listen(addr) -> Listener`, `connect(addr) -> Stream` (`AsyncRead + AsyncWrite`), `bind_udp(addr) -> Datagram`. Addresses are kubemu-level (cluster, host, port), not raw `SocketAddr`, so clusters can be partitioned.
- `Fs`: a minimal async API (read, write, rename, remove, list, fsync), added only when the first crate needs it.
- `Spawn` (OS processes): real-only for now. In sim, pods are `SimExecutor` objects (README §4.1), so the sim never forks.

The trait objects stay `Send + Sync` so real mode runs on multi-threaded tokio. The sim executor runs everything on one thread anyway.

The extended `disallowed-methods` list bans `tokio::spawn`, `tokio::time::*`, `tokio::net::*` and `tokio::fs::*` outside kubemu-env-real.

## Consequences

- M0 lands `Tasks` plus the sim executor and timers. `Net` lands early in M1 with the API server. `Fs` lands when needed.
- The DST harness moves from the hand-rolled step loop to real async tasks (clients, watchers, controllers) scheduled by the seeded executor. The trace records every scheduling decision.
- Async libraries are judged on whether they work over our I/O traits. hyper, rustls and h2 do. hickory-dns needs a runtime-provider adapter.
- If the custom executor turns out to be a maintenance burden, turmoil is the fallback for the net layer only.
