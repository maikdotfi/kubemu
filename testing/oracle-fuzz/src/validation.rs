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

/// A pick that is usually the first (valid) choice.
fn mostly<'a, T>(g: &mut Gen, items: &'a [T]) -> &'a T {
    if g.chance(3, 4) {
        &items[0]
    } else {
        g.pick(items)
    }
}

const QUANTITIES: &[&str] = &["100m", "1", "2", "500Mi", "1Gi", "-1"];

fn resources(g: &mut Gen) -> Value {
    let mut r = Map::new();
    for list in ["limits", "requests"] {
        if g.chance(1, 2) {
            let mut m = Map::new();
            for k in ["cpu", "memory"] {
                if g.chance(1, 2) {
                    m.insert(k.into(), json!(*mostly(g, QUANTITIES)));
                }
            }
            if !m.is_empty() {
                r.insert(list.into(), Value::Object(m));
            }
        }
    }
    Value::Object(r)
}

/// A pod that is valid unless the dice say otherwise, touching only
/// ported rules (`kubemu_validation::validation`).
fn pod(g: &mut Gen) -> Value {
    let mut volumes = Vec::new();
    let mut vol_names = Vec::new();
    for _ in 0..g.below(3) {
        let name = *mostly(g, &["data", "cache", "Bad_Name", "", "data"]);
        let mut v = json!({"name": name});
        match g.below(8) {
            0 => v["configMap"] = json!({"name": "cm"}),
            1 => v["secret"] = json!({"secretName": "s"}),
            2 => {
                v["emptyDir"] = json!({});
                v["configMap"] = json!({"name": "cm"});
            }
            3 => {}
            _ => v["emptyDir"] = json!({}),
        }
        vol_names.push(name);
        volumes.push(v);
    }
    let mut containers = Vec::new();
    let n = if g.chance(1, 20) { 0 } else { 1 + g.below(3) };
    for i in 0..n {
        let mut c = Map::new();
        let names: &[&str] = if i == 0 {
            &["app", "", "App", "a_b"]
        } else {
            &["sidecar", "app", "", "x-1"]
        };
        c.insert("name".into(), json!(*mostly(g, names)));
        let image = *mostly(g, &["nginx", "", "busybox:1.36"]);
        if !image.is_empty() || g.chance(1, 2) {
            c.insert("image".into(), json!(image));
        }
        if g.chance(1, 6) {
            c.insert(
                "imagePullPolicy".into(),
                json!(*g.pick(&["Never", "Sometimes", "Always"])),
            );
        }
        if g.chance(1, 10) {
            c.insert(
                "terminationMessagePolicy".into(),
                json!(*g.pick(&["FallbackToLogsOnError", "Bogus"])),
            );
        }
        let mut ports = Vec::new();
        for j in 0..g.below(3) {
            let mut p = Map::new();
            p.insert(
                "containerPort".into(),
                json!(*mostly(g, &[80 + j as i64, 0, 70000, -1])),
            );
            if i == 0 && j == 0 && g.chance(1, 5) {
                p.insert("hostPort".into(), json!(*g.pick(&[8080, 99999])));
            }
            if g.chance(1, 3) {
                p.insert(
                    "name".into(),
                    json!(*mostly(
                        g,
                        &[
                            "http",
                            "HTTP",
                            "a--b",
                            "-x",
                            "toolongportname1",
                            "8080",
                            "web"
                        ]
                    )),
                );
            }
            if g.chance(1, 5) {
                p.insert("protocol".into(), json!(*g.pick(&["UDP", "SCTP", "HTTP"])));
            }
            ports.push(Value::Object(p));
        }
        if !ports.is_empty() {
            c.insert("ports".into(), Value::Array(ports));
        }
        let env: Vec<Value> = (0..g.below(3))
            .map(|_| json!({"name": *mostly(g, &["FOO", "", "A=B", "ok.name-1", "é"]), "value": "v"}))
            .collect();
        if !env.is_empty() {
            c.insert("env".into(), Value::Array(env));
        }
        let mut mounts = Vec::new();
        for _ in 0..g.below(3) {
            let name = if !vol_names.is_empty() && g.chance(3, 4) {
                *g.pick(&vol_names)
            } else {
                *g.pick(&["missing", ""])
            };
            mounts.push(json!({"name": name, "mountPath": *mostly(g, &["/data", "", "/cache"])}));
        }
        if !mounts.is_empty() {
            c.insert("volumeMounts".into(), Value::Array(mounts));
        }
        if g.chance(1, 2) {
            c.insert("resources".into(), resources(g));
        }
        containers.push(Value::Object(c));
    }
    let mut spec = json!({"containers": containers});
    if !volumes.is_empty() {
        spec["volumes"] = Value::Array(volumes);
    }
    if g.chance(1, 6) {
        spec["restartPolicy"] = json!(*g.pick(&["Never", "OnFailure", "Sometimes"]));
    }
    if g.chance(1, 6) {
        spec["dnsPolicy"] = json!(*g.pick(&["Default", "ClusterFirstWithHostNet", "Bogus"]));
    }
    if g.chance(1, 8) {
        spec["nodeSelector"] = json!({*g.pick(&["disk", "bad key!", "example.com/zone"]): "ssd"});
    }
    if g.chance(1, 8) {
        spec[*g.pick(&["hostname", "subdomain"])] = json!(*g.pick(&["web", "Web_1"]));
    }
    json!({
        "apiVersion": "v1", "kind": "Pod",
        "metadata": {"name": *mostly(g, &["web", "P", "a_b"]), "namespace": "default",
                     "labels": {*mostly(g, &["app", "bad key!"]): "x"}},
        "spec": spec,
    })
}

