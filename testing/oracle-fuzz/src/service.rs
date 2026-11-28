//! Service cluster IPs (README §12.3 step 1) against the oracle's `service`
//! op: a Service create or update through kubemu's API, and through
//! upstream's Service REST storage. Compared: the status code, the causes
//! of an `Invalid` error, and on success what IP allocation decides
//! (`clusterIP`, `clusterIPs`, `ipFamilies`, `ipFamilyPolicy`).
//!
//! Which free IP gets picked is not compared (both pick at random), only
//! that it is free and in the CIDR. Requested IPv4 addresses stay inside
//! the CIDR, where the oracle's allocator words errors as 1.37's does.

use std::net::Ipv4Addr;

use kubemu_api::{Api, Cidr, ClusterNetwork, HttpRequest};
use serde_json::{Map, Value, json};

use crate::{CaseResult, Gen, Oracle};

const SVCS: &str = "/api/v1/namespaces/default/services";

fn pick<'a, T>(g: &mut Gen, xs: &'a [T]) -> &'a T {
    &xs[g.below(xs.len() as u64) as usize]
}

/// An IP or IP-like string for `clusterIP`/`clusterIPs`: free and taken
/// hosts, the network and broadcast addresses, `None`, malformed and IPv6.
fn ip(g: &mut Gen, cidr: Cidr, used: &[String]) -> String {
    let host = |n: u32| Ipv4Addr::from(u32::from(cidr.first_host()) + n).to_string();
    match g.below(12) {
        0..=3 => host(2 + g.below(5) as u32),
        4 => used.first().cloned().unwrap_or_else(|| host(0)),
        5 => cidr.network().to_string(),
        6 => cidr.broadcast().to_string(),
        7 | 11 => "None".into(),
        8 => (*pick(g, &["", "10.96.0.300", "010.96.0.5", "x"])).into(),
        9 => (*pick(g, &["fd00::10", "::ffff:10.96.0.5"])).into(),
        _ => host(3),
    }
}

fn service_spec(g: &mut Gen, cidr: Cidr, used: &[String]) -> Map<String, Value> {
    let mut spec = Map::new();
    let ty = *pick(
        g,
        &[
            "",
            "",
            "ClusterIP",
            "NodePort",
            "LoadBalancer",
            "ExternalName",
        ],
    );
    if !ty.is_empty() {
        spec.insert("type".into(), json!(ty));
    }
    if ty == "ExternalName" {
        spec.insert("externalName".into(), json!("db.example.com"));
    }
    if ty != "ExternalName" || g.chance(1, 4) {
        spec.insert("ports".into(), json!([{"port": 80}]));
    }
    if g.chance(1, 2) {
        spec.insert("selector".into(), json!({"app": "a"}));
    }
    let primary = ip(g, cidr, used);
    if g.chance(2, 3) {
        spec.insert("clusterIP".into(), json!(primary));
    }
    match g.below(10) {
        0 => {
            spec.insert("clusterIPs".into(), json!([primary]));
        }
        1 => {
            spec.insert("clusterIPs".into(), json!([primary, "fd00::1"]));
        }
        2 => {
            let other = ip(g, cidr, used);
            spec.insert("clusterIPs".into(), json!([other]));
        }
        3 => {
            spec.insert("clusterIPs".into(), json!([primary, "None"]));
        }
        _ => {}
    }
    if g.chance(1, 4) {
        let f = pick(
            g,
            &[
                json!(["IPv4"]),
                json!(["IPv6"]),
                json!(["IPv4", "IPv6"]),
                json!(["IPv6", "IPv4"]),
                json!(["IPv4", "IPv4"]),
                json!(["IPv5"]),
            ],
        )
        .clone();
        spec.insert("ipFamilies".into(), f);
    }
    if g.chance(1, 4) {
        let p = *pick(
            g,
            &[
                "SingleStack",
                "PreferDualStack",
                "RequireDualStack",
                "Bogus",
            ],
        );
        spec.insert("ipFamilyPolicy".into(), json!(p));
    }
    spec
}

/// A valid Service to update: allocated, requested, headless or
/// ExternalName.
fn old_spec(g: &mut Gen, cidr: Cidr) -> Value {
    let requested = Ipv4Addr::from(u32::from(cidr.first_host()) + 9).to_string();
    let ports = json!([{"port": 80}]);
    pick(
        g,
        &[
            json!({"selector": {"app": "a"}, "ports": ports}),
            json!({"clusterIP": requested, "ports": ports}),
            json!({"clusterIP": "None", "selector": {"app": "a"}, "ports": ports}),
            json!({"clusterIP": "None"}),
            json!({"ipFamilyPolicy": "PreferDualStack", "ports": ports}),
            json!({"type": "NodePort", "ports": ports}),
            json!({"type": "ExternalName", "externalName": "db.example.com"}),
        ],
    )
    .clone()
}

