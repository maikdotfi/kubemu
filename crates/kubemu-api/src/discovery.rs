//! Discovery and OpenAPI, served from the resource table (ADR 0012).
//! Only served resources appear.

use std::collections::BTreeMap;

use kubemu_api_types::{KUBERNETES_VERSION, OPENAPI_V2, OPENAPI_V3, RESOURCES, ResourceInfo};
use serde_json::{Value, json};

use crate::http::{AGGREGATED_DISCOVERY_CONTENT_TYPE, Body, HttpResponse};
use crate::status::{ApiError, ApiResult};

/// Served groups (non-core) and their versions, in table order.
fn groups() -> BTreeMap<&'static str, Vec<&'static str>> {
    let mut g: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
    for r in RESOURCES.iter().filter(|r| !r.group.is_empty()) {
        let vs = g.entry(r.group).or_default();
        if !vs.contains(&r.version) {
            vs.push(r.version);
        }
    }
    g
}

fn group_json(name: &str, versions: &[&str]) -> Value {
    let vs: Vec<Value> = versions
        .iter()
        .map(|v| json!({"groupVersion": format!("{name}/{v}"), "version": v}))
        .collect();
    json!({
        "name": name,
        "versions": vs,
        "preferredVersion": vs[0],
    })
}

/// `GET /api` (legacy).
pub fn api_versions() -> Value {
    json!({"kind": "APIVersions", "versions": ["v1"], "serverAddressByClientCIDRs": []})
}

/// `GET /apis` (legacy).
pub fn api_group_list() -> Value {
    let gs: Vec<Value> = groups().iter().map(|(n, vs)| group_json(n, vs)).collect();
    json!({"kind": "APIGroupList", "apiVersion": "v1", "groups": gs})
}

/// `GET /apis/<group>`.
pub fn api_group(group: &str) -> ApiResult<Value> {
    let gs = groups();
    let vs = gs
        .get(group)
        .ok_or_else(|| ApiError::not_found_path(format!("get /apis/{group}")))?;
    let mut g = group_json(group, vs);
    g["kind"] = json!("APIGroup");
    g["apiVersion"] = json!("v1");
    Ok(g)
}

/// `GET /api/v1`, `GET /apis/<group>/<version>`.
pub fn api_resource_list(group: &str, version: &str) -> ApiResult<Value> {
    let rs: Vec<&ResourceInfo> = RESOURCES
        .iter()
        .filter(|r| r.group == group && r.version == version)
        .collect();
    if rs.is_empty() {
        return Err(ApiError::not_found_path(format!("get {group}/{version}")));
    }
    let mut out = Vec::new();
    for r in rs {
        let mut e = json!({
            "name": r.plural,
            "singularName": r.singular,
            "namespaced": r.namespaced,
            "kind": r.kind,
            "verbs": r.verbs,
        });
        if !r.short_names.is_empty() {
            e["shortNames"] = json!(r.short_names);
        }
        if !r.categories.is_empty() {
            e["categories"] = json!(r.categories);
        }
        out.push(e);
        for s in r.subresources {
            out.push(json!({
                "name": format!("{}/{}", r.plural, s.name),
                "singularName": "",
                "namespaced": r.namespaced,
                "kind": r.kind,
                "verbs": s.verbs,
            }));
        }
    }
    let gv = if group.is_empty() {
        version.to_owned()
    } else {
        format!("{group}/{version}")
    };
    Ok(json!({"kind": "APIResourceList", "apiVersion": "v1", "groupVersion": gv, "resources": out}))
}

fn aggregated_resource(r: &ResourceInfo) -> Value {
    let kind = json!({"group": r.group, "version": r.version, "kind": r.kind});
    let mut e = json!({
        "resource": r.plural,
        "responseKind": kind,
        "scope": if r.namespaced { "Namespaced" } else { "Cluster" },
        "singularResource": r.singular,
        "verbs": r.verbs,
    });
    if !r.short_names.is_empty() {
        e["shortNames"] = json!(r.short_names);
    }
    if !r.categories.is_empty() {
        e["categories"] = json!(r.categories);
    }
    if !r.subresources.is_empty() {
        e["subresources"] = r
            .subresources
            .iter()
            .map(|s| json!({"subresource": s.name, "responseKind": kind, "verbs": s.verbs}))
            .collect();
    }
    e
}