/// A service, valid unless the dice say otherwise (ported rules only).
fn service(g: &mut Gen) -> Value {
    let mut spec = Map::new();
    let ty = *mostly(
        g,
        &[
            "",
            "NodePort",
            "LoadBalancer",
            "ExternalName",
            "ClusterIP",
            "Bogus",
        ],
    );
    if !ty.is_empty() {
        spec.insert("type".into(), json!(ty));
    }
    if ty == "ExternalName" {
        spec.insert(
            "externalName".into(),
            json!(*mostly(
                g,
                &["db.example.com", "", "Bad_Name", "db.example.com."]
            )),
        );
    }
    let mut ports = Vec::new();
    let n = if g.chance(1, 10) { 0 } else { 1 + g.below(3) };
    for i in 0..n {
        let mut p = Map::new();
        let name = *mostly(
            g,
            &[
                ["http", "https", "metrics"][i as usize],
                "",
                "HTTP",
                "http",
                "a.b",
            ],
        );
        if !name.is_empty() || g.chance(1, 2) {
            p.insert("name".into(), json!(name));
        }
        p.insert("port".into(), json!(*mostly(g, &[80 + i as i64, 0, 70000])));
        if g.chance(1, 5) {
            p.insert("protocol".into(), json!(*g.pick(&["UDP", "HTTP"])));
        }
        if g.chance(1, 3) {
            let tp = match g.below(7) {
                0 => json!(8080),
                1 => json!(0),
                2 => json!(70000),
                3 => json!("http"),
                4 => json!("HTTP"),
                5 => json!("--"),
                _ => json!("toolongportname1"),
            };
            p.insert("targetPort".into(), tp);
        }
        if g.chance(1, 10) {
            p.insert("nodePort".into(), json!(30080 + i as i64));
        }
        ports.push(Value::Object(p));
    }
    if !ports.is_empty() {
        spec.insert("ports".into(), Value::Array(ports));
    }
    if g.chance(1, 2) {
        spec.insert(
            "selector".into(),
            json!({*mostly(g, &["app", "bad key!"]): "x"}),
        );
    }
    if g.chance(1, 5) {
        spec.insert(
            "sessionAffinity".into(),
            json!(*g.pick(&["ClientIP", "Sticky", "None"])),
        );
    }
    json!({
        "apiVersion": "v1", "kind": "Service",
        "metadata": {"name": *mostly(g, &["svc", "1svc", "Svc", "a.b"]), "namespace": "default"},
        "spec": spec,
    })
}

fn sorted_causes(v: &Value) -> Vec<(String, String, String)> {
    let mut out: Vec<(String, String, String)> = v
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| {
            let s = |k: &str| c[k].as_str().unwrap_or("").to_owned();
            (s("reason"), s("field"), s("message"))
        })
        .collect();
    out.sort();
    out
}

/// One validation case: a pod or service through kubemu's defaulting and
/// validation, and through the oracle's `rest.BeforeCreate`. Causes are
/// compared as sets (Go iterates resource maps in random order),
/// messages included: kubectl prints them.
pub fn validate_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let (obj, plural) = if g.chance(1, 2) {
        (pod(&mut g), "pods")
    } else {
        (service(&mut g), "services")
    };
    let r = kubemu_api_types::resource("", "v1", plural).expect("served");
    let theirs = match oracle.call("validate", json!({"object": obj}))? {
        Ok(v) => sorted_causes(&v),
        Err(e) => return Ok(Err(format!("validate: oracle error {e}\n  object: {obj}"))),
    };
    let mut defaulted = obj.clone();
    kubemu_validation::default(r.group, r.kind, &mut defaulted);
    let ours: Vec<(String, String, String)> = {
        let mut v: Vec<_> = kubemu_validation::validate(r, &defaulted)
            .into_iter()
            .map(|c| (c.reason.to_owned(), c.field, c.message))
            .collect();
        v.sort();
        v
    };
    Ok(if ours == theirs {
        Ok(!ours.is_empty())
    } else {
        let only = |a: &[(String, String, String)], b: &[(String, String, String)]| {
            a.iter()
                .filter(|x| !b.contains(x))
                .map(|x| format!("\n      {x:?}"))
                .collect::<String>()
        };
        Err(format!(
            "validate {} mismatch\n  object: {obj}\n  only kubemu:{}\n  only oracle:{}",
            r.kind,
            only(&ours, &theirs),
            only(&theirs, &ours)
        ))
    })
}
