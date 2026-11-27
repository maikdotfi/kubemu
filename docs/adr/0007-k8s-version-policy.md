# 0007: Kubernetes version policy

- Status: Proposed
- Date: 2026-11-27

## Context

See README §5, §10. Which minor version the API targets and how the Go oracle, kubectl (mise.toml) and kind are pinned to it.

Facts as of 2026-11-27:
- mise pins kubectl 1.37.3.
- `k8s-openapi` 0.29.0 supports v1_33 to v1_37, and v1_37 is its latest.
- Upstream supports the three newest minors. 1.38 is expected around December 2026.

## Options

1. **One target minor, the latest that k8s-openapi supports.** Everything is pinned to it, and bumped in one PR when a new minor is supported.
2. **A support window (for example, the latest three minors) selected at runtime.** That means k8s-openapi feature juggling and an oracle and test matrix per version. A lot of cost for an emulator nobody runs in production.
3. **Pin to an older minor** (for example, the one that matches the GKE/EKS default channel). It's more conservative, but kubemu would lag the tools users actually run.

## Decision

Proposed: **option 1, a single target minor, currently 1.37.**

These are all pinned in lockstep:
| Thing | Pin |
|---|---|
| `k8s-openapi` feature | `v1_37` (workspace-level, one place) |
| Go oracle | `k8s.io/apimachinery`, `k8s.io/api` at `v0.37.x`. `sigs.k8s.io/structured-merge-diff` at the version 0.37 requires |
| kubectl (mise) | `1.37.x` |
| kind node image | `kindest/node:v1.37.x` |
| Conformance binaries | `v1.37.x` |
| `/version` endpoint | reports `1.37` with a `-kubemu` suffix in `gitVersion` |

- Patch releases are taken freely through the weekly dependency job.
- Minor bumps happen within about one month of k8s-openapi supporting the new minor, all pins in one commit. The oracle catches any change in semantics.
- Clients up to ±1 minor version must work (kubectl's own skew policy). The trace-differential corpus covers this.

## Consequences

- A CI check confirms the pins agree: k8s-openapi feature = oracle `go.mod` = mise kubectl minor.
- Users on older clusters still get a 1.37 API, which in practice is backwards compatible for GA resources.
- The Go oracle can now add real k8s dependencies (ADR gate for the M0 SMP oracle).
