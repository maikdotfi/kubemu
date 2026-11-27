# testing

| Dir | What | Status |
|---|---|---|
| `sim/` | DST harness (`kubemu-sim`): seeded runs, invariants, determinism check | store-only workload |
| `oracle-go/` | Go oracle for differential fuzzing (SMP, SSA, validation) | merge, JSON patch, SMP |
| `corpus/` | Helm charts, operators, golden install tests | empty |
| `conformance/` | Upstream e2e skip lists and pass-rate tracking | empty |
| `bench/` | Resource and latency budgets (README §4.4) | empty |

Run a sweep: `mise run sim -- --seeds 1000 --check-determinism`.
Replay one seed: `cargo run -p kubemu-sim -- --seed 1234`.
