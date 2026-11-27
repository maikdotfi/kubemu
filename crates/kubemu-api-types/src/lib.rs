//! What kubemu knows about the resources it serves, from one source: the
//! upstream `api/openapi-spec` vendored under `openapi/` at the pinned
//! Kubernetes version (ADR 0007, ADR 0012). `build.rs` filters it to the
//! resources listed in `served.rs` and generates:
//! - [`RESOURCES`]: the table discovery is served from;
//! - [`OPENAPI_V2`] and [`OPENAPI_V3`]: the documents as served;
//! - [`Schema`]: every reachable definition as a navigable tree carrying the
//!   patch metadata (`x-kubernetes-patch-*`, list types) for strategic merge
//!   patch and the types for pruning and structural validation.
//!
//! Everything is static data: nothing is parsed at runtime.

mod generated {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

use generated::{DEFINITIONS, GVKS, NODES};
pub use generated::{KUBERNETES_VERSION, OPENAPI_V2, OPENAPI_V3, RESOURCES};

/// A served resource, as discovery reports it.
#[derive(Debug)]
pub struct ResourceInfo {
    /// Empty for the core group.
    pub group: &'static str,
    pub version: &'static str,
    pub kind: &'static str,
    pub plural: &'static str,
    pub singular: &'static str,
    pub namespaced: bool,
    pub short_names: &'static [&'static str],
    pub categories: &'static [&'static str],
    /// Sorted, as the apiserver lists them.
    pub verbs: &'static [&'static str],
    pub subresources: &'static [Subresource],
    pub schema: SchemaId,
}

#[derive(Debug)]
pub struct Subresource {
    pub name: &'static str,
    pub verbs: &'static [&'static str],
}

impl ResourceInfo {
    /// `v1` or `apps/v1`.
    pub fn api_version(&self) -> String {
        if self.group.is_empty() {
            self.version.to_owned()
        } else {
            format!("{}/{}", self.group, self.version)
        }
    }

    pub fn schema(&self) -> Schema {
        Schema(self.schema)
    }

    pub fn has_verb(&self, verb: &str) -> bool {
        self.verbs.contains(&verb)
    }

    pub fn subresource(&self, name: &str) -> Option<&'static Subresource> {
        self.subresources.iter().find(|s| s.name == name)
    }
}

/// Finds a served resource by group, version and plural name.
pub fn resource(group: &str, version: &str, plural: &str) -> Option<&'static ResourceInfo> {
    RESOURCES
        .iter()
        .find(|r| r.group == group && r.version == version && r.plural == plural)
}

/// Finds a served resource by group, version and kind.
pub fn resource_for_kind(group: &str, version: &str, kind: &str) -> Option<&'static ResourceInfo> {
    RESOURCES
        .iter()
        .find(|r| r.group == group && r.version == version && r.kind == kind)
}

/// One served OpenAPI v3 document (`/openapi/v3/<path>`).
#[derive(Debug)]
pub struct OpenApiV3 {
    /// `api/v1` or `apis/<group>/<version>`.
    pub path: &'static str,
    /// Changes whenever the document does; clients cache by it.
    pub hash: &'static str,
    pub document: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaId(pub u32);

/// A node of the schema arena.
#[derive(Debug)]
pub struct Node {
    pub ty: Ty,
    pub ext: Ext,
}

#[derive(Debug)]
pub enum Ty {
    /// A `$ref` to a definition. The referring node may add extensions.
    Ref(SchemaId),
    Object {
        fields: &'static [(&'static str, SchemaId)],
        required: &'static [&'static str],
    },
    /// `additionalProperties`: a map with string keys.
    Map(SchemaId),
    Array(SchemaId),
    String,
    Integer,
    Number,
    Boolean,
    /// A string or an integer.
    IntOrString,
    /// `resource.Quantity`: a string, but numbers decode too.
    Quantity,
    /// Anything, kept as is (`RawExtension`, `FieldsV1`).
    Any,
}

