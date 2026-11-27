# Architecture decision records

The decisions in README §14 are written here first. Copy `0000-template.md`, take the next number, and open a PR. Status moves Proposed → Accepted; superseded ADRs stay, with a pointer.

| ADR | Topic | Status |
|---|---|---|
| [0001](0001-sim-executor.md) | Determinism model: synchronous core, tokio shell | Accepted |
| [0002](0002-store-snapshots-and-watch-cache.md) | Store data structure and watch cache | Accepted |
| [0003](0003-pod-networking.md) | Pod networking default | Proposed |
| [0004](0004-rootful-first.md) | Rootful-first privilege model | Proposed |
| [0005](0005-image-format.md) | Image format | Proposed |
| [0006](0006-turso-boundary.md) | Turso integration boundary | Proposed |
| [0007](0007-k8s-version-policy.md) | Kubernetes version policy | Accepted |
| [0008](0008-ci-forge.md) | Forge choice for CI | Accepted |
| [0009](0009-license-and-naming.md) | License and naming | Proposed |
| [0010](0010-object-model-and-wire-format.md) | Object model and wire format | Proposed |
| [0011](0011-api-server-shell.md) | API server layering: synchronous frontend, thin HTTP shell | Proposed |
| [0012](0012-schema-source.md) | Source of schemas, discovery and OpenAPI | Proposed |
| [0013](0013-validation-and-defaulting.md) | Validation and defaulting | Proposed |