/// An update of `stored`: retyped, IPs dropped or moved, families and
/// policy changed.
fn mutate(g: &mut Gen, stored: &Value, cidr: Cidr, used: &[String]) -> Value {
    let mut spec = stored["spec"].as_object().cloned().unwrap_or_default();
    for _ in 0..1 + g.below(3) {
        match g.below(8) {
            0 => {
                spec.insert("type".into(), json!("ExternalName"));
                spec.insert("externalName".into(), json!("db.example.com"));
            }
            1 => {
                spec.insert("type".into(), json!(*pick(g, &["ClusterIP", "NodePort"])));
                spec.insert("ports".into(), json!([{"port": 80}]));
            }
            2 => {
                spec.remove("clusterIP");
                spec.remove("clusterIPs");
            }
            3 => {
                let x = ip(g, cidr, used);
                spec.insert("clusterIP".into(), json!(x));
                if g.chance(1, 2) {
                    spec.insert("clusterIPs".into(), json!([x]));
                }
            }
            4 => {
                spec.remove("ipFamilies");
            }
            5 => {
                spec.remove("ipFamilyPolicy");
            }
            6 => {
                let p = *pick(g, &["SingleStack", "PreferDualStack", "RequireDualStack"]);
                spec.insert("ipFamilyPolicy".into(), json!(p));
            }
            _ => {
                let f = pick(
                    g,
                    &[json!(["IPv4"]), json!(["IPv4", "IPv6"]), json!(["IPv6"])],
                )
                .clone();
                spec.insert("ipFamilies".into(), f);
            }
        }
    }
    json!({"apiVersion": "v1", "kind": "Service",
        "metadata": {"name": "s", "namespace": "default"}, "spec": spec})
}

/// Causes as a sorted list, with quoted IPv4 addresses nobody asked for
/// (picked at random, so they differ) replaced by `<allocated>`.
fn sorted_causes(v: &Value, asked: &[String]) -> Vec<(String, String, String)> {
    let mask = |m: &str| -> String {
        m.split('"')
            .enumerate()
            .map(|(i, part)| {
                let picked = i % 2 == 1
                    && part.parse::<Ipv4Addr>().is_ok()
                    && !asked.iter().any(|a| a == part);
                if picked { "<allocated>" } else { part }
            })
            .collect::<Vec<_>>()
            .join("\"")
    };
    let mut out: Vec<_> = v
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| {
            let s = |k: &str| c[k].as_str().unwrap_or("").to_owned();
            (s("reason"), s("field"), mask(&s("message")))
        })
        .collect();
    out.sort();
    out
}

/// What allocation decided, with IPs neither side was asked for replaced
/// by `<allocated>` (after checking kubemu's is free and in the CIDR).
fn ip_fields(obj: &Value, asked: &[String], cidr: Cidr, used: &[String]) -> Result<Value, String> {
    let s = &obj["spec"];
    let mut ips: Vec<Value> = Vec::new();
    for x in s["clusterIPs"].as_array().into_iter().flatten() {
        let x = x.as_str().unwrap_or("");
        if asked.iter().any(|a| a == x) || x == "None" {
            ips.push(json!(x));
            continue;
        }
        let ok = x.parse::<Ipv4Addr>().is_ok_and(|a| {
            a >= cidr.first_host() && a <= cidr.last_host() && !used.iter().any(|u| u == x)
        });
        if !ok {
            return Err(format!("allocated {x}, not free in {cidr}"));
        }
        ips.push(json!("<allocated>"));
    }
    let primary = ips.first().cloned().unwrap_or(Value::Null);
    let clusterip = match s["clusterIP"].as_str() {
        None | Some("") => Value::Null,
        Some(_) => primary,
    };
    let list = |v: &Value| match v.as_array() {
        Some(a) if !a.is_empty() => v.clone(),
        _ => Value::Null,
    };
    Ok(json!({
        "clusterIP": clusterip,
        "clusterIPs": list(&json!(ips)),
        "ipFamilies": list(&s["ipFamilies"]),
        "ipFamilyPolicy": s["ipFamilyPolicy"],
    }))
}

