//! Service cluster IPs (README §12.3 step 1, ADR 0003): what upstream's
//! Service REST storage does to a Service before the registry strategy
//! sees it (`pkg/registry/core/service/storage`, `beginCreate` and
//! `beginUpdate`), for a single-stack IPv4 cluster:
//!
//! - `normalizeClusterIPs`, and on update `patchAllocatedValues`: keep
//!   `clusterIP` and `clusterIPs` in step, and carry the allocated IPs
//!   over an update that leaves them out;
//! - `initIPFamilyFields`: default `ipFamilyPolicy` and `ipFamilies`, and
//!   refuse families and policies the cluster cannot serve;
//! - allocation: a requested IP must be in the Service CIDR and free, with
//!   the 1.37 allocator's messages; otherwise a free one is picked.
//!
//! The IPs in use are the `clusterIPs` of the cluster's stored Services.
//! There is no separate record (no `IPAddress` or `ServiceCIDR` objects),
//! so deleting a Service frees its IPs, and forks and snapshots carry
//! them. Node ports are not allocated yet.

use std::collections::BTreeSet;
use std::net::Ipv4Addr;

use kubemu_env::Rng;
use kubemu_net::{Cidr, ClusterNetwork};
use kubemu_validation::field::{Cause, invalid_json};
use serde_json::{Map, Value, json};

use crate::status::{ApiError, ApiResult};

fn spec_mut(svc: &mut Value) -> &mut Map<String, Value> {
    if !svc["spec"].is_object() {
        svc["spec"] = json!({});
    }
    svc["spec"].as_object_mut().expect("set above")
}

fn str_of<'a>(spec: &'a Map<String, Value>, k: &str) -> &'a str {
    spec.get(k).and_then(Value::as_str).unwrap_or("")
}

fn strs(spec: &Map<String, Value>, k: &str) -> Vec<String> {
    spec.get(k)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn set_strs(spec: &mut Map<String, Value>, k: &str, v: &[String]) {
    if v.is_empty() {
        spec.remove(k);
    } else {
        spec.insert(k.into(), json!(v));
    }
}

fn policy(spec: &Map<String, Value>) -> Option<&str> {
    spec.get("ipFamilyPolicy").and_then(Value::as_str)
}

fn needs_cluster_ip(spec: &Map<String, Value>) -> bool {
    str_of(spec, "type") != "ExternalName"
}

fn selectorless(spec: &Map<String, Value>) -> bool {
    spec.get("selector")
        .and_then(Value::as_object)
        .is_none_or(Map::is_empty)
}

/// `familyOf`, for IPs that passed validation.
fn family_of(ip: &str) -> &'static str {
    match ip.parse::<std::net::IpAddr>() {
        Ok(std::net::IpAddr::V6(a)) if a.to_ipv4_mapped().is_none() => "IPv6",
        _ => "IPv4",
    }
}

fn invalid_service(svc: &Value, causes: Vec<Cause>) -> ApiError {
    ApiError::invalid("Service", "", crate::object::name(svc), causes)
}

/// The cluster IPs of every stored Service but `except` (a namespace and
/// name), in `cidr`.
pub(crate) fn used_ips<'a>(
    services: impl Iterator<Item = &'a Value>,
    except: Option<(&str, &str)>,
) -> BTreeSet<Ipv4Addr> {
    services
        .filter(|s| {
            except.is_none_or(|(ns, n)| {
                crate::object::namespace(s) != ns || crate::object::name(s) != n
            })
        })
        .flat_map(|s| {
            s["spec"]["clusterIPs"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|ip| ip.as_str()?.parse().ok())
        })
        .collect()
}

