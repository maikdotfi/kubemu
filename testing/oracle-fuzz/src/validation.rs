//! Quantities, defaulting and validation (ADR 0013) against the oracle's
//! `quantity`, `default` and `validate` ops.

use kubemu_api_types::{Schema, Ty};
use serde_json::{Map, Value, json};

use crate::{CaseResult, Gen, Oracle, same_json};

/// A quantity as people write them, and as they mistype them.
fn quantity_text(g: &mut Gen) -> String {
    let mut s = String::new();
    if g.chance(1, 20) {
        s.push(' ');
    }
    match g.below(8) {
        0 => s.push('-'),
        1 => s.push('+'),
        _ => {}
    }
    let digits = |g: &mut Gen, max: u64| -> String {
        let n = g.below(max + 1);
        (0..n)
            .map(|i| {
                // Leading zeros and runs of zeros are interesting.
                if (i == 0 && g.chance(1, 6)) || g.chance(1, 3) {
                    '0'
                } else {
                    char::from(b'0' + g.below(10) as u8)
                }
            })
            .collect()
    };
    let long = g.chance(1, 8);
    s.push_str(&digits(g, if long { 24 } else { 5 }));
    if g.chance(1, 3) {
        s.push('.');
        s.push_str(&digits(g, if long { 14 } else { 4 }));
    }
    let suffix = match g.below(30) {
        0..=9 => "",
        10 => "n",
        11 => "u",
        12 | 13 => "m",
        14 => "k",
        15 => "M",
        16 => "G",
        17 => "T",
        18 => "P",
        19 => "E",
        20 | 21 => "Ki",
        22 | 23 => "Mi",
        24 => "Gi",
        25 => "Ti",
        26 => "Ei",
        27 => *g.pick(&["kk", "i", "x", "K", "mi", "e", "E-", "e+"]),
        _ => {
            let e = format!(
                "{}{}{}",
                g.pick(&["e", "E"]),
                g.pick(&["", "", "-", "+"]),
                g.below(25)
            );
            s.push_str(&e);
            ""
        }
    };
    s.push_str(suffix);
    if g.chance(1, 20) {
        s.push(' ');
    }
    s
}

/// One quantity case: parse, optionally round up to milli, serialize.
pub fn quantity_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let value = if g.chance(1, 10) {
        // Integers keep their JSON text in serde_json; floats may not.
        json!(g.below(5000) as i64 - 1000)
    } else {
        json!(quantity_text(&mut g))
    };
    let round_up = g.chance(1, 2);
    let ours = kubemu_validation::quantity::canonicalize_json(&value, round_up)
        .map(Value::String)
        .map_err(str::to_owned);
    let theirs = oracle.call("quantity", json!({"value": value, "roundUp": round_up}))?;
    // Error texts are upstream's constants: compare them too.
    Ok(if ours == theirs {
        Ok(ours.is_ok())
    } else {
        Err(format!(
            "quantity mismatch\n  value:  {value} roundUp={round_up}\n  kubemu: {ours:?}\n  oracle: {theirs:?}"
        ))
    })
}

/// Strings for any string field: enum values defaulting branches on,
/// images with and without tags, and the empty string.
const WORDS: &[&str] = &[
    "",
    "x",
    "None",
    "ClientIP",
    "ClusterIP",
    "NodePort",
    "LoadBalancer",
    "ExternalName",
    "TCP",
    "UDP",
    "Always",
    "nginx",
    "nginx:latest",
    "nginx:1.25",
    "localhost:5000/app",
    "Nginx",
    "nginx@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "10.0.0.1",
    "Opaque",
    "a___b",
    "a__b:v1",
    "Example.com/app",
    "example.com:5000/team/app:latest",
    "[::1]:5000/app",
    "app:TAG",
    "app:",
    "nginx:1@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "nginx@sha512:aaaa",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
];

/// Fields defaulting reads or writes: visited more often.
const HOT: &[&str] = &[
    "spec",
    "status",
    "metadata",
    "name",
    "containers",
    "initContainers",
    "ephemeralContainers",
    "ports",
    "protocol",
    "volumes",
    "livenessProbe",
    "readinessProbe",
    "startupProbe",
    "httpGet",
    "grpc",
    "lifecycle",
    "postStart",
    "preStop",
    "resources",
    "limits",
    "requests",
    "env",
    "valueFrom",
    "fieldRef",
    "fileKeyRef",
    "type",
    "sessionAffinity",
    "sessionAffinityConfig",
    "clientIP",
    "targetPort",
    "port",
    "externalIPs",
    "image",
    "imagePullPolicy",
    "hostNetwork",
    "containerPort",
    "hostPort",
    "secret",
    "configMap",
    "downwardAPI",
    "projected",
    "sources",
    "serviceAccountToken",
    "hostPath",
    "items",
    "ephemeral",
    "volumeClaimTemplate",
    "subsets",
    "capacity",
    "allocatable",
    "podIP",
    "podIPs",
    "ip",
    "loadBalancer",
    "ingress",
    "overhead",
    "containerStatuses",
    "allocatedResources",
    "serviceAccountName",
    "serviceAccount",
    "terminationGracePeriodSeconds",
    "enableServiceLinks",
    "azureDisk",
    "rbd",
    "iscsi",
    "scaleIO",
    "data",
    "labels",
    "reference",
    "pullPolicy",
];