/// Aggregated discovery (`apidiscovery.k8s.io/v2`) for `/api` (`core`)
/// or `/apis`.
pub fn aggregated(core: bool) -> HttpResponse {
    let mut by_group: BTreeMap<&str, BTreeMap<&str, Vec<Value>>> = BTreeMap::new();
    for r in RESOURCES.iter().filter(|r| r.group.is_empty() == core) {
        by_group
            .entry(r.group)
            .or_default()
            .entry(r.version)
            .or_default()
            .push(aggregated_resource(r));
    }
    let items: Vec<Value> = by_group
        .into_iter()
        .map(|(g, vs)| {
            let versions: Vec<Value> = vs
                .into_iter()
                .map(|(v, rs)| json!({"version": v, "resources": rs, "freshness": "Current"}))
                .collect();
            json!({"metadata": {"name": g}, "versions": versions})
        })
        .collect();
    let mut resp = HttpResponse::json(
        200,
        json!({
            "kind": "APIGroupDiscoveryList",
            "apiVersion": "apidiscovery.k8s.io/v2",
            "metadata": {},
            "items": items,
        }),
    );
    resp.content_type = AGGREGATED_DISCOVERY_CONTENT_TYPE.into();
    resp
}

/// `GET /version`.
pub fn version() -> Value {
    let minor = KUBERNETES_VERSION
        .trim_start_matches('v')
        .split('.')
        .nth(1)
        .unwrap_or("0");
    json!({
        "major": "1",
        "minor": minor,
        "emulationMajor": "1",
        "emulationMinor": minor,
        "gitVersion": format!("{KUBERNETES_VERSION}+kubemu"),
        "gitCommit": "",
        "gitTreeState": "clean",
        "buildDate": "1970-01-01T00:00:00Z",
        "goVersion": "",
        "compiler": "kubemu",
        "platform": "linux/amd64",
    })
}

pub fn openapi_v2() -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "application/json".into(),
        body: Body::Static(OPENAPI_V2),
        warnings: Vec::new(),
    }
}

/// `GET /openapi/v3` (the index) or `GET /openapi/v3/<path>`.
pub fn openapi_v3(path: Option<&str>) -> ApiResult<HttpResponse> {
    match path {
        None => {
            let paths: serde_json::Map<String, Value> = OPENAPI_V3
                .iter()
                .map(|d| {
                    (
                        d.path.to_owned(),
                        json!({"serverRelativeURL": format!("/openapi/v3/{}?hash={}", d.path, d.hash)}),
                    )
                })
                .collect();
            Ok(HttpResponse::json(200, json!({"paths": paths})))
        }
        Some(p) => {
            let d = OPENAPI_V3
                .iter()
                .find(|d| d.path == p)
                .ok_or_else(|| ApiError::not_found_path(format!("get /openapi/v3/{p}")))?;
            Ok(HttpResponse {
                status: 200,
                content_type: "application/json".into(),
                body: Body::Static(d.document),
                warnings: Vec::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_documents() {
        let core = api_resource_list("", "v1").unwrap();
        let names: Vec<&str> = core["resources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"pods") && names.contains(&"pods/status"));
        assert!(!names.contains(&"deployments"));
        let groups = api_group_list();
        let names: Vec<&str> = groups["groups"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| g["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["authorization.k8s.io", "discovery.k8s.io"]);
        assert!(api_group("apps").is_err());
        assert!(api_resource_list("apps", "v1").is_err());
    }

    #[test]
    fn aggregated_documents() {
        let core = aggregated(true).value().unwrap();
        assert_eq!(core["items"].as_array().unwrap().len(), 1);
        assert_eq!(core["items"][0]["metadata"]["name"], "");
        let pods = core["items"][0]["versions"][0]["resources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["resource"] == "pods")
            .unwrap()
            .clone();
        assert_eq!(pods["scope"], "Namespaced");
        assert_eq!(pods["shortNames"], json!(["po"]));
        assert_eq!(pods["subresources"][0]["subresource"], "status");
        let apis = aggregated(false).value().unwrap();
        assert!(
            apis["items"]
                .as_array()
                .unwrap()
                .iter()
                .any(|g| g["metadata"]["name"] == "discovery.k8s.io")
        );
    }

    #[test]
    fn openapi() {
        let idx = openapi_v3(None).unwrap().value().unwrap();
        let url = idx["paths"]["api/v1"]["serverRelativeURL"]
            .as_str()
            .unwrap();
        assert!(url.starts_with("/openapi/v3/api/v1?hash="));
        assert!(openapi_v3(Some("api/v1")).unwrap().value().unwrap()["components"].is_object());
        assert!(openapi_v3(Some("apis/apps/v1")).is_err());
        assert_eq!(version()["minor"], "37");
    }
}
