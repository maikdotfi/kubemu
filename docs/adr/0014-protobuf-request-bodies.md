# 0014: Protobuf request bodies for built-in types

- Status: Accepted (signed off by the project owner, 2026-11-28)
- Date: 2026-11-28
- Amends: [0010](0010-object-model-and-wire-format.md) (wire format only; the stored form stays JSON)

## Context

ADR 0010 chose JSON only on the wire and said it would be revisited if a common client sends protobuf with no JSON fallback. That check has failed (README §12.1 step 8, §14). Observed against `kubemu up` on 2026-11-28:

- **Typed client-go clients send protobuf bodies.** kubectl 1.37.3 (`kubectl create configmap`) sends `Content-Type: application/vnd.kubernetes.protobuf` with `Accept: application/vnd.kubernetes.protobuf,application/json`. It gets a 415 and fails. It does not retry with JSON.
- **Helm 4.3.2 can't install anything.** Its first write, the release Secret, goes through the typed clientset and gets 415. This happens with and without `--server-side`.
- **Responses can stay JSON.** Every one of these clients lists `application/json` in `Accept` and decodes whatever the response `Content-Type` says.
- **kubectl wants `/openapi/v2` as protobuf for create-only kinds.** For `kubectl create -f` of SelfSubjectAccessReview with default validation, kubectl reads `/openapi/v3`, finds no PATCH operation for the kind, and falls back to `/openapi/v2`. It sends `Accept: application/com.github.proto-openapi.spec.v2@v1.0+protobuf` only, gets 406, and aborts unless the user passes `--validate=false`. A real apiserver serves this, so the same command works there.
- **What works today:** kubectl `apply`, `create -f`, `replace`, `patch`, `delete`, `get`, `wait` and watches. They use the dynamic client, which sends JSON.

The constraints still hold: the store holds JSON (0010), schemas come from one pinned upstream source (0012, 0007), compatibility is proven against oracles (§1.4), no new dependency without code that uses it.

`helm install` is a §0 goal and an M2 exit criterion. Nobody can configure the content type: kubectl has no flag for it and Helm doesn't expose one. So kubemu has to accept protobuf.

## Options

1. **Decode protobuf request bodies to JSON with a wire table generated from Go.** Keep responses in JSON.
   - The Go oracle reflects over the `k8s.io/api` and apimachinery types at the pinned version. It emits a table with one entry per message: field number → JSON name, wire type, and Go facts the `.proto` files don't carry: pointer or value, `omitempty`, `[]byte`, and the special types (`Time`, `MicroTime`, `Duration`, `Quantity`, `IntOrString`, `RawExtension`).
   - The table is vendored into `kubemu-api-types`.
   - A small hand-written protobuf wire reader in Rust walks the body using the table and produces the JSON that Go would produce from proto → struct → JSON.
2. **The same, but read vendored `generated.proto` with `protox` + `prost-reflect`.** This is the standard tooling. But `.proto` files don't say which fields Go omits when they're zero, or how the special types map to JSON. Go's protobuf marshaller writes every value field even when it's zero (`hostNetwork: false`, `namespace: ""`, `creationTimestamp` as zero seconds), so turning that into Go's JSON needs the Go facts anyway. That means a table from Go plus three new dependencies.
3. **Full protobuf:** request bodies, response encoding and watch framing. This doubles the encoding surface, and no client needs it: every observed client accepts JSON back.
4. **Stay JSON-only and document the gap.** There's no workaround for Helm or kubectl's typed commands, so this gives up `helm install`.
5. **Translate through embedded Go (cgo or a sidecar).** This breaks the single-Rust-binary principle (§1.1) and the determinism boundary.

For `/openapi/v2`:

- a. **Serve it as protobuf (gnostic `openapi_v2.Document`), produced by Go at vendor time.**
  - The Go side converts the v2 JSON that kubemu serves into protobuf bytes and commits them.
  - kubemu serves the bytes unchanged.
  - CI fails if the bytes are stale.
- b. **Encode gnostic protobuf in Rust.** That means writing and maintaining a protobuf encoder for a schema nobody else in the codebase uses.
- c. **Leave it.** Users pass `--validate=false` for create-only kinds.

## Decision

**Options 1 and a.**

- **Decoding.**
  - A request body with `Content-Type: application/vnd.kubernetes.protobuf` is decoded at the frontend's decode stage into `serde_json::Value`. The rest of the pipeline is unchanged: pruning, `fieldValidation`, defaulting, validation, store.
  - The body is the `k8s\0` magic plus a `runtime.Unknown` envelope (`typeMeta`, `raw`, `contentEncoding`, `contentType`). `raw` holds the typed message, and the envelope's `apiVersion`/`kind` pick the entry in the table.
  - This covers create, update, subresource bodies (`/status`, `/binding`, `/eviction`, `/token`) and `DeleteOptions` on delete and deletecollection.