/// `beginCreate`: normalize, default families, allocate.
pub(crate) fn begin_create(
    svc: &mut Value,
    net: &ClusterNetwork,
    used: &BTreeSet<Ipv4Addr>,
    dry_run: bool,
    rng: &dyn Rng,
) -> ApiResult<()> {
    let spec = spec_mut(svc);
    // normalizeClusterIPs, create.
    let ip = str_of(spec, "clusterIP").to_owned();
    if !ip.is_empty() && strs(spec, "clusterIPs").is_empty() {
        spec.insert("clusterIPs".into(), json!([ip]));
    }
    init_ip_families(svc, None)?;
    alloc_cluster_ips(svc, net.service_cidr, used, dry_run, rng)
}

/// `beginUpdate`: carry allocated values over, normalize, default
/// families, and allocate when an ExternalName Service gets a type that
/// needs a cluster IP.
pub(crate) fn begin_update(
    svc: &mut Value,
    old: &Value,
    net: &ClusterNetwork,
    used: &BTreeSet<Ipv4Addr>,
    dry_run: bool,
    rng: &dyn Rng,
) -> ApiResult<()> {
    let empty = Map::new();
    let old_spec = old["spec"].as_object().unwrap_or(&empty);
    let (old_ip, old_ips) = (str_of(old_spec, "clusterIP"), strs(old_spec, "clusterIPs"));
    let spec = spec_mut(svc);
    // patchAllocatedValues.
    if needs_cluster_ip(old_spec) && needs_cluster_ip(spec) {
        if str_of(spec, "clusterIP").is_empty() {
            spec.insert("clusterIP".into(), json!(old_ip));
        }
        if strs(spec, "clusterIPs").is_empty() && !old_ips.is_empty() {
            set_strs(spec, "clusterIPs", &old_ips);
        }
    }
    // normalizeClusterIPs, update.
    let ip = str_of(spec, "clusterIP").to_owned();
    let ips = strs(spec, "clusterIPs");
    if !old_ips.is_empty() && ips.is_empty() && old_ip == ip {
        set_strs(spec, "clusterIPs", &old_ips);
    }
    if old_ip != ip {
        let same = strs(spec, "clusterIPs") == old_ips;
        if !old_ip.is_empty() && ip.is_empty() {
            if same {
                spec.remove("clusterIPs");
            }
        } else if same {
            set_strs(spec, "clusterIPs", &[ip]);
        }
    }
    init_ip_families(svc, Some(old_spec))?;
    if !needs_cluster_ip(old_spec) && needs_cluster_ip(svc["spec"].as_object().expect("set")) {
        return alloc_cluster_ips(svc, net.service_cidr, used, dry_run, rng);
    }
    upgrade_cluster_ips(svc, old_spec, net.service_cidr, used, dry_run, rng)
}

/// `isMatchingPreferDualStackClusterIPFields`: an update that leaves a
/// PreferDualStack Service's IPs and families alone is not re-examined.
fn matching_prefer_dual_stack(spec: &Map<String, Value>, old: Option<&Map<String, Value>>) -> bool {
    let Some(old) = old else { return false };
    let Some(p) = policy(spec) else { return false };
    let ty = str_of(spec, "type");
    str_of(old, "type") == ty
        && matches!(ty, "ClusterIP" | "NodePort" | "LoadBalancer")
        && p == "PreferDualStack"
        && policy(old).is_none_or(|p| p == "PreferDualStack")
        && strs(spec, "clusterIPs") == strs(old, "clusterIPs")
        && strs(spec, "ipFamilies") == strs(old, "ipFamilies")
}

