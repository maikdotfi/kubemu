# 0013: Validation and defaulting

- Status: Accepted (signed off by the project owner, 2026-11-27)
- Date: 2026-11-27

## Context

See README §0 (no full conformance), §1.4, §5, §10. A real apiserver runs defaulting and validation on every write. For a Pod alone that's hundreds of rules (`SetDefaults_*`, `ValidatePod*`). kubemu has to decide how much of this to reproduce, and how.

Forces:
- kubemu exists to run real workloads in tests, not to be an apiserver. Most rules never change what a test can see.
- Some rules are observable and matter. If a default isn't filled in, controllers and clients see something different (for example `restartPolicy`, `dnsPolicy`, `imagePullPolicy`, port `protocol`, Service `type` and `sessionAffinity`). If an obviously invalid object is accepted, a test passes that would fail on a real cluster.
- The single-binary rule (§1.1) rules out calling Go at runtime.
- The Go oracle already exists and can wrap the real defaulting and validation functions.

## Options

1. **Port only what users can observe, and check every ported rule against the Go oracle.**
2. **Port all upstream defaulting and validation** for each served resource. A large, ongoing amount of work, mostly on rules no test sees.
3. **Generate Rust from the Go code.** Too ambitious for now, and the hand-written validation code doesn't generate cleanly.
4. **Call the Go code at runtime** (a sidecar process or cgo). Breaks the single-binary rule. Rejected.
5. **Structural validation only** (against the schema), with no defaulting. Controllers and clients would see missing defaults. Too little.

## Decision

**Option 1, kept minimal.**

- **Every resource:** structural validation against its schema (ADR 0012). That means types, required fields and unknown-field handling per ADR 0010. Plus the metadata rules every resource shares: names, namespaces, labels, annotations, `generateName`, immutable fields such as `uid` and `creationTimestamp`, and resourceVersion preconditions.
- **Per resource:** defaulting and validation are added only when a user can observe them, through:
  - what the object looks like when read back;
  - how kubemu's own controllers and emulated components behave;
  - an error a real cluster would return for a mistake people plausibly make.
- **Every ported rule is differentially tested** against the Go oracle's defaulting and validation, added to `testing/oracle-go`. The fuzz is limited to the rules ported, so unported rules don't fail it.
- **Anything not ported is listed** in the compatibility report as "accepted without validation", so the gap is visible (principle 1.6).
- **Revisit as the backend grows.** When kubemu starts implementing or emulating more of a resource (a new controller, real pod runtime fields in M2), the rules that become observable are ported at the same time.

## Consequences

- kubemu accepts some objects a real apiserver would reject. That's a known, documented gap, not a bug, unless a test shows it matters. A case like that then becomes a regression fixture with the rule ported.
- `testing/oracle-go` grows validation and defaulting entry points for the T0 types.
- Most per-resource work lives in `kubemu-api-types` (or a small validation module next to it), keyed by GVK. It runs in the core, so DST exercises it.