/// One case: a create, or an update of a Service kubemu stored.
pub fn service_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let cidr: Cidr = pick(&mut g, &["10.96.0.0/24", "10.96.0.0/28"])
        .parse()
        .expect("valid");
    let net = ClusterNetwork::new("10.64.0.0/16".parse().expect("valid"), cidr).expect("disjoint");
    let (env, _) = kubemu_env_real::real_env(Some(seed));
    let mut api = Api::new(env);
    api.add_cluster_with("c", net);
    let call = |api: &mut Api, method: &str, path: &str, body: &Value| {
        let r = api
            .handle("c", &HttpRequest::new(method, path).json(body))
            .response();
        (r.status, r.value().unwrap_or(Value::Null))
    };
    // The `kubernetes` Service holds the first host; maybe one more is taken.
    let mut used = vec![cidr.first_host().to_string()];
    if g.chance(1, 2) {
        let taken = Ipv4Addr::from(u32::from(cidr.first_host()) + 2).to_string();
        let body = json!({"metadata": {"name": "taken"}, "spec": {"clusterIP": taken, "ports": [{"port": 80}]}});
        let (s, v) = call(&mut api, "POST", SVCS, &body);
        if s != 201 {
            return Ok(Err(format!("setup: taken service refused: {v}")));
        }
        used.push(taken);
    }
    let (old, obj) = if g.chance(1, 3) {
        let body = json!({"metadata": {"name": "s"}, "spec": old_spec(&mut g, cidr)});
        let (s, stored) = call(&mut api, "POST", SVCS, &body);
        if s != 201 {
            return Ok(Err(format!("setup: old service refused: {stored}")));
        }
        let new = mutate(&mut g, &stored, cidr, &used);
        (Some(stored), new)
    } else {
        let spec = service_spec(&mut g, cidr, &used);
        let obj = json!({"apiVersion": "v1", "kind": "Service",
            "metadata": {"name": "s", "namespace": "default"}, "spec": spec});
        (None, obj)
    };
    let mut payload = json!({"object": obj, "cidr": cidr.to_string(), "used": used});
    if let Some(o) = &old {
        payload["old"] = o.clone();
    }
    let answer = oracle.call("service", payload)?;
    let (code, ours) = match &old {
        None => call(&mut api, "POST", SVCS, &obj),
        Some(_) => call(&mut api, "PUT", &format!("{SVCS}/s"), &obj),
    };
    let theirs = match answer {
        Ok(v) => v,
        // Upstream bug: an update making a Service headless and
        // selectorless under a dual-stack policy gets an IPv6 family on a
        // single-stack cluster; `updateClusterIPs` then allocates from the
        // missing IPv6 allocator and panics (the apiserver answers 500).
        // kubemu answers with validation's 422 (the IP may not change).
        Err(e)
            if e.starts_with("panic: runtime error: invalid memory address")
                && old.is_some()
                && obj["spec"]["clusterIP"] == "None"
                && code == 422 =>
        {
            return Ok(Ok(false));
        }
        Err(e) => {
            return Ok(Err(format!(
                "service: oracle error {e}\n  object: {obj}\n  old: {old:?}"
            )));
        }
    };
    let mut asked: Vec<String> = Vec::new();
    for v in [&obj, old.as_ref().unwrap_or(&Value::Null)] {
        asked.extend(v["spec"]["clusterIP"].as_str().map(str::to_owned));
        for x in v["spec"]["clusterIPs"].as_array().into_iter().flatten() {
            asked.extend(x.as_str().map(str::to_owned));
        }
    }
    let their_code = theirs["code"].as_u64().unwrap_or(0) as u16;
    let mismatch = |what: String| {
        Ok(Err(format!(
            "service {what}\n  object: {obj}\n  old:    {}\n  kubemu: {code} {ours}\n  oracle: {theirs}",
            old.as_ref().map_or("-".into(), Value::to_string)
        )))
    };
    if code != their_code {
        return mismatch(format!("status {code} != {their_code}"));
    }
    if !(200..300).contains(&code) {
        let mine = sorted_causes(&ours["details"]["causes"], &asked);
        let other = sorted_causes(&theirs["causes"], &asked);
        return if mine == other {
            Ok(Ok(false))
        } else {
            mismatch("causes differ".into())
        };
    }
    let mine = match ip_fields(&ours, &asked, cidr, &used) {
        Ok(v) => v,
        Err(e) => return mismatch(e),
    };
    let other = ip_fields(&theirs["object"], &asked, cidr, &used).unwrap_or(Value::Null);
    if mine == other {
        Ok(Ok(true))
    } else {
        mismatch(format!("IP fields differ: kubemu {mine}, oracle {other}"))
    }
}
