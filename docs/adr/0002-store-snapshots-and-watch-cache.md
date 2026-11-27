# 0002: Store data structure and watch cache

- Status: Proposed
- Date: 2026-11-27

## Context

See README §4.3, §2.2. Forking a cluster must be O(1), so the store needs a persistent (copy-on-write) map. The scaffold uses BTreeMap plus a bounded event log as a placeholder.

Forces:
- Deterministic, ordered iteration (AGENTS.md: `BTreeMap` where order leaks). List results come back in key order.
- Budgets (README §4.4): fork < 50 ms, idle cluster < 20 MB RSS.
- Watch semantics: resume from a resourceVersion, `410 Gone` after compaction, bookmarks, fan-out to many watchers (informers in every controller and operator).
- Lists filter by resource, namespace, labels and fields. A label or field index is just another ordered map.

## Options

Persistent map for objects:
1. **`imbl::OrdMap`** (maintained fork of `im`). A persistent B-tree: O(1) clone, ordered iteration, range queries, `Send + Sync` via `Arc`.
2. **`rpds::RedBlackTreeMap`**. Persistent and ordered, but a binary tree: more allocations and worse cache behaviour than a B-tree. Needs the `ArcK` pointer kind for `Send`.
3. **Custom persistent B-tree.** Full control, but a lot of code to write and fuzz for no clear gain at M0.
4. **`BTreeMap` + clone on fork.** Simple, but a fork is O(n). It breaks the budget once a cluster holds thousands of objects.

Watch cache:
- a. **A single per-cluster event log** (today's design), with watchers filtering on read.
- b. **Per-resource logs**, like the apiserver's cacher: separate history per resource, so a busy resource can't compact away a quiet resource's history.

## Decision

Proposed: **`imbl::OrdMap` (option 1) with a per-cluster log (option a) for M0/M1.** Revisit (b) if the informer harness shows too many 410s.

- `ClusterStore` holds `OrdMap<ObjectKey, Object<V>>`. Values are `Arc`-shared, so a fork copies a root pointer and diverges lazily.
- The event log becomes a persistent vector (`imbl::Vector`), so a fork keeps its history and watchers on the fork can resume.
- Compaction is bounded by event count, configurable per cluster. Count, not age, keeps it deterministic with no clock dependency. Age-based compaction can come later through `Env::clock`.
- Bookmarks are synthesised at the current resourceVersion when a watcher asks for them. They don't go into the log.
- Fan-out: a watcher is a cursor plus a wakeup registered with `Env` tasks (ADR 0001). Writers append and wake, and each watcher pulls. Slow watchers can't block writers, and if they fall behind compaction they get 410.
- Secondary indexes (namespace, labels, owner UID) are also `OrdMap`s inside `ClusterStore`, so they fork for free.

## Consequences

- New dependency: `imbl` (in kubemu-store only).
- `Store::fork(cluster, new_name)` lands in M0 as an O(1) operation with a property test: a mutation on the fork is never visible in the parent, and the other way round.
- `OrdMap` lookups are slower than `BTreeMap` by a constant factor. M1 benchmarks will show whether that matters.
- Iteration order is still key order, so traces and list output are unchanged.
