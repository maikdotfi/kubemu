# 0010: Object model and wire format

- Status: Proposed
- Date: 2026-11-27

## Context

See README §2.2, §5, §1.6. M1 brings the API server and the T0 resources. Two questions come with it:
- What does the store hold? `kubemu-store` is generic over the value type `V`, and today only the DST harness fills it in.
- Which encodings does the API accept and return?

Forces:
- Patching works on JSON. Merge patch is done and differentially fuzzed against the Go oracle. SMP, JSON patch and SSA will be too.
- CRDs (T2, M3) are dynamic objects with no Rust type.
- Principle 1.6: fidelity is never silently degraded. If a client sends a field, it must either be kept or be dropped the way a real apiserver drops it, with the same warning or error.
- A real apiserver drops fields that aren't in a built-in type's schema. Depending on `fieldValidation` (`Ignore`, `Warn` (the default) or `Strict`), it then says nothing, returns a warning header, or rejects the request.
- Real clients can send and ask for JSON, protobuf (built-in types only) and, more recently, CBOR.

## Options

Stored form:
1. **Dynamic JSON (`serde_json::Value`)** for every object. Typed `k8s-openapi` structs are used only where code needs them.
2. **Typed `k8s-openapi` structs** for built-in types, with dynamic JSON only for CRDs. Converting to structs drops fields they don't know about, and silently. Every resource also needs its own code path.
3. **A custom value tree** (for example, `Arc`-shared nodes) to make clones and forks cheaper. Possible later if benchmarks show `Value` costs too much. Not worth it before there's a measurement.

Wire format:
- a. **JSON only.** Protobuf and CBOR requests get a clear `406`/`415`.
- b. **JSON plus protobuf** for built-in types. That doubles the encoding surface, and the protobuf schemas would have to be vendored and kept in step.

## Decision

**Option 1 and option a.**

- The store holds `serde_json::Value` for every resource, built-in or custom.
- Controllers (and any other core code that wants types) convert to and from `k8s-openapi` structs at their own boundary. They write back with patches, so fields they don't know about survive.
- Unknown fields are handled the way the apiserver handles them: pruned against the schema (ADR 0012), with `fieldValidation` honoured (`Ignore`, `Warn` with a `Warning` header, `Strict` with a `400`).
- **JSON only on the wire.**
  - An `Accept` header listing JSON among its types gets JSON.
  - A request that accepts only protobuf or CBOR gets `406 Not Acceptable`.
  - A request body in protobuf or CBOR gets `415 Unsupported Media Type`.
  - Both errors are logged loudly and go into the compatibility report.
  - YAML is a client-side concern (kubectl converts it to JSON), so it isn't served.

## Consequences

- One code path for every resource. CRDs need no special case in the store or the patch engine.
- `k8s-openapi` (feature `v1_37`, ADR 0007) becomes a dependency of `kubemu-api-types` and of `kubemu-controllers`, but not of the store.
- `serde_json::Value` uses `BTreeMap` for its objects unless the `preserve_order` feature is on. That's deterministic, but it sorts keys, so response key order is not the client's order. Clients don't depend on key order. Byte-level comparisons against a real apiserver (trace differential) normalise it.
- **To verify before this ADR is accepted:** the informer harness (M1) has to show that client-go 1.37, kubectl and Helm work with JSON only. If a common client turns out to send protobuf with no JSON fallback, this ADR is revisited.
- Option 3 stays open. The M1 budget baselines (RSS, fork time) will show whether `Value` is too expensive.
