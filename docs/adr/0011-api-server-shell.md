# 0011: API server layering: synchronous frontend, thin HTTP shell

- Status: Proposed
- Date: 2026-11-27

## Context

See README §2.2, §5, ADR 0001, ADR 0002. M1 brings the API server, which is the first tokio code (ADR 0001). The questions are where request handling lives, how the tokio HTTP layer reaches the synchronous core, and what M1 serves (transport, auth, how many clusters).

### Prior art: Turso's Postgres frontend

ADR 0001 copied Turso's engine/simulator split. Turso's Postgres support shows the same split one level up, for a network protocol. We read it at commit [`e07cee7`](https://github.com/tursodatabase/turso/tree/e07cee7fd3bb8a6d9bc94a20f85fba09c3bc2626/postgres):

- **Engine** (`turso_core`): synchronous. Concurrency is handled inside it: an `Arc<Database>` shared by all clients, one `Connection` per client.
- **Frontend** (`postgres/frontend`, crate `turso_pg`): **synchronous, no async runtime, no wire protocol.** A `PgConnection` session per client holds transaction state and settings, and implements Postgres semantics: catalog, casts, roles, statements. Errors come out typed, already carrying their SQLSTATE.
- **Server** (`postgres/server/lib.rs`, crate `turso_pg_server`): a **thin tokio shell**. The `pgwire` crate speaks the protocol. An accept loop spawns one task and one session per socket. Handlers call the synchronous frontend **inline, inside the async handler**. There's no actor, no channel and no `spawn_blocking`. Most of the server's own code converts values and errors to the wire format.
- **No TLS** (`process_socket(socket, None, …)`).
- Server tests drive the real wire protocol against an in-memory database.

## Options

Layering:
1. **Put request handling in `kubemu-apiserver`**, next to the HTTP code. API semantics would then sit in the shell, outside DST.
2. **A synchronous frontend crate between the store and the HTTP shell**, following Turso. Chosen.

How the shell reaches the core:
- a. **One task owns the core.** Handlers send it requests over a channel. That gives one serialisation point and a request log, but at the cost of an extra hop and a task to manage.
- b. **The core behind a `Mutex`**, called inline from handlers, as Turso does. Chosen. The same ordering and request log come from the frontend writing the log while it holds the lock.

Transport:
- i. **Plain HTTP.** Chosen for M1.
- ii. **HTTPS with a CA that kubemu creates.** Needed eventually (webhooks, clients that refuse plain HTTP), but nothing in M1 needs it.

## Decision

**Option 2, option b, option i.**

### Layers
| Layer | Crate | Kind (ADR 0001) |
|---|---|---|
| Engine | `kubemu-store` | core |
| Frontend | `kubemu-api` (**new**) | core |
| Shell | `kubemu-apiserver` | shell |

**`kubemu-api`: the synchronous frontend.** It doesn't know about HTTP. It takes a typed request (verb, group/version/resource, namespace, name, subresource, query options, body, `fieldValidation`, preconditions) and returns a typed response or a Kubernetes `Status` error (reason, code, details). It owns the request pipeline:
- decode;
- admission (built-in only in M1);
- defaulting and validation (ADR 0013);
- unknown-field pruning (ADR 0010);
- patch (`kubemu-patch`);
- the store;
- encode.

It also owns discovery and OpenAPI responses (ADR 0012) and watch cursors (ADR 0002). It takes time and randomness only through `Env`: creation timestamps, and UIDs as UUIDv4 drawn from `Env::rng`. It records each request it handles in the event trace.

**The DST harness drives `kubemu-api`**, not the store directly. Simulated clients go through the real request path, so L0 covers API semantics, not just storage.

**`kubemu-apiserver`: the thin tokio shell.**
- hyper and axum, HTTP/1.1. The shell parses the URL, query and headers into a frontend request, calls the frontend, and writes the response.
- **The core sits behind a `Mutex`** and is called inline from handlers. Locks are held only for a single frontend call, never across an `.await`.
- **Watches:** the handler keeps a cursor and waits on a tokio `Notify`. When woken, it takes the lock briefly to pull new events, releases it, and streams them as chunked JSON. Bookmarks and `410 Gone` behave as in ADR 0002.
- Any decision worth testing belongs in `kubemu-api`, not here (ADR 0001).

### What M1 serves
- **Plain HTTP.** `kubemu kubeconfig` writes `server: http://127.0.0.1:<port>`. kubectl, Helm and client-go accept that. TLS is added when something needs it.
- **Authentication and authorization: none.** Any credential, or none, is treated as cluster-admin. `SelfSubjectAccessReview` and `SelfSubjectRulesReview` answer "allowed". This is logged at startup and noted in the compatibility report (principle 1.6). Real RBAC is T3.
- **One cluster per listener.**

### Multi-cluster: deferred
Serving several clusters is a future concern, not designed here. The store is already multi-tenant (`Store` holds named `ClusterStore`s), so adding it later doesn't need a redesign. Options to consider then: a path prefix per cluster on one listener (`/clusters/<name>`, which client-go accepts in the server URL), or a listener per cluster. Until then, several clusters means several listeners, or several kubemu processes.

## Consequences

- New crate `kubemu-api` (core). The no-tokio check (ADR 0001) covers it.
- Tokio, hyper and axum arrive in `kubemu-apiserver` and `kubemu-cli` only. The same commit adds the tokio bans to `clippy.toml` (`tokio::net::*::bind`/`connect`, `tokio::fs::*`, `tokio::process::*`) and the cargo-deny check that keeps tokio out of the core crates, as ADR 0001 promised. Binding the listener goes through `kubemu-env-real`.
- A single `Mutex` serialises all API requests. That matches the sim's one-step-at-a-time model. If benchmarks show contention, the fix is a shorter critical section (encode outside the lock), not a different model.
- Shell tests follow Turso: they drive real HTTP against an in-memory core. Everything else is tested at the `kubemu-api` level, in unit tests and DST.
- No TLS means no TLS dependencies yet. The first feature that needs TLS (most likely admission webhooks, M3) brings its own ADR or amends this one.
- exec, attach, logs and port-forward (M2) need protocol upgrades (WebSocket or SPDY) on HTTP/1.1. They are added to the shell at M2.
