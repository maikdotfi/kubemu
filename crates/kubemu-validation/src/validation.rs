//! Per-resource validation (ADR 0013): the metadata rules every resource
//! shares, plus the rules for mistakes people plausibly make in the
//! objects kubemu acts on. Upstream: `pkg/apis/core/validation`, through
//! the registry strategies (`rest.BeforeCreate`).
//!
//! Ported, for pods: volume names and the one-source rule; container
//! names (unique), images, ports, env names, volume mounts, pull and
//! termination message policies, requests against limits; restart and
//! DNS policies; node selector, hostname and subdomain. For services:
//! ports (names, numbers, protocols, target ports), type, session
//! affinity, selector, external names, node ports on ClusterIP services,
//! cluster IPs and IP families (on create, and that they do not change).
//! Everything else is listed in [`UNPORTED`] and accepted unchecked.

use std::collections::BTreeSet;
use std::net::IpAddr;

use kubemu_api_types::ResourceInfo;
use serde_json::{Map, Value};

use crate::field::{
    Cause, duplicate, forbidden, invalid, invalid_json, invalid_num, not_found, not_supported,
    required,
};
use crate::meta::{self, NameRule, name_errors, name_rule};
use crate::quantity::Quantity;

/// Validation kubemu does not do, per kind: accepted without checks, so
/// the gap is visible (principle 1.6). Served in the compatibility report.
pub const UNPORTED: &[(&str, &str)] = &[
    ("*", "ownerReferences, finalizers and managedFields shape"),
    ("Pod", "updates: spec immutability (ValidatePodUpdate)"),
    ("Pod", "init and ephemeral containers"),
    ("Pod", "probes, lifecycle hooks, security contexts"),
    ("Pod", "volume sources' own fields; volume devices; subPath"),
    (
        "Pod",
        "env valueFrom and envFrom; resource names; hugepages",
    ),
    ("Pod", "affinity, tolerations, topology spread, DNS config"),
    ("Pod", "service account, node name, priority, runtime class"),
    (
        "Pod",
        "host port conflicts; pod-level resources; OS-specific rules",
    ),
    (
        "Service",
        "external IPs; updates: type-dependent fields and traffic policies",
    ),
    (
        "Service",
        "traffic policies, health check and load balancer fields",
    ),
    ("Service", "session affinity config; app protocol"),
    ("Endpoints", "everything but metadata"),
    ("EndpointSlice", "everything but metadata"),
    ("Event", "everything but metadata"),
    ("Node", "everything but metadata"),
    ("Namespace", "spec.finalizers"),
    ("Secret", "type-specific keys; data key names"),
    ("ConfigMap", "data key names and size"),
    ("ServiceAccount", "everything but metadata"),
];

/// Validates `obj`, an object of `r` about to be created: metadata first,
/// then the kind's own rules, in upstream order.
pub fn validate(r: &ResourceInfo, obj: &Value) -> Vec<Cause> {
    validate_with(r, obj, None)
}

/// Validates `obj` replacing `old`: the create rules, plus what may not
/// change (only Services' cluster IPs and families so far).
pub fn validate_update(r: &ResourceInfo, obj: &Value, old: &Value) -> Vec<Cause> {
    validate_with(r, obj, Some(old))
}

fn validate_with(r: &ResourceInfo, obj: &Value, old: Option<&Value>) -> Vec<Cause> {
    let mut c = Vec::new();
    object_meta(r, obj, &mut c);
    let spec = obj.get("spec").and_then(Value::as_object);
    let empty = Map::new();
    match (r.group, r.kind) {
        ("", "Pod") => pod_spec(spec.unwrap_or(&empty), &mut c),
        ("", "Service") => {
            let old_spec = old.map(|o| o.get("spec").and_then(Value::as_object).unwrap_or(&empty));
            service_spec(spec.unwrap_or(&empty), old_spec, &mut c)
        }
        _ => {}
    }
    c
}

fn object_meta(r: &ResourceInfo, obj: &Value, c: &mut Vec<Cause>) {
    let m = obj.get("metadata").and_then(Value::as_object);
    let s = |k: &str| {
        m.and_then(|m| m.get(k))
            .and_then(Value::as_str)
            .unwrap_or("")
    };
    meta::validate_name(s("name"), name_rule(r.group, r.plural), c);
    if r.namespaced {
        meta::validate_namespace_name(s("namespace"), c);
    }
    if let Some(Value::Object(l)) = m.and_then(|m| m.get("labels")) {
        meta::validate_labels(l, "metadata.labels", c);
    }
    if let Some(Value::Object(a)) = m.and_then(|m| m.get("annotations")) {
        meta::validate_annotations(a, c);
    }
}

