# bench

Benchmarks for the resource and latency budgets in README §4.4 that exist in M1 (`kubemu-bench`). Run `mise run bench`; CI runs it on every merge.

| Metric | How | Budget | Gate |
|---|---|---|---|
| `idle_rss_mib` | RSS of `kubemu up` 2 s after ready (`/proc/PID/status`) | < 20 MiB | budget, and > 25% over the baseline |
| `empty_cluster_ready_ms` | spawn of `kubemu up` until the default ServiceAccount answers 200 over HTTP | < 100 ms | budget |
| `fork_empty_us` | `Api::fork_cluster` of a settled empty cluster | < 50 ms | budget |
| `fork_10k_objects_us` | the same with 10k ConfigMaps | < 50 ms | budget |

Each is the median of 5 runs (forks: of 50). Times are gated only by budget: shared CI runners are too noisy to compare with a baseline from another machine. Until M4 a fork is the store fork alone (ADR 0002); databases and volumes join it then.

`baseline.json` is the recorded baseline (`mise run bench-record`), from a 2-vCPU orb on 2026-11-28, release build: 4.1 MiB idle RSS, 4.1 ms to ready, under 1 µs per fork, empty or with 10k objects.
