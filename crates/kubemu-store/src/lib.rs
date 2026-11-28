//! Versioned in-memory object store (README §2.2, ADR 0002).
//!
//! One [`Store`] holds every emulated cluster as a tenant; each tenant is a
//! [`ClusterStore`] with its own resourceVersion sequence, like one etcd per
//! cluster. Watches are pull-based cursors over a bounded event log; asking
//! for history that has been compacted away yields [`StoreError::Gone`]
//! (HTTP 410).
//!
//! Objects and the log are persistent `imbl` structures, so
//! [`Store::fork`] is O(1): parent and fork share structure and diverge
//! lazily. A fork keeps the parent's history, so watchers can resume on it.
//!
//! Not yet: label/field indexes, finalizers, ownerReferences, bookmarks
//! (they need object metadata, M1).

use std::collections::BTreeMap;
use std::fmt;

use imbl::{OrdMap, Vector};
pub type ResourceVersion = u64;

/// Identifies an object inside one cluster. `namespace` is empty for
/// cluster-scoped resources. Ordering is deterministic (lexicographic).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectKey {
    pub resource: String,
    pub namespace: String,
    pub name: String,
}

impl ObjectKey {
    pub fn new(
        resource: impl Into<String>,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            resource: resource.into(),
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

impl fmt::Display for ObjectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.namespace.is_empty() {
            write!(f, "{}/{}", self.resource, self.name)
        } else {
            write!(f, "{}/{}/{}", self.resource, self.namespace, self.name)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Object<V> {
    pub resource_version: ResourceVersion,
    pub value: V,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    Added,
    Modified,
    Deleted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event<V> {
    pub kind: EventKind,
    pub key: ObjectKey,
    pub resource_version: ResourceVersion,
    /// For `Deleted`, the last state of the object.
    pub value: V,
    /// For `Modified`, the state before the write. Watch filters need it to
    /// see an object leave a selector, and point-in-time lists to roll back.
    pub prev: Option<V>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoreError {
    AlreadyExists(ObjectKey),
    NotFound(ObjectKey),
    Conflict {
        key: ObjectKey,
        expected: ResourceVersion,
        actual: ResourceVersion,
    },
    /// The requested resourceVersion is older than the retained history.
    Gone {
        requested: ResourceVersion,
        oldest: ResourceVersion,
    },
    UnknownCluster(String),
    ClusterExists(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists(k) => write!(f, "{k} already exists"),
            Self::NotFound(k) => write!(f, "{k} not found"),
            Self::Conflict {
                key,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "conflict on {key}: expected rv {expected}, have {actual}"
                )
            }
            Self::Gone { requested, oldest } => {
                write!(
                    f,
                    "resourceVersion {requested} too old (oldest retained {oldest})"
                )
            }
            Self::UnknownCluster(c) => write!(f, "unknown cluster {c:?}"),
            Self::ClusterExists(c) => write!(f, "cluster {c:?} already exists"),
        }
    }
}

impl std::error::Error for StoreError {}

pub type Result<T> = std::result::Result<T, StoreError>;

/// One cluster's objects plus its event history. `Clone` is O(1).
#[derive(Clone, Debug)]
pub struct ClusterStore<V: Clone> {
    objects: OrdMap<ObjectKey, Object<V>>,
    rv: ResourceVersion,
    /// Every write appends exactly one event, so retained resourceVersions
    /// are contiguous: `log[i].resource_version == log[0].resource_version + i`.
    log: Vector<Event<V>>,
    log_capacity: usize,
}

impl<V: Clone> ClusterStore<V> {
    pub fn new(log_capacity: usize) -> Self {
        Self {
            objects: OrdMap::new(),
            rv: 0,
            log: Vector::new(),
            log_capacity: log_capacity.max(1),
        }
    }

    pub fn resource_version(&self) -> ResourceVersion {
        self.rv
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn get(&self, key: &ObjectKey) -> Option<&Object<V>> {
        self.objects.get(key)
    }

    /// Lists a resource, optionally within one namespace, in key order,
    /// together with the store resourceVersion the list is consistent at.
    /// Keys sort by (resource, namespace, name), so this is a range scan.
    pub fn list(
        &self,
        resource: &str,
        namespace: Option<&str>,
    ) -> (Vec<(&ObjectKey, &Object<V>)>, ResourceVersion) {
        let from = ObjectKey::new(resource, namespace.unwrap_or(""), "");
        let items = self
            .objects
            .range(from..)
            .take_while(|(k, _)| {
                k.resource == resource && namespace.is_none_or(|ns| k.namespace == ns)
            })
            .collect();
        (items, self.rv)
    }

    pub fn create(&mut self, key: ObjectKey, value: V) -> Result<ResourceVersion> {
        if self.objects.contains_key(&key) {
            return Err(StoreError::AlreadyExists(key));
        }
        let rv = self.bump();
        self.objects.insert(
            key.clone(),
            Object {
                resource_version: rv,
                value: value.clone(),
            },
        );
        self.record(EventKind::Added, key, rv, value, None);
        Ok(rv)
    }

    /// Replaces an object. With `expected` set, fails on a resourceVersion
    /// mismatch (optimistic concurrency, as with `metadata.resourceVersion`).
    pub fn update(
        &mut self,
        key: &ObjectKey,
        value: V,
        expected: Option<ResourceVersion>,
    ) -> Result<ResourceVersion> {
        self.check(key, expected)?;
        let rv = self.bump();
        let prev = self
            .objects
            .insert(
                key.clone(),
                Object {
                    resource_version: rv,
                    value: value.clone(),
                },
            )
            .map(|o| o.value);
        self.record(EventKind::Modified, key.clone(), rv, value, prev);
        Ok(rv)
    }

    pub fn delete(
        &mut self,
        key: &ObjectKey,
        expected: Option<ResourceVersion>,
    ) -> Result<Object<V>> {
        self.check(key, expected)?;
        let old = self.objects.remove(key).expect("checked above");
        let rv = self.bump();
        self.record(EventKind::Deleted, key.clone(), rv, old.value.clone(), None);
        Ok(old)
    }

    /// Every event with resourceVersion strictly greater than `since`.
    /// `since == current rv` is valid and returns nothing.
    pub fn events_since(&self, since: ResourceVersion) -> Result<Vec<Event<V>>> {
        // The oldest rv we can resume from is the one just before the first
        // retained event; with an empty log, only the current rv.
        let oldest = self.log.front().map_or(self.rv, |e| e.resource_version - 1);
        if since < oldest {
            return Err(StoreError::Gone {
                requested: since,
                oldest,
            });
        }
        let skip = usize::try_from(since - oldest).expect("log index fits usize");
        Ok(self.log.iter().skip(skip).cloned().collect())
    }

    /// The oldest resourceVersion a watch can still resume from.
    pub fn oldest_resumable(&self) -> ResourceVersion {
        self.log.front().map_or(self.rv, |e| e.resource_version - 1)
    }

    fn check(&self, key: &ObjectKey, expected: Option<ResourceVersion>) -> Result<ResourceVersion> {
        let obj = self
            .objects
            .get(key)
            .ok_or_else(|| StoreError::NotFound(key.clone()))?;
        match expected {
            Some(e) if e != obj.resource_version => Err(StoreError::Conflict {
                key: key.clone(),
                expected: e,
                actual: obj.resource_version,
            }),
            _ => Ok(obj.resource_version),
        }
    }

    fn bump(&mut self) -> ResourceVersion {
        self.rv += 1;
        self.rv
    }

    fn record(
        &mut self,
        kind: EventKind,
        key: ObjectKey,
        resource_version: ResourceVersion,
        value: V,
        prev: Option<V>,
    ) {
        if self.log.len() == self.log_capacity {
            self.log.pop_front();
        }
        self.log.push_back(Event {
            kind,
            key,
            resource_version,
            value,
            prev,
        });
    }
}

/// All clusters, keyed by name (deterministic iteration order).
#[derive(Clone, Debug, Default)]
pub struct Store<V: Clone> {
    clusters: BTreeMap<String, ClusterStore<V>>,
}

impl<V: Clone> Store<V> {
    pub const DEFAULT_LOG_CAPACITY: usize = 10_000;

    pub fn new() -> Self {
        Self {
            clusters: BTreeMap::new(),
        }
    }

    /// Adds a cluster if missing and returns it.
    pub fn add_cluster(&mut self, name: impl Into<String>) -> &mut ClusterStore<V> {
        self.clusters
            .entry(name.into())
            .or_insert_with(|| ClusterStore::new(Self::DEFAULT_LOG_CAPACITY))
    }

    /// Copies cluster `from` into a new cluster `to` in O(1). Objects, the
    /// resourceVersion and the retained event history are shared; writes to
    /// either side are never visible on the other.
    pub fn fork(&mut self, from: &str, to: impl Into<String>) -> Result<&mut ClusterStore<V>> {
        let to = to.into();
        if self.clusters.contains_key(&to) {
            return Err(StoreError::ClusterExists(to));
        }
        let forked = self.cluster(from)?.clone();
        Ok(self.clusters.entry(to).or_insert(forked))
    }

    pub fn remove_cluster(&mut self, name: &str) -> Option<ClusterStore<V>> {
        self.clusters.remove(name)
    }

    pub fn cluster(&self, name: &str) -> Result<&ClusterStore<V>> {
        self.clusters
            .get(name)
            .ok_or_else(|| StoreError::UnknownCluster(name.into()))
    }

    pub fn cluster_mut(&mut self, name: &str) -> Result<&mut ClusterStore<V>> {
        self.clusters
            .get_mut(name)
            .ok_or_else(|| StoreError::UnknownCluster(name.into()))
    }

    pub fn cluster_names(&self) -> impl Iterator<Item = &str> {
        self.clusters.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cm(name: &str) -> ObjectKey {
        ObjectKey::new("configmaps", "default", name)
    }

    #[test]
    fn crud_and_resource_versions() {
        let mut s = ClusterStore::new(100);
        let rv1 = s.create(cm("a"), 1).unwrap();
        assert_eq!(rv1, 1);
        assert_eq!(
            s.create(cm("a"), 2),
            Err(StoreError::AlreadyExists(cm("a")))
        );
        let rv2 = s.update(&cm("a"), 2, Some(rv1)).unwrap();
        assert!(rv2 > rv1);
        assert!(matches!(
            s.update(&cm("a"), 3, Some(rv1)),
            Err(StoreError::Conflict { .. })
        ));
        assert_eq!(s.delete(&cm("a"), None).unwrap().value, 2);
        assert!(s.get(&cm("a")).is_none());
        assert_eq!(s.resource_version(), 3);
    }

    #[test]
    fn list_filters_by_namespace() {
        let mut s = ClusterStore::new(100);
        s.create(ObjectKey::new("pods", "a", "p1"), ()).unwrap();
        s.create(ObjectKey::new("pods", "b", "p2"), ()).unwrap();
        s.create(ObjectKey::new("nodes", "", "n1"), ()).unwrap();
        assert_eq!(s.list("pods", None).0.len(), 2);
        assert_eq!(s.list("pods", Some("a")).0.len(), 1);
        assert_eq!(s.list("nodes", None).1, 3);
    }

    #[test]
    fn watch_resume_and_compaction() {
        let mut s = ClusterStore::new(3);
        for i in 0..5 {
            s.create(cm(&i.to_string()), i).unwrap();
        }
        // Retained rvs 3, 4, 5: resuming from 2 works, from 1 is Gone.
        assert_eq!(s.events_since(2).unwrap().len(), 3);
        assert_eq!(s.events_since(5).unwrap().len(), 0);
        assert_eq!(
            s.events_since(1),
            Err(StoreError::Gone {
                requested: 1,
                oldest: 2
            })
        );
    }

    #[test]
    fn clusters_are_isolated() {
        let mut st: Store<u8> = Store::new();
        st.add_cluster("east").create(cm("x"), 1).unwrap();
        st.add_cluster("west");
        assert!(st.cluster("west").unwrap().is_empty());
        assert_eq!(st.cluster("west").unwrap().resource_version(), 0);
        assert!(matches!(
            st.cluster("nope"),
            Err(StoreError::UnknownCluster(_))
        ));
    }

    #[test]
    fn fork_shares_history_and_diverges() {
        let mut st: Store<u8> = Store::new();
        let a = st.add_cluster("a");
        a.create(cm("x"), 1).unwrap();
        a.create(cm("y"), 2).unwrap();
        let f = st.fork("a", "f").unwrap();
        // History is shared: a watcher from before the fork resumes on it.
        assert_eq!(f.events_since(0).unwrap().len(), 2);
        f.update(&cm("x"), 9, None).unwrap();
        f.create(cm("z"), 3).unwrap();
        st.cluster_mut("a").unwrap().delete(&cm("y"), None).unwrap();

        let a = st.cluster("a").unwrap();
        let f = st.cluster("f").unwrap();
        assert_eq!(a.get(&cm("x")).unwrap().value, 1);
        assert!(a.get(&cm("y")).is_none() && a.get(&cm("z")).is_none());
        assert_eq!(f.get(&cm("x")).unwrap().value, 9);
        assert!(f.get(&cm("y")).is_some());
        // Same rv sequence from the fork point on, different events.
        assert_eq!(a.resource_version(), 3);
        assert_eq!(f.resource_version(), 4);
        assert_eq!(a.events_since(2).unwrap()[0].kind, EventKind::Deleted);
        assert_eq!(f.events_since(2).unwrap()[0].kind, EventKind::Modified);

        assert_eq!(
            st.fork("a", "f").err(),
            Some(StoreError::ClusterExists("f".into()))
        );
        assert!(matches!(
            st.fork("nope", "g"),
            Err(StoreError::UnknownCluster(_))
        ));
    }

    mod prop {
        use super::*;
        use proptest::prelude::*;
        use proptest::test_runner::RngSeed;

        #[derive(Clone, Debug)]
        enum Op {
            Create(u8, u8),
            Update(u8, u8),
            Delete(u8),
        }

        fn op() -> impl Strategy<Value = Op> {
            prop_oneof![
                (0..8u8, any::<u8>()).prop_map(|(k, v)| Op::Create(k, v)),
                (0..8u8, any::<u8>()).prop_map(|(k, v)| Op::Update(k, v)),
                (0..8u8).prop_map(Op::Delete),
            ]
        }

        type Model = BTreeMap<ObjectKey, u8>;

        fn apply(s: &mut ClusterStore<u8>, m: &mut Model, op: &Op) {
            let key = |k: &u8| cm(&k.to_string());
            match op {
                Op::Create(k, v) => {
                    if s.create(key(k), *v).is_ok() {
                        m.insert(key(k), *v);
                    }
                }
                Op::Update(k, v) => {
                    if s.update(&key(k), *v, None).is_ok() {
                        m.insert(key(k), *v);
                    }
                }
                Op::Delete(k) => {
                    if s.delete(&key(k), None).is_ok() {
                        m.remove(&key(k));
                    }
                }
            }
        }

        fn contents(s: &ClusterStore<u8>) -> Model {
            s.list("configmaps", None)
                .0
                .into_iter()
                .map(|(k, o)| (k.clone(), o.value))
                .collect()
        }

        proptest! {
            // Fixed seed: the property run is reproducible like everything else.
            #![proptest_config(ProptestConfig {
                cases: 512,
                rng_seed: RngSeed::Fixed(0x6b75_6265_6d75),
                failure_persistence: None,
                ..ProptestConfig::default()
            })]

            /// ADR 0002: a mutation on a fork is never visible in the parent,
            /// and the other way round. Both sides behave like independent
            /// copies taken at the fork point.
            #[test]
            fn fork_is_isolated(
                before in proptest::collection::vec(op(), 0..40),
                ops in proptest::collection::vec((any::<bool>(), op()), 0..80),
                cap in 1usize..20,
            ) {
                let mut st: Store<u8> = Store::new();
                *st.add_cluster("p") = ClusterStore::new(cap);
                let mut parent = Model::new();
                for o in &before {
                    apply(st.cluster_mut("p").unwrap(), &mut parent, o);
                }
                let fork_rv = st.cluster("p").unwrap().resource_version();
                st.fork("p", "f").unwrap();
                let at_fork = parent.clone();
                let mut fork = parent.clone();

                for (on_fork, o) in &ops {
                    if *on_fork {
                        apply(st.cluster_mut("f").unwrap(), &mut fork, o);
                    } else {
                        apply(st.cluster_mut("p").unwrap(), &mut parent, o);
                    }
                }

                for (name, model) in [("p", &parent), ("f", &fork)] {
                    let s = st.cluster(name).unwrap();
                    prop_assert_eq!(&contents(s), model);
                    // Replaying a side's own events on top of the fork point
                    // reproduces it, while that history is still retained.
                    if let Ok(events) = s.events_since(fork_rv) {
                        let mut m = at_fork.clone();
                        let mut rv = fork_rv;
                        for e in events {
                            prop_assert_eq!(e.resource_version, rv + 1);
                            rv = e.resource_version;
                            match e.kind {
                                EventKind::Deleted => { m.remove(&e.key); }
                                _ => { m.insert(e.key, e.value); }
                            }
                        }
                        prop_assert_eq!(&m, model);
                        prop_assert_eq!(rv, s.resource_version());
                    }
                }
            }
        }
    }
}