fn str_of<'a>(m: &'a Map<String, Value>, k: &str) -> &'a str {
    m.get(k).and_then(Value::as_str).unwrap_or("")
}

fn int_of(m: &Map<String, Value>, k: &str) -> i64 {
    m.get(k).and_then(Value::as_i64).unwrap_or(0)
}

fn objects<'a>(m: &'a Map<String, Value>, k: &str) -> impl Iterator<Item = &'a Map<String, Value>> {
    m.get(k)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
}

/// `ValidateDNS1123Label`.
fn dns1123_label(value: &str, path: &str, c: &mut Vec<Cause>) {
    for e in name_errors(value, NameRule::Dns1123Label) {
        c.push(invalid(path, value, &e));
    }
}

/// `validation.IsValidPortNum`.
fn port_num(value: i64, path: &str, c: &mut Vec<Cause>) {
    if !(1..=65535).contains(&value) {
        c.push(invalid_num(
            path,
            value,
            "must be between 1 and 65535, inclusive",
        ));
    }
}

/// `validation.IsValidPortName`.
fn port_name_errors(name: &str) -> Vec<&'static str> {
    let mut e = Vec::new();
    if name.len() > 15 {
        e.push("must be no more than 15 characters");
    }
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        e.push("must contain only alpha-numeric characters (a-z, 0-9), and hyphens (-)");
    }
    if !name.bytes().any(|b| b.is_ascii_lowercase()) {
        e.push("must contain at least one letter (a-z)");
    }
    if name.contains("--") {
        e.push("must not contain consecutive hyphens");
    }
    if name.starts_with('-') || name.ends_with('-') {
        e.push("must not begin or end with a hyphen");
    }
    e
}

const PROTOCOLS: [&str; 3] = ["SCTP", "TCP", "UDP"];

fn protocol(p: &Map<String, Value>, path: &str, c: &mut Vec<Cause>) {
    let proto = str_of(p, "protocol");
    if proto.is_empty() {
        c.push(required(&format!("{path}.protocol"), ""));
    } else if !PROTOCOLS.contains(&proto) {
        c.push(not_supported(
            &format!("{path}.protocol"),
            proto,
            &PROTOCOLS,
        ));
    }
}

/// `ValidatePodSpec`, the ported part.
fn pod_spec(spec: &Map<String, Value>, c: &mut Vec<Cause>) {
    if !spec.contains_key("terminationGracePeriodSeconds") {
        c.push(required("spec.terminationGracePeriodSeconds", ""));
    }
    let volumes = volumes(spec, c);
    containers(spec, &volumes, c);
    match str_of(spec, "restartPolicy") {
        "Always" | "OnFailure" | "Never" => {}
        "" => c.push(required("spec.restartPolicy", "")),
        other => c.push(not_supported(
            "spec.restartPolicy",
            other,
            &["Always", "OnFailure", "Never"],
        )),
    }
    match str_of(spec, "dnsPolicy") {
        "ClusterFirstWithHostNet" | "ClusterFirst" | "Default" | "None" => {}
        "" => c.push(required("spec.dnsPolicy", "")),
        other => c.push(not_supported(
            "spec.dnsPolicy",
            other,
            &["ClusterFirstWithHostNet", "ClusterFirst", "Default", "None"],
        )),
    }
    if let Some(Value::Object(sel)) = spec.get("nodeSelector") {
        meta::validate_labels(sel, "spec.nodeSelector", c);
    }
    for k in ["hostname", "subdomain"] {
        let v = str_of(spec, k);
        if !v.is_empty() {
            dns1123_label(v, &format!("spec.{k}"), c);
        }
    }
}

