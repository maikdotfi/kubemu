//! Defaulting: the scheme defaulters the apiserver runs on every decoded
//! object (`k8s.io/kubernetes/pkg/apis/*/v1/defaults.go` and the generated
//! `SetObjectDefaults_*`), at the 1.37 feature-gate defaults.
//!
//! Objects are JSON as decoded (quantities canonical). Go tests a string
//! with `== ""` and a pointer with `== nil`; here an absent or null field
//! is both, and an empty string counts as unset for strings. A default
//! that is a Go zero value of an omitempty field is not written.
//!
//! Not ported (feature-gated off in 1.37, or moved out of defaulting):
//! pod-level resource requests (`PodLevelResourcesFixDefaulting` defers
//! them to `PrepareForCreate`), H2C probe protocol (`H2CContainerProbe`).

use serde_json::{Map, Value, json};

use crate::quantity;

type Obj = Map<String, Value>;

fn is_unset_str(v: Option<&Value>) -> bool {
    v.is_none_or(|v| v.is_null() || v.as_str() == Some(""))
}

/// `if obj.F == "" { obj.F = v }`.
fn default_str(m: &mut Obj, key: &str, v: &str) {
    if is_unset_str(m.get(key)) {
        m.insert(key.into(), json!(v));
    }
}

/// `if obj.F == 0 { obj.F = v }`.
fn default_int(m: &mut Obj, key: &str, v: i64) {
    if m.get(key)
        .is_none_or(|x| x.is_null() || x.as_i64() == Some(0))
    {
        m.insert(key.into(), json!(v));
    }
}

/// `if obj.F == nil { obj.F = &v }`.
fn default_ptr(m: &mut Obj, key: &str, v: Value) {
    if m.get(key).is_none_or(Value::is_null) {
        m.insert(key.into(), v);
    }
}

fn obj<'a>(m: &'a mut Obj, key: &str) -> Option<&'a mut Obj> {
    m.get_mut(key).and_then(Value::as_object_mut)
}

fn each<'a>(m: &'a mut Obj, key: &str) -> impl Iterator<Item = &'a mut Obj> {
    m.get_mut(key)
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object_mut)
}

/// `SetDefaults_ResourceList`: quantities rounded up to milli units.
fn resource_list(m: &mut Obj, key: &str) {
    if let Some(list) = obj(m, key) {
        for v in list.values_mut() {
            if let Ok(s) = quantity::canonicalize_json(v, true) {
                *v = Value::String(s);
            }
        }
    }
}

fn resources(m: &mut Obj, key: &str) {
    if let Some(r) = obj(m, key) {
        resource_list(r, "limits");
        resource_list(r, "requests");
    }
}

/// Defaults `obj`, a decoded object of the given group and kind, in place.
pub fn default(group: &str, kind: &str, obj: &mut Value) {
    let Some(m) = obj.as_object_mut() else {
        return;
    };
    match (group, kind) {
        ("", "Pod") => pod(m),
        ("", "Service") => service(m),
        ("", "Namespace") => namespace(m),
        ("", "Secret") => default_str(m, "type", "Opaque"),
        ("", "Endpoints") => {
            for ss in each(m, "subsets") {
                for p in each(ss, "ports") {
                    default_str(p, "protocol", "TCP");
                }
            }
        }
        ("", "Node") => node(m),
        ("discovery.k8s.io", "EndpointSlice") => {
            for p in each(m, "ports") {
                default_ptr(p, "name", json!(""));
                default_ptr(p, "protocol", json!("TCP"));
            }
        }
        // ConfigMap's `data = {}` is invisible in JSON (omitempty).
        _ => {}
    }
}