/// `initIPFamilyFields` for a cluster whose only family is IPv4.
fn init_ip_families(svc: &mut Value, old: Option<&Map<String, Value>>) -> ApiResult<()> {
    const PRIMARY: &str = "IPv4";
    let spec = spec_mut(svc);
    if !needs_cluster_ip(spec) || matching_prefer_dual_stack(spec, old) {
        return Ok(());
    }
    let headless_selectorless = str_of(spec, "clusterIP") == "None" && selectorless(spec);
    if policy(spec).is_none() {
        let p = match old.and_then(policy) {
            Some(p) => p,
            None if headless_selectorless => "RequireDualStack",
            None => "SingleStack",
        };
        spec.insert("ipFamilyPolicy".into(), json!(p));
    }
    let causes = kubemu_validation::validation::service_cluster_ips(spec, old);
    if !causes.is_empty() {
        return Err(invalid_service(svc, causes));
    }
    let p = policy(spec).unwrap_or("").to_owned();
    let pv = json!(p);
    let mut ips = strs(spec, "clusterIPs");
    let mut families = strs(spec, "ipFamilies");
    let mut c = Vec::new();
    if let Some(old) = old {
        if p == "SingleStack" {
            if ips == strs(old, "clusterIPs") && ips.len() > 1 {
                ips.truncate(1);
            }
            if families == strs(old, "ipFamilies") && families.len() > 1 {
                families.truncate(1);
            }
        } else {
            if !ips.is_empty() && ips.len() < strs(old, "clusterIPs").len() {
                c.push(invalid_json(
                    "spec.ipFamilyPolicy",
                    &pv,
                    "must be 'SingleStack' to release the secondary cluster IP",
                ));
            }
            if !families.is_empty() && families.len() < strs(old, "ipFamilies").len() {
                c.push(invalid_json(
                    "spec.ipFamilyPolicy",
                    &pv,
                    "must be 'SingleStack' to release the secondary IP family",
                ));
            }
        }
    }
    if p == "SingleStack" {
        if ips.len() == 2 {
            c.push(invalid_json(
                "spec.ipFamilyPolicy",
                &pv,
                "must be 'RequireDualStack' or 'PreferDualStack' when multiple cluster IPs are specified",
            ));
        }
        if families.len() == 2 {
            c.push(invalid_json(
                "spec.ipFamilyPolicy",
                &pv,
                "must be 'RequireDualStack' or 'PreferDualStack' when multiple IP families are specified",
            ));
        }
    }
    for (i, ip) in ips.iter().enumerate() {
        if ip == "None" {
            break;
        }
        let fam = family_of(ip);
        if i >= families.len() {
            if fam == PRIMARY {
                families.push(fam.to_owned());
            } else {
                c.push(invalid_json(
                    &format!("spec.clusterIPs[{i}]"),
                    &json!(ips),
                    &format!("{fam} is not configured on this cluster"),
                ));
            }
        }
    }
    let commit = |spec: &mut Map<String, Value>, ips: &[String], families: &[String]| {
        set_strs(spec, "clusterIPs", ips);
        set_strs(spec, "ipFamilies", families);
    };
    if !c.is_empty() {
        return Err(invalid_service(svc, c));
    }
    if headless_selectorless {
        if families.is_empty() {
            families.push(PRIMARY.into());
        }
        if families.len() < 2 && p != "SingleStack" {
            let other = if families[0] == "IPv4" {
                "IPv6"
            } else {
                "IPv4"
            };
            families.push(other.into());
        }
        commit(spec, &ips, &families);
        return Ok(());
    }
    if p == "RequireDualStack" {
        c.push(invalid_json(
            "spec.ipFamilyPolicy",
            &pv,
            "this cluster is not configured for dual-stack services",
        ));
    }
    for (i, f) in families.iter().enumerate() {
        if f != PRIMARY {
            c.push(invalid_json(
                &format!("spec.ipFamilies[{i}]"),
                &json!(f),
                "not configured on this cluster",
            ));
        }
    }
    if !c.is_empty() {
        return Err(invalid_service(svc, c));
    }
    if families.is_empty() {
        families.push(PRIMARY.into());
    }
    commit(spec, &ips, &families);
    Ok(())
}