/// Volume source fields, in the order `validateVolumeSource` counts them,
/// with the path segment it reports (upstream typos included).
const VOLUME_SOURCES: &[(&str, &str)] = &[
    ("emptyDir", "emptyDir"),
    ("hostPath", "hostPath"),
    ("gitRepo", "gitRepo"),
    ("gcePersistentDisk", "gcePersistentDisk"),
    ("awsElasticBlockStore", "awsElasticBlockStore"),
    ("secret", "secret"),
    ("nfs", "nfs"),
    ("iscsi", "iscsi"),
    ("glusterfs", "glusterfs"),
    ("flocker", "flocker"),
    ("persistentVolumeClaim", "persistentVolumeClaim"),
    ("rbd", "rbd"),
    ("cinder", "cinder"),
    ("cephfs", "cephFS"),
    ("quobyte", "quobyte"),
    ("downwardAPI", "downwarAPI"),
    ("fc", "fc"),
    ("flexVolume", "flexVolume"),
    ("configMap", "configMap"),
    ("azureFile", "azureFile"),
    ("vsphereVolume", "vsphereVolume"),
    ("photonPersistentDisk", "photonPersistentDisk"),
    ("portworxVolume", "portworxVolume"),
    ("azureDisk", "azureDisk"),
    ("storageos", "storageos"),
    ("projected", "projected"),
    ("scaleIO", "scaleIO"),
    ("csi", "csi"),
    ("ephemeral", "ephemeral"),
    ("image", "image"),
];

/// `ValidateVolumes`: the names of the volumes that are valid.
fn volumes(spec: &Map<String, Value>, c: &mut Vec<Cause>) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for (i, v) in objects(spec, "volumes").enumerate() {
        let path = format!("spec.volumes[{i}]");
        let mut el = Vec::new();
        let mut n = 0;
        for (key, seg) in VOLUME_SOURCES {
            if v.get(*key).is_some_and(|x| !x.is_null()) {
                if n > 0 {
                    el.push(forbidden(
                        &format!("{path}.{seg}"),
                        "may not specify more than 1 volume type",
                    ));
                } else {
                    n += 1;
                }
            }
        }
        if n == 0 {
            el.push(required(&path, "must specify a volume type"));
        }
        let name = str_of(v, "name");
        if name.is_empty() {
            el.push(required(&format!("{path}.name"), ""));
        } else {
            dns1123_label(name, &format!("{path}.name"), &mut el);
        }
        if names.contains(name) {
            el.push(duplicate(&format!("{path}.name"), name));
        }
        if el.is_empty() {
            names.insert(name.to_owned());
        } else {
            c.extend(el);
        }
    }
    names
}

/// `validateContainers` for regular containers.
fn containers(spec: &Map<String, Value>, volumes: &BTreeSet<String>, c: &mut Vec<Cause>) {
    let list: Vec<&Map<String, Value>> = objects(spec, "containers").collect();
    if list.is_empty() {
        c.push(required("spec.containers", ""));
        return;
    }
    let mut names = BTreeSet::new();
    for (i, ctr) in list.into_iter().enumerate() {
        let path = format!("spec.containers[{i}]");
        container_common(ctr, volumes, &path, c);
        let name = str_of(ctr, "name");
        if !names.insert(name) {
            c.push(duplicate(&format!("{path}.name"), name));
        }
    }
}

/// `validateContainerCommon`, the ported part.
fn container_common(
    ctr: &Map<String, Value>,
    volumes: &BTreeSet<String>,
    path: &str,
    c: &mut Vec<Cause>,
) {
    let name = str_of(ctr, "name");
    if name.is_empty() {
        c.push(required(&format!("{path}.name"), ""));
    } else {
        dns1123_label(name, &format!("{path}.name"), c);
    }
    if str_of(ctr, "image").is_empty() {
        c.push(required(&format!("{path}.image"), ""));
    }
    match str_of(ctr, "terminationMessagePolicy") {
        "File" | "FallbackToLogsOnError" => {}
        "" => c.push(required(&format!("{path}.terminationMessagePolicy"), "")),
        other => c.push(not_supported(
            &format!("{path}.terminationMessagePolicy"),
            other,
            &["File", "FallbackToLogsOnError"],
        )),
    }
    container_ports(ctr, path, c);
    for (i, e) in objects(ctr, "env").enumerate() {
        let p = format!("{path}.env[{i}].name");
        let n = str_of(e, "name");
        if n.is_empty() {
            c.push(required(&p, ""));
        } else if !n
            .chars()
            .all(|ch| ch.is_ascii() && !ch.is_ascii_control() && ch != '=')
        {
            c.push(invalid(
                &p,
                n,
                "a valid environment variable name must consist only of printable ASCII characters other than '='",
            ));
        }
    }
    let mut mount_paths = BTreeSet::new();
    for (i, m) in objects(ctr, "volumeMounts").enumerate() {
        let p = format!("{path}.volumeMounts[{i}]");
        let n = str_of(m, "name");
        if n.is_empty() {
            c.push(required(&format!("{p}.name"), ""));
        }
        if !volumes.contains(n) {
            c.push(not_found(&format!("{p}.name"), n));
        }
        let mp = str_of(m, "mountPath");
        if mp.is_empty() {
            c.push(required(&format!("{p}.mountPath"), ""));
        }
        if !mount_paths.insert(mp) {
            c.push(invalid(&format!("{p}.mountPath"), mp, "must be unique"));
        }
    }
    match str_of(ctr, "imagePullPolicy") {
        "Always" | "IfNotPresent" | "Never" => {}
        "" => c.push(required(&format!("{path}.imagePullPolicy"), "")),
        other => c.push(not_supported(
            &format!("{path}.imagePullPolicy"),
            other,
            &["Always", "IfNotPresent", "Never"],
        )),
    }
    if let Some(r) = ctr.get("resources").and_then(Value::as_object) {
        resource_requirements(r, &format!("{path}.resources"), c);
    }
}

