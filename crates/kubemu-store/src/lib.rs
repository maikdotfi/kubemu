//! Versioned in-memory object store (README §2.2).
//!
//! One [`Store`] holds every emulated cluster as a tenant; each tenant is a
//! [`ClusterStore`] with its own resourceVersion sequence, like one etcd per
//! cluster. Watches read from a bounded event log; asking for history that
//! has been compacted away yields [`StoreError::Gone`] (HTTP 410).
//!
//! Not yet: field/label indexes, finalizers, ownerReferences, bookmarks and
//! O(1) snapshots via persistent maps (ADR 0002).

use std::collections::{BTreeMap, VecDeque};
use std::fmt;

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
        }
    }
}

impl std::error::Error for StoreError {}

pub type Result<T> = std::result::Result<T, StoreError>;

/// One cluster's objects plus its event history.
#[derive(Clone, Debug)]
pub struct ClusterStore<V> {
    objects: BTreeMap<ObjectKey, Object<V>>,
    rv: ResourceVersion,
    log: VecDeque<Event<V>>,
    log_capacity: usize,
}

impl<V: Clone> ClusterStore<V> {
    pub fn new(log_capacity: usize) -> Self {
        Self {
            objects: BTreeMap::new(),
            rv: 0,
            log: VecDeque::new(),
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
    pub fn list(
        &self,
        resource: &str,
        namespace: Option<&str>,
    ) -> (Vec<(&ObjectKey, &Object<V>)>, ResourceVersion) {
        let items = self
            .objects
            .iter()
            .filter(|(k, _)| k.resource == resource && namespace.is_none_or(|ns| k.namespace == ns))
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
        self.record(EventKind::Added, key, rv, value);
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
        self.objects.insert(
            key.clone(),
            Object {
                resource_version: rv,
                value: value.clone(),
            },
        );
        self.record(EventKind::Modified, key.clone(), rv, value);
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
        self.record(EventKind::Deleted, key.clone(), rv, old.value.clone());
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
        Ok(self
            .log
            .iter()
            .filter(|e| e.resource_version > since)
            .cloned()
            .collect())
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
    ) {
        if self.log.len() == self.log_capacity {
            self.log.pop_front();
        }
        self.log.push_back(Event {
            kind,
            key,
            resource_version,
            value,
        });
    }
}

/// All clusters, keyed by name (deterministic iteration order).
#[derive(Clone, Debug, Default)]
pub struct Store<V> {
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
}