/// `allocClusterIPs` for a single-stack cluster: `ipFamilies` is
/// `["IPv4"]` by now, so one IP.
fn alloc_cluster_ips(
    svc: &mut Value,
    cidr: Cidr,
    used: &BTreeSet<Ipv4Addr>,
    dry_run: bool,
    rng: &dyn Rng,
) -> ApiResult<()> {
    let spec = spec_mut(svc);
    let mut ips = strs(spec, "clusterIPs");
    if !needs_cluster_ip(spec) || ips.first().is_some_and(|ip| ip == "None") {
        return Ok(());
    }
    if ips.is_empty() {
        ips.push(String::new());
    }
    let ip = alloc_ip(svc, &ips, 0, cidr, used, dry_run, rng)?;
    let spec = spec_mut(svc);
    spec.insert("clusterIP".into(), json!(ip));
    spec.insert("clusterIPs".into(), json!([ip]));
    Ok(())
}

/// `updateClusterIPs`, case C: a family was added. On a single-stack
/// cluster only the headless-selectorless default does that, adding the
/// secondary family to a primary of IPv6; when the added family is IPv4,
/// upstream allocates it (and validation then refuses the update). When
/// it is IPv6 upstream panics (no IPv6 allocator); kubemu leaves it to
/// validation.
fn upgrade_cluster_ips(
    svc: &mut Value,
    old: &Map<String, Value>,
    cidr: Cidr,
    used: &BTreeSet<Ipv4Addr>,
    dry_run: bool,
    rng: &dyn Rng,
) -> ApiResult<()> {
    let spec = spec_mut(svc);
    let families = strs(spec, "ipFamilies");
    let old_headless = strs(old, "clusterIPs")
        .first()
        .is_some_and(|ip| ip == "None");
    if !needs_cluster_ip(old)
        || old_headless
        || strs(old, "ipFamilies").len() != 1
        || families.len() != 2
        || families[1] != "IPv4"
    {
        return Ok(());
    }
    let mut ips = strs(spec, "clusterIPs");
    if ips.len() < 2 {
        ips.push(String::new());
    }
    let ip = alloc_ip(svc, &ips, 1, cidr, used, dry_run, rng)?;
    ips[1] = ip;
    set_strs(spec_mut(svc), "clusterIPs", &ips);
    Ok(())
}

/// `allocIPs` for `ips[i]` with the 1.37 allocator (`MetaAllocator` over
/// one ServiceCIDR; its dry run skips the CIDR match and the in-use
/// check): the requested IP, or a free one when it is empty.
fn alloc_ip(
    svc: &Value,
    ips: &[String],
    i: usize,
    cidr: Cidr,
    used: &BTreeSet<Ipv4Addr>,
    dry_run: bool,
    rng: &dyn Rng,
) -> ApiResult<String> {
    let requested = &ips[i];
    if requested.is_empty() {
        return kubemu_net::allocate(cidr.first_host(), cidr.last_host(), used, rng)
            .map(|ip| ip.to_string())
            .ok_or_else(|| {
                ApiError::internal(format!(
                    "Internal error occurred: failed to allocate a serviceIP for Service {:?}: range is full",
                    crate::object::name(svc)
                ))
            });
    }
    // Validation let through only well-formed addresses of the family.
    let addr: Ipv4Addr = requested.parse().map_err(|_| {
        ApiError::internal(format!(
            "Internal error occurred: failed to parse service IP {requested:?}"
        ))
    })?;
    let why = if !dry_run && !cidr.contains(addr) {
        "the provided network does not match the current range".to_owned()
    } else if addr < cidr.first_host() || addr > cidr.last_host() {
        format!(
            "the provided IP ({addr}) is not in the valid range. The range of valid IPs is {cidr}"
        )
    } else if !dry_run && used.contains(&addr) {
        "provided IP is already allocated".to_owned()
    } else {
        return Ok(requested.clone());
    };
    Err(invalid_service(
        svc,
        vec![invalid_json(
            "spec.clusterIPs",
            &json!(ips),
            &format!("failed to allocate IP {requested}: {why}"),
        )],
    ))
}