fn container_ports(ctr: &Map<String, Value>, path: &str, c: &mut Vec<Cause>) {
    let mut names = BTreeSet::new();
    for (i, p) in objects(ctr, "ports").enumerate() {
        let pp = format!("{path}.ports[{i}]");
        let name = str_of(p, "name");
        if !name.is_empty() {
            let errs = port_name_errors(name);
            if !errs.is_empty() {
                for e in errs {
                    c.push(invalid(&format!("{pp}.name"), name, e));
                }
            } else if !names.insert(name) {
                c.push(duplicate(&format!("{pp}.name"), name));
            }
        }
        let cp = int_of(p, "containerPort");
        if cp == 0 {
            c.push(required(&format!("{pp}.containerPort"), ""));
        } else {
            port_num(cp, &format!("{pp}.containerPort"), c);
        }
        let hp = int_of(p, "hostPort");
        if hp != 0 {
            port_num(hp, &format!("{pp}.hostPort"), c);
        }
        protocol(p, &pp, c);
    }
}

/// `validateResourceRequirements`, for cpu and memory: non-negative, and
/// requests no more than limits.
fn resource_requirements(r: &Map<String, Value>, path: &str, c: &mut Vec<Cause>) {
    let empty = Map::new();
    let list = |k: &str| r.get(k).and_then(Value::as_object).unwrap_or(&empty);
    let (limits, requests) = (list("limits"), list("requests"));
    let parse = |v: &Value| v.as_str().and_then(|s| Quantity::parse(s).ok());
    for (k, v) in limits {
        nonnegative(v, &format!("{path}.limits[{k}]"), c);
    }
    for (k, v) in requests {
        nonnegative(v, &format!("{path}.requests[{k}]"), c);
        if let Some(l) = limits.get(k)
            && let (Some(rq), Some(lq)) = (parse(v), parse(l))
            && rq.cmp_value(&lq).is_gt()
        {
            c.push(invalid(
                &format!("{path}.requests"),
                v.as_str().unwrap_or(""),
                &format!(
                    "must be less than or equal to {k} limit of {}",
                    l.as_str().unwrap_or("")
                ),
            ));
        }
    }
}

fn nonnegative(v: &Value, path: &str, c: &mut Vec<Cause>) {
    if let Some(s) = v.as_str()
        && Quantity::parse(s).is_ok_and(|q| q.is_negative())
    {
        c.push(invalid(path, s, "must be greater than or equal to 0"));
    }
}

/// The strings of a string list field (`clusterIPs`, `ipFamilies`).
fn strs<'a>(m: &'a Map<String, Value>, k: &str) -> Vec<&'a str> {
    m.get(k)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

/// `isHeadlessService`: `clusterIPs` is exactly `["None"]`.
pub fn is_headless(spec: &Map<String, Value>) -> bool {
    strs(spec, "clusterIPs") == ["None"]
}

/// An IPv4 address as Go's `netutils.ParseIPSloppy` reads it: leading
/// zeros allowed in each octet.
fn sloppy_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    parts.len() == 4
        && parts.iter().all(|p| {
            let digits = p.trim_start_matches('0');
            !p.is_empty()
                && p.bytes().all(|b| b.is_ascii_digit())
                && digits.len() <= 3
                && digits.parse::<u16>().unwrap_or(0) <= 255
        })
}