- **Wire table.**
  - It is generated for all of `k8s.io/api`, not just T0, so T1–T3 kinds need no further wire work.
  - A `mise` task (`mise run wiretable`, next to `mise run openapi`) regenerates it.
  - `scripts/check-k8s-pins.sh` checks that the table and the OpenAPI come from the same Kubernetes tag.
  - The Rust reader is a pure function with no new dependencies.
- **Unknown or malformed input.**
  - Unknown proto field numbers are dropped silently. Go's decoder does the same, and a real apiserver applies no `fieldValidation` to protobuf bodies.
  - A body that fails to parse, or whose kind isn't in the table, gets a 400 or 415 `Status` naming the reason. That outcome is logged loudly and goes into the compatibility report.
- **Fidelity is fuzzed.**
  - A new oracle op, `proto`: the Rust side generates a random object for a kind and sends it as JSON. Go decodes it into the typed struct and returns two encodings, protobuf (with the envelope) and JSON.
  - Rust decodes the protobuf and the result must equal Go's JSON.
  - The op runs in `mise run ci` like the other fuzz ops, and every mismatch becomes a fixture.
- **Responses stay JSON.** A request whose `Accept` lists only protobuf or CBOR still gets 406. A CBOR body still gets 415. If a common client starts sending CBOR by default, this ADR is revisited the same way 0010 was.
- **`/openapi/v2`.**
  - Requests that accept the gnostic protobuf type get the vendored bytes.
  - The bytes are regenerated whenever the served v2 document changes, and CI checks they match the served JSON.
  - JSON requests are unchanged.

## Consequences

- **What this fixes:** kubectl's typed commands (`create configmap|secret|namespace|token`, `auth can-i`) and plain typed client-go clients. The informer harness (§12.1 step 8) can use client-go's defaults instead of forcing JSON. `kubectl create -f` of create-only kinds works with default validation.
- **What Helm still needs:**
  - This removes Helm's first blocker, the release Secret.
  - `helm install` with Helm 4's default `--server-side` still needs server-side apply (M3, still 415), so until M3 Helm works only with `--server-side=false`.
  - Whatever Helm hits after the Secret hasn't been observed yet. It gets checked as soon as decoding lands.
- **Protobuf and JSON requests round-trip differently.** Protobuf bodies come out faithful to Go's typed round trip (zero values omitted the way Go omits them). JSON bodies keep the README §12.2 gap (`hostNetwork: false` stays). The two can differ for the same logical object. The trace differential normalises this until the JSON gap is closed.
- **Maintenance:**
  - Two more vendored artefacts, the wire table and the v2 protobuf, are regenerated on each Kubernetes bump (ADR 0007) and pin-checked.
  - The table's size is tracked. It's expected to be a few hundred KB for all of `k8s.io/api`.
- **Unaffected:** CRDs and custom resources. Clients send JSON for them, and protobuf for a custom resource gets 415 as it does on a real apiserver.
- **Fallback:** Option 2 stays open if the hand-written reader proves fragile. It would reuse the same Go-generated facts.
- **Effort:** about one §12.1 step: a Go generator, a Rust reader with tests, a fuzz op, and the v2 bytes. It lands before the informer harness.

## Follow-up (2026-11-28, after implementation)

- Implemented as the core crate `kubemu-proto`. Its one entry point, `decode`, is called from `kubemu-api`'s write path. The wire table covers all of `k8s.io/api` (349 KB), and the `proto` fuzz op agreed with Go on 50,000 cases.
- Smoke-tested against `kubemu up`:
  - Work now: kubectl `create configmap|secret|namespace|serviceaccount`, `auth can-i`, and `helm install|upgrade|uninstall --server-side=false`.
  - `kubectl create token` still fails: it needs the `serviceaccounts/token` subresource, which kubemu doesn't serve, so the Consequences above overstated it.
  - `kubectl create -f` of create-only kinds now gets past `/openapi/v2`, but kubectl's validation then lists CRDs (M3). The Consequences above overstated this as well, and it still needs `--validate=false`.
- kube-openapi answers with `application/com.github.proto-openapi.spec.v2.v1.0+protobuf`, even when asked for the `@v1.0` spelling, because Go's MIME parser rejects `@`. It serves protobuf only when the client ranks it above JSON. kubemu does the same.
- Not done: refused bodies get a `Status` but no log line or compatibility-report entry, since kubemu has no report yet.