fn namespace(m: &mut Obj) {
    let name = m
        .get("metadata")
        .and_then(|md| md.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if !name.is_empty() {
        let md = m.entry("metadata").or_insert_with(|| json!({}));
        if let Some(md) = md.as_object_mut() {
            let labels = md.entry("labels").or_insert_with(|| json!({}));
            if let Some(l) = labels.as_object_mut() {
                l.insert("kubernetes.io/metadata.name".into(), json!(name));
            }
        }
    }
    let status = m.entry("status").or_insert_with(|| json!({}));
    if let Some(s) = status.as_object_mut() {
        default_str(s, "phase", "Active");
    }
}

fn node(m: &mut Obj) {
    if let Some(s) = obj(m, "status") {
        if !s.contains_key("allocatable")
            && let Some(c) = s.get("capacity").cloned()
        {
            s.insert("allocatable".into(), c);
        }
        resource_list(s, "capacity");
        resource_list(s, "allocatable");
    }
}

fn service(m: &mut Obj) {
    let spec = m.entry("spec").or_insert_with(|| json!({}));
    let Some(spec) = spec.as_object_mut() else {
        return;
    };
    default_str(spec, "sessionAffinity", "None");
    match spec.get("sessionAffinity").and_then(Value::as_str) {
        Some("None") => {
            spec.remove("sessionAffinityConfig");
        }
        Some("ClientIP") => {
            let has_timeout = spec
                .get("sessionAffinityConfig")
                .and_then(|c| c.get("clientIP"))
                .and_then(|c| c.get("timeoutSeconds"))
                .is_some();
            if !has_timeout {
                spec.insert(
                    "sessionAffinityConfig".into(),
                    json!({"clientIP": {"timeoutSeconds": 10800}}),
                );
            }
        }
        _ => {}
    }
    default_str(spec, "type", "ClusterIP");
    for p in each(spec, "ports") {
        default_str(p, "protocol", "TCP");
        let unset = match p.get("targetPort") {
            None => true,
            Some(Value::String(s)) => s.is_empty(),
            Some(v) => v.as_i64() == Some(0),
        };
        if unset {
            let port = p.get("port").cloned().unwrap_or(json!(0));
            p.insert("targetPort".into(), port);
        }
    }
    let ty = spec
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    let external_ips = spec
        .get("externalIPs")
        .and_then(Value::as_array)
        .is_some_and(|a| !a.is_empty());
    let externally_accessible =
        ty == "LoadBalancer" || ty == "NodePort" || (ty == "ClusterIP" && external_ips);
    if externally_accessible {
        default_str(spec, "externalTrafficPolicy", "Cluster");
    }
    if matches!(ty.as_str(), "NodePort" | "LoadBalancer" | "ClusterIP") {
        default_ptr(spec, "internalTrafficPolicy", json!("Cluster"));
    }
    if ty == "LoadBalancer" {
        default_ptr(spec, "allocateLoadBalancerNodePorts", json!(true));
        if let Some(status) = obj(m, "status")
            && let Some(lb) = obj(status, "loadBalancer")
        {
            for ing in each(lb, "ingress") {
                if !is_unset_str(ing.get("ip")) {
                    default_ptr(ing, "ipMode", json!("VIP"));
                }
            }
        }
    }
}

fn pod(m: &mut Obj) {
    if let Some(spec) = obj(m, "spec") {
        // SetDefaults_Pod.
        if spec
            .get("terminationGracePeriodSeconds")
            .and_then(Value::as_i64)
            .is_some_and(|g| g < 0)
        {
            spec.insert("terminationGracePeriodSeconds".into(), json!(1));
        }
        for key in ["containers", "initContainers"] {
            for c in each(spec, key) {
                requests_from_limits(c);
            }
        }
        default_ptr(spec, "enableServiceLinks", json!(true));
        if spec.get("hostNetwork").and_then(Value::as_bool) == Some(true) {
            for key in ["containers", "initContainers"] {
                for c in each(spec, key) {
                    for p in each(c, "ports") {
                        let cp = p.get("containerPort").and_then(Value::as_i64).unwrap_or(0);
                        // hostPort is omitempty: a defaulted 0 stays absent.
                        if cp != 0 && p.get("hostPort").is_none_or(|h| h.as_i64() == Some(0)) {
                            p.insert("hostPort".into(), json!(cp));
                        }
                    }
                }
            }
        }
        pod_spec(spec);
        resource_list(spec, "overhead");
        resources(spec, "resources");
    }
    if let Some(status) = obj(m, "status") {
        pod_status(status);
    }
}

/// Requests default to limits, on pods only (not templates).
fn requests_from_limits(c: &mut Obj) {
    let Some(r) = obj(c, "resources") else {
        return;
    };
    let limits = match r.get("limits") {
        Some(Value::Object(l)) if !l.is_empty() => l.clone(),
        _ => return,
    };
    let req = r.entry("requests").or_insert_with(|| json!({}));
    if let Some(req) = req.as_object_mut() {
        for (k, v) in limits {
            req.entry(k).or_insert(v);
        }
    }
}

/// `SetObjectDefaults` for everything under a pod spec; shared with pod
/// templates when workloads arrive (M2), which skip `SetDefaults_Pod`.
pub fn pod_spec(spec: &mut Obj) {
    // SetDefaults_PodSpec.
    let sa = spec
        .get("serviceAccountName")
        .filter(|v| !is_unset_str(Some(v)))
        .or_else(|| spec.get("serviceAccount"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if sa.is_empty() {
        spec.remove("serviceAccountName");
        spec.remove("serviceAccount");
    } else {
        spec.insert("serviceAccountName".into(), json!(sa));
        spec.insert("serviceAccount".into(), json!(sa));
    }
    default_str(spec, "dnsPolicy", "ClusterFirst");
    default_str(spec, "restartPolicy", "Always");
    default_ptr(spec, "securityContext", json!({}));
    default_ptr(spec, "terminationGracePeriodSeconds", json!(30));
    default_str(spec, "schedulerName", "default-scheduler");

    for v in each(spec, "volumes") {
        volume(v);
    }
    for key in ["initContainers", "containers", "ephemeralContainers"] {
        for c in each(spec, key) {
            container(c);
        }
    }
}

fn volume(v: &mut Obj) {
    if v.keys().all(|k| k == "name") {
        v.insert("emptyDir".into(), json!({}));
    }
    if let Some(img) = obj(v, "image")
        && is_unset_str(img.get("pullPolicy"))
    {
        let p = pull_policy(img.get("reference").and_then(Value::as_str).unwrap_or(""));
        img.insert("pullPolicy".into(), json!(p));
    }
    if let Some(h) = obj(v, "hostPath") {
        default_ptr(h, "type", json!(""));
    }
    for key in ["secret", "configMap"] {
        if let Some(s) = obj(v, key) {
            default_ptr(s, "defaultMode", json!(0o644));
        }
    }
    if let Some(d) = obj(v, "downwardAPI") {
        default_ptr(d, "defaultMode", json!(0o644));
        downward_items(d);
    }
    if let Some(i) = obj(v, "iscsi") {
        default_str(i, "iscsiInterface", "default");
    }
    if let Some(r) = obj(v, "rbd") {
        default_str(r, "pool", "rbd");
        default_str(r, "user", "admin");
        default_str(r, "keyring", "/etc/ceph/keyring");
    }
    if let Some(a) = obj(v, "azureDisk") {
        default_ptr(a, "cachingMode", json!("ReadWrite"));
        default_ptr(a, "fsType", json!("ext4"));
        default_ptr(a, "readOnly", json!(false));
        default_ptr(a, "kind", json!("Shared"));
    }
    if let Some(p) = obj(v, "projected") {
        default_ptr(p, "defaultMode", json!(0o644));
        for s in each(p, "sources") {
            if let Some(d) = obj(s, "downwardAPI") {
                downward_items(d);
            }
            if let Some(t) = obj(s, "serviceAccountToken") {
                default_ptr(t, "expirationSeconds", json!(3600));
            }
        }
    }
    if let Some(s) = obj(v, "scaleIO") {
        default_str(s, "storageMode", "ThinProvisioned");
        default_str(s, "fsType", "xfs");
    }
    if let Some(e) = obj(v, "ephemeral")
        && let Some(t) = obj(e, "volumeClaimTemplate")
    {
        let spec = t.entry("spec").or_insert_with(|| json!({}));
        if let Some(spec) = spec.as_object_mut() {
            default_ptr(spec, "volumeMode", json!("Filesystem"));
            resources(spec, "resources");
        }
    }
}

fn downward_items(d: &mut Obj) {
    for item in each(d, "items") {
        if let Some(f) = obj(item, "fieldRef") {
            default_str(f, "apiVersion", "v1");
        }
    }
}

fn container(c: &mut Obj) {
    // SetDefaults_Container.
    if is_unset_str(c.get("imagePullPolicy")) {
        let p = pull_policy(c.get("image").and_then(Value::as_str).unwrap_or(""));
        c.insert("imagePullPolicy".into(), json!(p));
    }
    default_str(c, "terminationMessagePath", "/dev/termination-log");
    default_str(c, "terminationMessagePolicy", "File");

    for p in each(c, "ports") {
        default_str(p, "protocol", "TCP");
    }
    for e in each(c, "env") {
        if let Some(from) = obj(e, "valueFrom") {
            if let Some(f) = obj(from, "fieldRef") {
                default_str(f, "apiVersion", "v1");
            }
            if let Some(f) = obj(from, "fileKeyRef") {
                default_ptr(f, "optional", json!(false));
            }
        }
    }
    resources(c, "resources");
    for key in ["livenessProbe", "readinessProbe", "startupProbe"] {
        if let Some(p) = obj(c, key) {
            default_int(p, "timeoutSeconds", 1);
            default_int(p, "periodSeconds", 10);
            default_int(p, "successThreshold", 1);
            default_int(p, "failureThreshold", 3);
            http_get(p);
            if let Some(g) = obj(p, "grpc") {
                default_ptr(g, "service", json!(""));
            }
        }
    }
    if let Some(l) = obj(c, "lifecycle") {
        for key in ["postStart", "preStop"] {
            if let Some(h) = obj(l, key) {
                http_get(h);
            }
        }
    }
}

fn http_get(handler: &mut Obj) {
    if let Some(h) = obj(handler, "httpGet") {
        default_str(h, "path", "/");
        default_str(h, "scheme", "HTTP");
    }
}

fn pod_status(s: &mut Obj) {
    // Keep podIP and podIPs in sync; podIP wins.
    let ip = s
        .get("podIP")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    let first = s
        .get("podIPs")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .map(|p| p.get("ip").and_then(Value::as_str).unwrap_or("").to_owned());
    match (ip.is_empty(), first) {
        (false, None) => {
            s.insert("podIPs".into(), json!([{"ip": ip}]));
        }
        (true, Some(f)) => {
            if f.is_empty() {
                s.remove("podIP");
            } else {
                s.insert("podIP".into(), json!(f));
            }
        }
        (false, Some(f)) if f != ip => {
            s.insert("podIPs".into(), json!([{"ip": ip}]));
        }
        _ => {}
    }
    for key in [
        "initContainerStatuses",
        "containerStatuses",
        "ephemeralContainerStatuses",
    ] {
        for c in each(s, key) {
            resource_list(c, "allocatedResources");
            resources(c, "resources");
        }
    }
    resource_list(s, "allocatedResources");
    resources(s, "resources");
}

/// `SetDefaults_Container`'s pull policy: `Always` for `:latest` (or no
/// tag and no digest), else `IfNotPresent`. An image the reference
/// grammar rejects has no tag, so `IfNotPresent`.
pub fn pull_policy(image: &str) -> &'static str {
    match crate::image::tag(image) {
        Some(t) if t == "latest" => "Always",
        _ => "IfNotPresent",
    }
}