/// `IsValidIPForLegacyField` with strict validation (on by default since
/// 1.36): IPs a stored object already had pass unchecked.
fn legacy_ip(path: &str, value: &str, existing: &[&str], c: &mut Vec<Cause>) -> bool {
    if existing.contains(&value) {
        return true;
    }
    let detail = match value.parse::<IpAddr>() {
        Ok(IpAddr::V6(a)) if a.to_ipv4_mapped().is_some() => {
            "must not be an IPv4-mapped IPv6 address"
        }
        Ok(_) => return true,
        Err(_) if sloppy_ipv4(value) => "must not have leading 0s",
        Err(_) => "must be a valid IP address, (e.g. 10.9.8.7 or 2001:db8::ffff)",
    };
    c.push(invalid(path, value, detail));
    false
}

/// `netutils.IsIPv6String`, for IPs that passed [`legacy_ip`].
fn is_ipv6(ip: &str) -> bool {
    ip.parse::<std::net::Ipv6Addr>()
        .is_ok_and(|a| a.to_ipv4_mapped().is_none())
}

fn json_list(xs: &[&str]) -> Value {
    if xs.is_empty() {
        Value::Null
    } else {
        Value::from(xs.to_vec())
    }
}

/// `ValidateServiceClusterIPsRelatedFields`: `clusterIP`, `clusterIPs`,
/// `ipFamilies` and `ipFamilyPolicy` on their own and against each other.
/// The API also runs it before allocating cluster IPs, as upstream's
/// allocator does.
pub fn service_cluster_ips(
    spec: &Map<String, Value>,
    old: Option<&Map<String, Value>>,
) -> Vec<Cause> {
    let mut c = Vec::new();
    if str_of(spec, "type") == "ExternalName" {
        return c;
    }
    let ip = str_of(spec, "clusterIP");
    let ips = strs(spec, "clusterIPs");
    let families = strs(spec, "ipFamilies");
    if !ip.is_empty() {
        if ips.is_empty() {
            c.push(required("spec.clusterIPs", ""));
        } else if ips[0] != ip {
            c.push(invalid_json(
                "spec.clusterIPs",
                &json_list(&ips),
                "first value must match `clusterIP`",
            ));
        }
    } else if !ips.is_empty() {
        c.push(invalid_json(
            "spec.clusterIPs",
            &json_list(&ips),
            "must be empty when `clusterIP` is not specified",
        ));
    }
    let mut seen = BTreeSet::new();
    for (i, f) in families.iter().enumerate() {
        let path = format!("spec.ipFamilies[{i}]");
        if !matches!(*f, "IPv4" | "IPv6") {
            c.push(not_supported(&path, f, &["IPv4", "IPv6"]));
        }
        if !seen.insert(*f) {
            c.push(duplicate(&path, f));
        }
    }
    if let Some(p) = spec.get("ipFamilyPolicy").and_then(Value::as_str)
        && !matches!(p, "SingleStack" | "PreferDualStack" | "RequireDualStack")
    {
        c.push(not_supported(
            "spec.ipFamilyPolicy",
            p,
            &["PreferDualStack", "RequireDualStack", "SingleStack"],
        ));
    }
    let existing = old.map(|o| strs(o, "clusterIPs")).unwrap_or_default();
    let mut bad_ips = false;
    for (i, x) in ips.iter().enumerate() {
        if i == 0 && *x == "None" {
            if ips.len() > 1 {
                bad_ips = true;
                c.push(invalid_json(
                    "spec.clusterIPs",
                    &json_list(&ips),
                    "'None' must be the first and only value",
                ));
            }
            continue;
        }
        bad_ips |= !legacy_ip(&format!("spec.clusterIPs[{i}]"), x, &existing, &mut c);
    }
    if ips.len() > 2 {
        c.push(invalid_json(
            "spec.clusterIPs",
            &json_list(&ips),
            "may only hold up to 2 values",
        ));
    }
    if bad_ips {
        return c;
    }
    if ips.len() > 1 && !(ips.iter().any(|x| is_ipv6(x)) && ips.iter().any(|x| !is_ipv6(x))) {
        c.push(invalid_json(
            "spec.clusterIPs",
            &json_list(&ips),
            "may specify no more than one IP for each IP family",
        ));
    }
    if !is_headless(spec) {
        for (i, (x, f)) in ips.iter().zip(&families).enumerate() {
            let want = match (*f, is_ipv6(x)) {
                ("IPv4", true) => "IPv4",
                ("IPv6", false) => "IPv6",
                _ => continue,
            };
            c.push(invalid(
                &format!("spec.clusterIPs[{i}]"),
                x,
                &format!("expected an {want} value as indicated by `ipFamilies[{i}]`"),
            ));
        }
    }
    c
}

