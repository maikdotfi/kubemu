//! The resources kubemu serves (README §5). Read only by `build.rs`, which
//! joins this table with the vendored upstream spec (ADR 0012): verbs,
//! schemas and OpenAPI paths come from the spec, never from here. What is
//! here is what the spec does not say: which resources and subresources are
//! served, short names and categories (as a real 1.37 apiserver reports
//! them).
//!
//! To serve a group-version not yet vendored, add its v3 file to
//! `scripts/fetch-openapi.sh` first.

pub struct Served {
    pub group: &'static str,
    pub version: &'static str,
    pub kind: &'static str,
    pub plural: &'static str,
    pub short_names: &'static [&'static str],
    pub categories: &'static [&'static str],
    pub subresources: &'static [&'static str],
}

const fn core(
    kind: &'static str,
    plural: &'static str,
    short_names: &'static [&'static str],
    categories: &'static [&'static str],
    subresources: &'static [&'static str],
) -> Served {
    Served {
        group: "",
        version: "v1",
        kind,
        plural,
        short_names,
        categories,
        subresources,
    }
}

/// T0 (M1).
pub const SERVED: &[Served] = &[
    core("ConfigMap", "configmaps", &["cm"], &[], &[]),
    core("Endpoints", "endpoints", &["ep"], &[], &[]),
    core("Event", "events", &["ev"], &[], &[]),
    core(
        "Namespace",
        "namespaces",
        &["ns"],
        &[],
        &["finalize", "status"],
    ),
    core("Node", "nodes", &["no"], &[], &["status"]),
    core("Pod", "pods", &["po"], &["all"], &["binding", "status"]),
    core("Secret", "secrets", &[], &[], &[]),
    core("ServiceAccount", "serviceaccounts", &["sa"], &[], &[]),
    core("Service", "services", &["svc"], &["all"], &["status"]),
    // Virtual: answered, never stored. No RBAC in M1, so always "allowed"
    // (ADR 0011).
    Served {
        group: "authorization.k8s.io",
        version: "v1",
        kind: "SelfSubjectAccessReview",
        plural: "selfsubjectaccessreviews",
        short_names: &[],
        categories: &[],
        subresources: &[],
    },
    Served {
        group: "authorization.k8s.io",
        version: "v1",
        kind: "SelfSubjectRulesReview",
        plural: "selfsubjectrulesreviews",
        short_names: &[],
        categories: &[],
        subresources: &[],
    },
    Served {
        group: "discovery.k8s.io",
        version: "v1",
        kind: "EndpointSlice",
        plural: "endpointslices",
        short_names: &[],
        categories: &[],
        subresources: &[],
    },
];