/// A well-typed value of `schema` (the oracle's typed decode must accept
/// it), with scalars from small sets so defaulting branches are hit.
fn typed(g: &mut Gen, schema: Schema, depth: u32) -> Option<Value> {
    Some(match schema.ty() {
        Ty::Object { .. } => {
            if depth == 0 {
                return None;
            }
            let mut m = Map::new();
            for (name, s) in schema.fields() {
                let p = if HOT.contains(&name) { 3 } else { 1 };
                if name == "apiVersion" || name == "kind" || !g.chance(p, 12) {
                    continue;
                }
                if let Some(v) = typed(g, s, depth - 1) {
                    m.insert(name.to_owned(), v);
                }
            }
            Value::Object(m)
        }
        Ty::Map(_) => {
            let values = schema.values().expect("maps have values");
            let mut m = Map::new();
            for _ in 0..g.below(3) {
                let k = (*g.pick(&["cpu", "memory", "a", "b"])).to_owned();
                if let Some(v) = typed(g, values, depth.saturating_sub(1)) {
                    m.insert(k, v);
                }
            }
            // Go's nil and empty maps differ to defaulting (`Allocatable ==
            // nil`) but both vanish from the baseline (omitempty).
            if m.is_empty() {
                return None;
            }
            Value::Object(m)
        }
        Ty::Array(_) => {
            if depth == 0 {
                return None;
            }
            let items = schema.items().expect("arrays have items");
            Value::Array(
                (0..g.below(3))
                    .filter_map(|_| typed(g, items, depth - 1))
                    .collect(),
            )
        }
        Ty::String => match schema.format() {
            Some("byte") => json!("aGk="),
            Some("date-time") => json!("2026-01-01T00:00:00Z"),
            _ => json!(*g.pick(WORDS)),
        },
        Ty::Integer => json!(*g.pick(&[0, 0, 1, 80, -5, 8080])),
        Ty::Number => json!(0.5),
        Ty::Boolean => json!(g.chance(1, 2)),
        Ty::IntOrString => {
            if g.chance(1, 2) {
                json!(*g.pick(&[0, 80, 8080]))
            } else {
                json!(*g.pick(&["", "http", "8080"]))
            }
        }
        Ty::Quantity => json!(*g.pick(&["1", "100m", "1n", "1.5Gi", "1500u", "+5", "2000k"])),
        Ty::Any | Ty::Ref(_) => return None,
    })
}

/// Kinds kubemu stores, weighted to those with defaults.
fn stored_kind(g: &mut Gen) -> &'static kubemu_api_types::ResourceInfo {
    let (group, plural) = *g.pick(&[
        ("", "pods"),
        ("", "pods"),
        ("", "pods"),
        ("", "services"),
        ("", "services"),
        ("", "namespaces"),
        ("", "secrets"),
        ("", "endpoints"),
        ("", "nodes"),
        ("", "configmaps"),
        ("", "serviceaccounts"),
        ("", "events"),
        ("discovery.k8s.io", "endpointslices"),
    ]);
    kubemu_api_types::resource(group, "v1", plural).expect("served")
}

/// One defaulting case. The oracle answers with its typed round trip of
/// the input (`baseline`) and the defaulted object; kubemu's defaulting
/// of the baseline must equal the latter. The round trip itself (Go's
/// omitempty, zero values) is decoding, a separate concern.
pub fn default_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let r = stored_kind(&mut g);
    let mut obj = typed(&mut g, r.schema(), 7).unwrap_or_else(|| json!({}));
    obj["apiVersion"] = json!(r.api_version());
    obj["kind"] = json!(r.kind);
    if g.chance(3, 4) {
        obj["metadata"]["name"] = json!(*g.pick(&["a", "b-1"]));
    }
    let theirs = match oracle.call("default", json!({"object": obj}))? {
        Ok(v) => v,
        // The typed decode refused the input: nothing to default.
        Err(_) => return Ok(Ok(false)),
    };
    let mut ours = theirs["baseline"].clone();
    kubemu_validation::default(r.group, r.kind, &mut ours);
    Ok(if same_json(&ours, &theirs["defaulted"]) {
        Ok(true)
    } else {
        Err(format!(
            "default {} mismatch\n  baseline: {}\n  kubemu:   {ours}\n  oracle:   {}",
            r.kind, theirs["baseline"], theirs["defaulted"]
        ))
    })
}

/// One registry strategy question, per stored kind in turn: kubemu's
/// answers were guesses until checked here (README §12.2).
pub fn strategy_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let stored: Vec<_> = kubemu_api_types::RESOURCES
        .iter()
        .filter(|r| r.group != "authorization.k8s.io")
        .collect();
    let r = stored[(seed % stored.len() as u64) as usize];
    let theirs = match oracle.call("strategy", json!({"group": r.group, "kind": r.kind}))? {
        Ok(v) => v,
        Err(e) => return Ok(Err(format!("strategy {}: {e}", r.kind))),
    };
    let ours = json!({
        "namespaceScoped": r.namespaced,
        "allowCreateOnUpdate": kubemu_api::strategy::allow_create_on_update(r),
        "allowUnconditionalUpdate": true,
    });
    Ok(if ours == theirs {
        Ok(true)
    } else {
        Err(format!(
            "strategy {} mismatch\n  kubemu: {ours}\n  oracle: {theirs}",
            r.kind
        ))
    })
}