/// `validateUpgradeDowngradeClusterIPs` and `...IPFamilies`: cluster IPs
/// and families do not change once set, except to or from ExternalName.
fn service_update(spec: &Map<String, Value>, old: &Map<String, Value>, c: &mut Vec<Cause>) {
    if str_of(spec, "type") == "ExternalName" || str_of(old, "type") == "ExternalName" {
        return;
    }
    let (new_headless, old_headless) = (is_headless(spec), is_headless(old));
    let policy = spec.get("ipFamilyPolicy").cloned().unwrap_or(Value::Null);
    let single = policy == "SingleStack";
    let (ips, old_ips) = (strs(spec, "clusterIPs"), strs(old, "clusterIPs"));
    let ips_json = json_list(&ips);
    let changed = |i: usize| {
        invalid_json(
            &format!("spec.clusterIPs[{i}]"),
            &ips_json,
            "may not change once set",
        )
    };
    if !(old_headless && new_headless) {
        if old_ips.len() == ips.len() {
            for (i, (a, b)) in old_ips.iter().zip(&ips).enumerate() {
                if a != b {
                    c.push(changed(i));
                }
            }
        } else if old_ips.len() > ips.len() {
            if ips.is_empty() {
                c.push(invalid_json(
                    "spec.clusterIPs[0]",
                    &ips_json,
                    "primary clusterIP can not be unset",
                ));
            }
            if !old_ips.is_empty() && !ips.is_empty() && ips[0] != old_ips[0] {
                c.push(changed(0));
            }
            if ips.len() == 1 && !single {
                c.push(invalid_json(
                    "spec.ipFamilyPolicy",
                    &policy,
                    "must be set to 'SingleStack' when releasing the secondary clusterIP",
                ));
            }
        } else if !old_ips.is_empty() && ips[0] != old_ips[0] {
            c.push(changed(0));
        }
    }
    if new_headless != old_headless || new_headless {
        return;
    }
    let (fams, old_fams) = (strs(spec, "ipFamilies"), strs(old, "ipFamilies"));
    let changed_fam = |v: &[&str]| {
        invalid_json(
            "spec.ipFamilies[0]",
            &json_list(v),
            "may not change once set",
        )
    };
    if old_fams.len() == fams.len() {
        for (a, b) in old_fams.iter().zip(&fams) {
            if a != b {
                c.push(changed_fam(&fams));
            }
        }
    } else if old_fams.len() > fams.len() {
        if ips.is_empty() {
            c.push(invalid_json(
                "spec.ipFamilies[0]",
                &json_list(&fams),
                "primary ipFamily can not be unset",
            ));
        }
        if !fams.is_empty() && fams[0] != old_fams[0] {
            c.push(changed_fam(&ips));
        }
        if fams.len() == 1 && !single {
            c.push(invalid_json(
                "spec.ipFamilyPolicy",
                &policy,
                "must be set to 'SingleStack' when releasing the secondary ipFamily",
            ));
        }
    } else if !old_fams.is_empty() && !fams.is_empty() && fams[0] != old_fams[0] {
        c.push(changed_fam(&ips));
    }
}

