# testing

| Dir | What | Status |
|---|---|---|
| `sim/` | DST harness (`kubemu-sim`): seeded runs, invariants, determinism check | store-only workload |
| `oracle-go/` | Go oracle for differential fuzzing (SMP, SSA, validation) | merge, JSON patch, SMP |
| `informer-go/` | client-go 1.37 informers against `kubemu up` under faults: list, watch-list, watch, relist after 410, bookmarks | `mise run informer` |
| `corpus/` | Helm charts, operators, golden install tests | empty |
| `conformance/` | Upstream e2e skip lists and pass-rate tracking | empty |
| `bench/` | Resource and latency budgets (README §4.4) | idle RSS, ready time, fork; baseline recorded |

Run a sweep: `mise run sim -- --seeds 1000 --check-determinism`.
Replay one seed: `cargo run -p kubemu-sim -- --seed 1234`.
