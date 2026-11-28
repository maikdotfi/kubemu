//! Watch cursors (ADR 0002): pull-based. The shell (or the sim) keeps a
//! [`WatchCursor`] and calls [`crate::Api::watch_poll`] when the store may
//! have changed; each poll returns the watch events to send.
//!
//! Semantics follow the apiserver:
//! - start at `resourceVersion` N: events after N, or an `ERROR` event with
//!   a 410 `Status` once N is compacted away;
//! - unset or `"0"`: synthetic `ADDED` events for the current state first;
//! - `sendInitialEvents=true` (watch-list): the same, then a `BOOKMARK`
//!   annotated `k8s.io/initial-events-end`;
//! - with selectors, an object entering the selection is `ADDED` and one
//!   leaving it `DELETED`, as clients' caches expect;
//! - `allowWatchBookmarks`: a `BOOKMARK` about once a minute;
//! - the watch ends at `timeoutSeconds`, or a random 30–60 minutes.

use std::collections::VecDeque;
use std::time::Duration;

use kubemu_env::MonoTime;
use kubemu_store::{ClusterStore, EventKind, ResourceVersion, StoreError};
use serde_json::{Value, json};

use crate::object::{Obj, with_resource_version};
use crate::read::Filter;

const BOOKMARK_EVERY: Duration = Duration::from_secs(60);

#[derive(Debug)]
pub struct WatchCursor {
    pub(crate) cluster: String,
    pub(crate) resource: String,
    pub(crate) namespace: Option<String>,
    pub(crate) filter: Filter,
    /// For bookmark objects.
    pub(crate) api_version: String,
    pub(crate) kind: String,
    /// The last resourceVersion the stream has covered.
    pub(crate) rv: ResourceVersion,
    pub(crate) pending: VecDeque<Value>,
    pub(crate) bookmarks: bool,
    pub(crate) last_bookmark: MonoTime,
    pub(crate) deadline: MonoTime,
    pub(crate) done: bool,
}

/// One poll's worth of a watch stream.
#[derive(Debug, Default)]
pub struct WatchBatch {
    /// Watch events (`{"type": ..., "object": ...}`), in order.
    pub events: Vec<Value>,
    /// The stream is over; close the connection after sending `events`.
    pub closed: bool,
}

impl WatchCursor {
    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    /// The last resourceVersion delivered (or covered by a bookmark).
    pub fn resource_version(&self) -> ResourceVersion {
        self.rv
    }

    pub fn deadline(&self) -> MonoTime {
        self.deadline
    }

    pub(crate) fn event(kind: &str, object: Value) -> Value {
        json!({"type": kind, "object": object})
    }

    pub(crate) fn bookmark(&self, initial_end: bool) -> Value {
        let mut meta = json!({"resourceVersion": self.rv.to_string()});
        if initial_end {
            meta["annotations"] = json!({"k8s.io/initial-events-end": "true"});
        }
        Self::event(
            "BOOKMARK",
            json!({"kind": self.kind, "apiVersion": self.api_version, "metadata": meta}),
        )
    }

    fn in_scope(&self, v: &Value) -> bool {
        self.filter.matches(v)
    }

    pub(crate) fn poll(&mut self, cs: &ClusterStore<Obj>, now: MonoTime) -> WatchBatch {
        let mut out = WatchBatch::default();
        if self.done {
            out.closed = true;
            return out;
        }
        out.events.extend(self.pending.drain(..));
        match cs.events_since(self.rv) {
            Ok(events) => {
                for e in events {
                    self.rv = e.resource_version;
                    if e.key.resource != self.resource
                        || self
                            .namespace
                            .as_ref()
                            .is_some_and(|ns| &e.key.namespace != ns)
                    {
                        continue;
                    }
                    let ev = match e.kind {
                        EventKind::Added => self
                            .in_scope(&e.value)
                            .then(|| ("ADDED", (*e.value).clone())),
                        EventKind::Deleted => self.in_scope(&e.value).then(|| {
                            (
                                "DELETED",
                                with_resource_version(&e.value, e.resource_version),
                            )
                        }),
                        EventKind::Modified => {
                            let was = e.prev.as_ref().is_some_and(|p| self.in_scope(p));
                            match (was, self.in_scope(&e.value)) {
                                (true, true) => Some(("MODIFIED", (*e.value).clone())),
                                (false, true) => Some(("ADDED", (*e.value).clone())),
                                // Leaving the selection: the new state, as DELETED.
                                (true, false) => Some(("DELETED", (*e.value).clone())),
                                (false, false) => None,
                            }
                        }
                    };
                    if let Some((t, obj)) = ev {
                        out.events.push(Self::event(t, obj));
                    }
                }
            }
            Err(StoreError::Gone { requested, oldest }) => {
                out.events.push(Self::event(
                    "ERROR",
                    crate::status::ApiError::gone(format!(
                        "too old resource version: {requested} ({oldest})"
                    ))
                    .to_status(),
                ));
                self.done = true;
                out.closed = true;
                return out;
            }
            Err(other) => {
                out.events.push(Self::event(
                    "ERROR",
                    crate::status::ApiError::internal(other.to_string()).to_status(),
                ));
                self.done = true;
                out.closed = true;
                return out;
            }
        }
        if self.bookmarks && now.saturating_since(self.last_bookmark) >= BOOKMARK_EVERY {
            self.last_bookmark = now;
            out.events.push(self.bookmark(false));
        }
        if now >= self.deadline {
            self.done = true;
            out.closed = true;
        }
        out
    }
}