/// `validateService`, the ported part.
fn service_spec(spec: &Map<String, Value>, old: Option<&Map<String, Value>>, c: &mut Vec<Cause>) {
    let ty = str_of(spec, "type");
    let ports: Vec<&Map<String, Value>> = objects(spec, "ports").collect();
    let headless = is_headless(spec);
    if ports.is_empty() && !headless && ty != "ExternalName" {
        c.push(required("spec.ports", ""));
    }
    if headless && matches!(ty, "LoadBalancer" | "NodePort") {
        c.push(invalid(
            "spec.clusterIPs[0]",
            "None",
            &format!("may not be set to 'None' for {ty} services"),
        ));
    }
    if ty == "ExternalName" {
        if !strs(spec, "clusterIPs").is_empty() {
            c.push(forbidden(
                "spec.clusterIPs",
                "may not be set for ExternalName services",
            ));
        }
        if !strs(spec, "ipFamilies").is_empty() {
            c.push(forbidden(
                "spec.ipFamilies",
                "may not be set for ExternalName services",
            ));
        }
        if spec.get("ipFamilyPolicy").is_some_and(|v| !v.is_null()) {
            c.push(forbidden(
                "spec.ipFamilyPolicy",
                "may not be set for ExternalName services",
            ));
        }
    }
    if ty == "ExternalName" {
        let cname = str_of(spec, "externalName");
        let cname = cname.strip_suffix('.').unwrap_or(cname);
        if cname.is_empty() {
            c.push(required("spec.externalName", ""));
        } else {
            for e in name_errors(cname, NameRule::Dns1123Subdomain) {
                c.push(invalid("spec.externalName", cname, &e));
            }
        }
    }
    let mut names = BTreeSet::new();
    let require_name = ports.len() > 1;
    for (i, p) in ports.iter().enumerate() {
        let pp = format!("spec.ports[{i}]");
        let name = str_of(p, "name");
        if require_name && name.is_empty() {
            c.push(required(&format!("{pp}.name"), ""));
        } else if !name.is_empty() {
            dns1123_label(name, &format!("{pp}.name"), c);
            if !names.insert(name) {
                c.push(duplicate(&format!("{pp}.name"), name));
            }
        }
        port_num(int_of(p, "port"), &format!("{pp}.port"), c);
        protocol(p, &pp, c);
        let tp = format!("{pp}.targetPort");
        match p.get("targetPort") {
            Some(Value::String(s)) => {
                for e in port_name_errors(s) {
                    c.push(invalid(&tp, s, e));
                }
            }
            v => port_num(v.and_then(Value::as_i64).unwrap_or(0), &tp, c),
        }
    }
    if let Some(Value::Object(sel)) = spec.get("selector") {
        meta::validate_labels(sel, "spec.selector", c);
    }
    match str_of(spec, "sessionAffinity") {
        "" => c.push(required("spec.sessionAffinity", "")),
        "ClientIP" | "None" => {}
        other => c.push(not_supported(
            "spec.sessionAffinity",
            other,
            &["ClientIP", "None"],
        )),
    }
    if str_of(spec, "sessionAffinity") == "None" && spec.contains_key("sessionAffinityConfig") {
        c.push(forbidden(
            "spec.sessionAffinityConfig",
            "must not be set when session affinity is None",
        ));
    }
    c.extend(service_cluster_ips(spec, old));
    if let Some(old) = old {
        service_update(spec, old, c);
    }
    const TYPES: [&str; 4] = ["ClusterIP", "ExternalName", "LoadBalancer", "NodePort"];
    if ty.is_empty() {
        c.push(required("spec.type", ""));
    } else if !TYPES.contains(&ty) {
        c.push(not_supported("spec.type", ty, &TYPES));
    }
    if ty == "ClusterIP" {
        for (i, p) in ports.iter().enumerate() {
            if int_of(p, "nodePort") != 0 {
                c.push(forbidden(
                    &format!("spec.ports[{i}].nodePort"),
                    "may not be used when `type` is 'ClusterIP'",
                ));
            }
        }
    }
    let mut node_ports = BTreeSet::new();
    for (i, p) in ports.iter().enumerate() {
        let np = int_of(p, "nodePort");
        if np != 0 && !node_ports.insert((str_of(p, "protocol"), np)) {
            c.push(Cause {
                reason: "FieldValueDuplicate",
                message: format!("Duplicate value: {np}"),
                field: format!("spec.ports[{i}].nodePort"),
            });
        }
    }
    // Upstream reports the (protocol, port) key as the internal struct.
    let mut seen = BTreeSet::new();
    for (i, p) in ports.iter().enumerate() {
        let (proto, port) = (str_of(p, "protocol"), int_of(p, "port"));
        if !seen.insert((proto, port)) {
            c.push(Cause {
                reason: "FieldValueDuplicate",
                message: format!(
                    "Duplicate value: {{\"Name\":\"\",\"Protocol\":{proto:?},\"AppProtocol\":null,\"Port\":{port},\"TargetPort\":0,\"NodePort\":0}}"
                ),
                field: format!("spec.ports[{i}]"),
            });
        }
    }
}