/// The OpenAPI extensions kubemu acts on.
#[derive(Debug, Default)]
pub struct Ext {
    /// `merge`, `retainKeys`, `merge,retainKeys` or `replace`.
    pub patch_strategy: Option<&'static str>,
    pub merge_key: Option<&'static str>,
    /// `atomic`, `set` or `map`.
    pub list_type: Option<&'static str>,
    pub map_keys: &'static [&'static str],
    /// `atomic` or `granular`.
    pub map_type: Option<&'static str>,
    pub format: Option<&'static str>,
    pub preserve_unknown_fields: bool,
}

/// A position in a schema: a definition, or a field, item or map value
/// reached from one. Extensions come from the position itself first, then
/// from the definition it refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Schema(SchemaId);

impl Schema {
    /// Follows `$ref`s from this position, innermost last.
    fn chain(self) -> impl Iterator<Item = &'static Node> {
        let mut next = Some(self.0);
        std::iter::from_fn(move || {
            let n = &NODES[next?.0 as usize];
            next = match n.ty {
                Ty::Ref(id) => Some(id),
                _ => None,
            };
            Some(n)
        })
    }

    /// A definition by its full name (`io.k8s.api.core.v1.Pod`).
    pub fn definition(name: &str) -> Option<Self> {
        DEFINITIONS
            .binary_search_by(|(n, _)| (*n).cmp(name))
            .ok()
            .map(|i| Self(DEFINITIONS[i].1))
    }

    /// The schema of a kind, served or not (`Status`, `DeleteOptions` and
    /// `WatchEvent` are here too, under every group).
    pub fn for_kind(group: &str, version: &str, kind: &str) -> Option<Self> {
        GVKS.iter()
            .find(|(g, v, k, _)| *g == group && *v == version && *k == kind)
            .map(|(.., id)| Self(*id))
    }

    pub fn id(self) -> SchemaId {
        self.0
    }

    /// The type, with `$ref`s resolved.
    pub fn ty(self) -> &'static Ty {
        &self.chain().last().expect("chain is never empty").ty
    }

    fn ext<T>(self, f: impl Fn(&'static Ext) -> Option<T>) -> Option<T> {
        self.chain().find_map(|n| f(&n.ext))
    }

    pub fn field(self, name: &str) -> Option<Self> {
        match self.ty() {
            Ty::Object { fields, .. } => fields
                .iter()
                .find(|(f, _)| *f == name)
                .map(|(_, id)| Self(*id)),
            _ => None,
        }
    }

    pub fn fields(self) -> impl Iterator<Item = (&'static str, Self)> {
        let fields: &'static [(&'static str, SchemaId)] = match self.ty() {
            Ty::Object { fields, .. } => fields,
            _ => &[],
        };
        fields.iter().map(|(n, id)| (*n, Self(*id)))
    }

    pub fn required(self) -> &'static [&'static str] {
        match self.ty() {
            Ty::Object { required, .. } => required,
            _ => &[],
        }
    }

    /// The element schema of an array.
    pub fn items(self) -> Option<Self> {
        match self.ty() {
            Ty::Array(id) => Some(Self(*id)),
            _ => None,
        }
    }

    /// The value schema of a map.
    pub fn values(self) -> Option<Self> {
        match self.ty() {
            Ty::Map(id) => Some(Self(*id)),
            _ => None,
        }
    }

    pub fn patch_strategy(self) -> Option<&'static str> {
        self.ext(|e| e.patch_strategy)
    }

    /// Whether the patch strategy includes `s` (strategies combine, as in
    /// `merge,retainKeys`).
    pub fn has_patch_strategy(self, s: &str) -> bool {
        self.patch_strategy()
            .is_some_and(|ps| ps.split(',').any(|p| p == s))
    }

    pub fn merge_key(self) -> Option<&'static str> {
        self.ext(|e| e.merge_key)
    }

    pub fn list_type(self) -> Option<&'static str> {
        self.ext(|e| e.list_type)
    }

    pub fn map_keys(self) -> &'static [&'static str] {
        self.ext(|e| (!e.map_keys.is_empty()).then_some(e.map_keys))
            .unwrap_or(&[])
    }

    pub fn map_type(self) -> Option<&'static str> {
        self.ext(|e| e.map_type)
    }

    pub fn format(self) -> Option<&'static str> {
        self.ext(|e| e.format)
    }

    pub fn preserve_unknown_fields(self) -> bool {
        self.chain().any(|n| n.ext.preserve_unknown_fields) || matches!(self.ty(), Ty::Any)
    }

    /// Follows a path of field names (`["spec", "containers"]`); array
    /// items and map values are stepped through with `"*"`.
    pub fn at(self, path: &[&str]) -> Option<Self> {
        path.iter().try_fold(self, |s, seg| match *seg {
            "*" => s.items().or_else(|| s.values()),
            name => s.field(name),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_table() {
        let pods = resource("", "v1", "pods").unwrap();
        assert_eq!(pods.kind, "Pod");
        assert!(pods.namespaced);
        assert_eq!(pods.short_names, ["po"]);
        assert_eq!(
            pods.verbs,
            [
                "create",
                "delete",
                "deletecollection",
                "get",
                "list",
                "patch",
                "update",
                "watch"
            ]
        );
        assert_eq!(
            pods.subresource("status").unwrap().verbs,
            ["get", "patch", "update"]
        );
        assert!(pods.subresource("exec").is_none());

        let ns = resource("", "v1", "namespaces").unwrap();
        assert!(!ns.namespaced);
        assert!(!ns.has_verb("deletecollection"));
        assert!(!resource("", "v1", "nodes").unwrap().namespaced);

        let eps = resource_for_kind("discovery.k8s.io", "v1", "EndpointSlice").unwrap();
        assert_eq!(eps.api_version(), "discovery.k8s.io/v1");
        assert!(resource("apps", "v1", "deployments").is_none());
    }

    #[test]
    fn every_resource_has_its_schema() {
        for r in RESOURCES {
            let s = r.schema();
            assert!(s.field("metadata").is_some(), "{}", r.kind);
            assert_eq!(Schema::for_kind(r.group, r.version, r.kind), Some(s));
        }
        assert!(Schema::for_kind("", "v1", "Status").is_some());
    }

    #[test]
    fn patch_metadata() {
        let pod = resource("", "v1", "pods").unwrap().schema();
        let containers = pod.at(&["spec", "containers"]).unwrap();
        assert_eq!(containers.patch_strategy(), Some("merge"));
        assert_eq!(containers.merge_key(), Some("name"));
        assert_eq!(containers.list_type(), Some("map"));
        assert_eq!(containers.map_keys(), ["name"]);
        let volumes = pod.at(&["spec", "volumes"]).unwrap();
        assert!(volumes.has_patch_strategy("merge") && volumes.has_patch_strategy("retainKeys"));
        let port = pod.at(&["spec", "containers", "*", "ports", "*", "containerPort"]);
        assert!(matches!(port.unwrap().ty(), Ty::Integer));
        let labels = pod.at(&["metadata", "labels"]).unwrap();
        assert!(matches!(labels.values().unwrap().ty(), Ty::String));
        let owners = pod.at(&["metadata", "ownerReferences"]).unwrap();
        assert_eq!(owners.merge_key(), Some("uid"));
        assert!(matches!(
            pod.at(&["spec", "containers", "*", "resources", "limits", "*"])
                .unwrap()
                .ty(),
            Ty::Quantity
        ));
        assert!(
            pod.at(&["metadata", "managedFields", "*", "fieldsV1"])
                .unwrap()
                .preserve_unknown_fields()
        );
        assert!(pod.field("nope").is_none());
    }

    #[test]
    fn openapi_documents_are_filtered() {
        assert!(KUBERNETES_VERSION.starts_with("v1."));
        let v2 = |s: &str| OPENAPI_V2.contains(s);
        assert!(v2("\"/api/v1/namespaces/{namespace}/pods/{name}/status\""));
        assert!(!v2("\"/api/v1/namespaces/{namespace}/pods/{name}/exec\""));
        assert!(!v2("/apis/apps/v1"));
        assert!(!v2("io.k8s.api.apps.v1.Deployment\""));
        let paths: Vec<&str> = OPENAPI_V3.iter().map(|d| d.path).collect();
        assert_eq!(paths, ["api/v1", "apis/discovery.k8s.io/v1"]);
        assert!(OPENAPI_V3.iter().all(|d| d.hash.len() == 16));
    }
}
