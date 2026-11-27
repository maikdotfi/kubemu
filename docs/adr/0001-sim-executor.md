# 0001: Simulation executor: tokio in both modes

- Status: Proposed
- Date: 2026-11-27

## Context

See README §3.1. The sim Env needs a single-threaded deterministic executor with virtual time, seeded RNG and simulated net/fs. This also decides the shape of the Tasks, Net and Fs traits in kubemu-env.

Constraint (owner decision, 2026-11-27): **kubemu uses tokio.** No custom executor, and no build-time runtime swap.

Forces:
- Principle 1.2: nondeterminism goes through `Env`, enforced by lints.
- tokio's current-thread runtime is deterministic given deterministic inputs. It's FIFO with no work stealing, and in paused-time mode time only advances when every task is idle. The remaining sources of nondeterminism are:
  1. real I/O (sockets, fs, processes, signals);
  2. the blocking pool and the multi-thread scheduler;
  3. the per-runtime RNG behind `select!` branch order;
  4. ambient clocks and RNGs outside tokio.

## Options

1. **tokio + turmoil for sim networking.** turmoil runs simulated hosts on tokio and gives tokio-shaped `TcpStream`/`TcpListener`, partitions and latency.
   - + Mature host and link model, tokio-rs maintained.
   - − Brings its own host/address model and RNG. kubemu still needs its own addressing (cluster, pod IP, Service VIP) and wants faults driven by our `FaultSchedule`.
2. **tokio + our own in-memory net behind `Env`.** The sim `Net` hands out `tokio::io::duplex` pairs routed through a small link table that the fault engine controls (drop, delay, reset, partition).
   - + One fault model, one seed, addressing that matches ours.
   - − We write the link model (small: connect/accept, datagrams, per-link faults).
3. **madsim.** Rejected: it replaces tokio at build time, which conflicts with the constraint.

## Decision

Proposed: **tokio everywhere, with option 2 for sim networking.** We can revisit turmoil when `Net` lands in M1 if our link model grows beyond a few hundred lines.

**Runtime**
- Real mode: tokio multi-thread runtime, created only in kubemu-env-real / kubemu-cli.
- Sim mode: `Builder::new_current_thread().enable_time().start_paused(true).rng_seed(RngSeed::from_bytes(&seed))`.
  - `rng_seed` makes `select!` ordering seeded. It requires `--cfg tokio_unstable`, set workspace-wide in `.cargo/config.toml`.
  - `start_paused` requires tokio's `test-util` feature. It's enabled only by kubemu-env-sim, and kubemu-cli must not depend on that crate, so the shipped binary doesn't get it.

**What code may use directly** (deterministic under the sim runtime):
- `tokio::spawn`, `tokio::task::{JoinSet, yield_now}`;
- `tokio::time::{sleep, timeout, interval, Instant}`;
- `tokio::sync::*`, `select!`, `tokio::io` traits and `duplex`.

There's no `Tasks` trait: tokio is the task API.

**Banned outside kubemu-env-real** (added to `clippy.toml`):
- `tokio::task::spawn_blocking`, `block_in_place`;
- `tokio::runtime::Builder::new_multi_thread`;
- `tokio::net::*`, `tokio::fs::*`, `tokio::process::*`, `tokio::signal::*`.

The existing std bans stay.

**Env**
- `Clock`: the mono clock is backed by `tokio::time::Instant` in both modes, so `Env::clock` and tokio timers always agree. Wall time = a fixed origin + mono in sim, `SystemTime` in real.
- `Rng`: unchanged. The tokio RNG seed is derived from the sim seed.
- `Net`: `listen`, `connect` and `bind_udp` over kubemu-level addresses, returning boxed `AsyncRead + AsyncWrite` streams. Real = `tokio::net`, sim = option 2. Lands in M1 with the API server.
- `Fs`, `Spawn`: real-only until a crate needs them; added then.

## Consequences

- We can't randomise task scheduling order the way a custom executor could, so some ordering bugs are harder to find. Mitigation: the sim inserts seeded `yield_now` and sleep jitter at `Env` seams (net reads and writes, store watch wakeups) to perturb interleavings per seed.
- The DST harness moves from the hand-rolled step loop to real tokio tasks (writers, watchers, later controllers) on the paused runtime. The trace and the determinism check stay as they are.
- Everything builds with `tokio_unstable`. tokio's unstable API surface can change between minor versions, so pin tokio and review it in the weekly dependency job.
- hyper, h2 and rustls work over tokio I/O traits unchanged. hickory-dns works with tokio directly.
- If tokio ever exposes a scheduling hook, or `rng_seed` changes, we revisit this ADR.
