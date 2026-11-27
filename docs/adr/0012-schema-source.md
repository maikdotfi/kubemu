# 0012: Source of schemas, discovery and OpenAPI

- Status: Accepted (signed off by the project owner, 2026-11-27)
- Date: 2026-11-27

## Context

See README §5 (T0 includes discovery and OpenAPI v2/v3), §1.4. Several parts of M1 need the same facts about each resource: its group, version, kind, plural name, short names, scope (namespaced or cluster-wide), verbs and schema.
- **Discovery:** `/api`, `/apis`, the per-group-version lists, and the aggregated discovery v2 that kubectl 1.37 asks for first.
- **OpenAPI:** `/openapi/v2` and `/openapi/v3`. kubectl uses these for `explain`, client-side validation and, in some code paths, the patch strategy.
- **Strategic merge patch** (`kubemu-patch`): merge keys and strategies (`x-kubernetes-patch-merge-key`, `x-kubernetes-patch-strategy`, `x-kubernetes-list-type`).
- **Unknown-field pruning** (ADR 0010).

Upstream publishes all of this as generated files in `kubernetes/kubernetes` under `api/openapi-spec/` (`swagger.json` for v2, `v3/*.json` for v3). The patch extensions are part of them.

## Options

1. **Vendor upstream `api/openapi-spec` at v1.37.x.** Filter it at build time to the resources kubemu serves, and embed the result in the binary. Discovery is a small table generated from the same files.
2. **Generate everything from `k8s-openapi`.** That crate has the types and some metadata, but not the full OpenAPI documents or the patch extensions in a form we can serve. It would be more code, and could drift from upstream.
3. **Hand-write the schemas** for the resources we serve. Drifts immediately. Rejected.

## Decision

**Option 1.**

- A `mise` task fetches `api/openapi-spec` at the tag pinned by ADR 0007 and writes it under `crates/kubemu-api-types/openapi/`. The pin check from ADR 0007 also covers this tag.
- The vendored files are committed, so builds need no network access.
- `kubemu-api-types` embeds the filtered specs and exposes:
  - the resource table (GVK, plural, scope, verbs, short names, categories) that discovery is served from;
  - the OpenAPI v2 and v3 documents as served;
  - per-path patch metadata for `kubemu-patch`;
  - the schemas used for pruning and structural validation.
- Only resources kubemu actually serves appear in discovery and OpenAPI. Asking for anything else gets a clear `404` and an entry in the compatibility report.
- CRDs (M3) add their own entries to the same table at runtime.

## Consequences

- One source of truth that matches the Go oracle's version, because both are pinned by ADR 0007.
- Binary size grows by the compressed filtered specs. That size is tracked (README §4.4), with no hard target.
- A minor version bump refreshes the vendored files in the same commit as the other pins.
- `kubemu-patch` reads SMP metadata from the schema, not from a hand-kept table. The differential fuzz checks both sides use the same metadata.
