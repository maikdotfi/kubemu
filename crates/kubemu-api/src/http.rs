//! The HTTP-shaped request and reply the frontend speaks (ADR 0011). The
//! tokio shell only moves bytes: it fills an [`HttpRequest`] from the wire
//! and writes the [`Reply`] back. Path parsing and content negotiation live
//! here, in the core, so DST covers them.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::status::{ApiError, ApiResult};
use crate::watch::WatchCursor;

#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    /// `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.
    pub method: String,
    /// The URL path, percent-decoded segments not yet split.
    pub path: String,
    /// The raw query string, without `?`.
    pub query: String,
    pub content_type: Option<String>,
    pub accept: Option<String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new(method: &str, path_and_query: &str) -> Self {
        let (path, query) = path_and_query
            .split_once('?')
            .unwrap_or((path_and_query, ""));
        Self {
            method: method.to_owned(),
            path: path.to_owned(),
            query: query.to_owned(),
            ..Self::default()
        }
    }

    pub fn json(mut self, body: &Value) -> Self {
        self.body = body.to_string().into_bytes();
        self.content_type = Some("application/json".into());
        self
    }

    pub fn with_content_type(mut self, ct: &str) -> Self {
        self.content_type = Some(ct.to_owned());
        self
    }

    pub fn with_accept(mut self, accept: &str) -> Self {
        self.accept = Some(accept.to_owned());
        self
    }
}

/// A response body.
#[derive(Clone, Debug)]
pub enum Body {
    Json(Value),
    /// Pre-rendered JSON (embedded OpenAPI documents).
    Static(&'static str),
    Text(String),
    /// Pre-rendered binary (the protobuf OpenAPI v2, ADR 0014).
    Bytes(&'static [u8]),
}

impl Body {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Json(v) => v.to_string().into_bytes(),
            Self::Static(s) => s.as_bytes().to_vec(),
            Self::Text(s) => s.as_bytes().to_vec(),
            Self::Bytes(b) => b.to_vec(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub content_type: String,
    pub body: Body,
    /// `Warning` headers (`299 - "..."`), e.g. for dropped unknown fields.
    pub warnings: Vec<String>,
}

impl HttpResponse {
    pub fn json(status: u16, v: Value) -> Self {
        Self {
            status,
            content_type: "application/json".into(),
            body: Body::Json(v),
            warnings: Vec::new(),
        }
    }

    pub fn error(e: &ApiError) -> Self {
        Self::json(e.code, e.to_status())
    }

    /// The JSON body, if any (tests and the sim read it).
    pub fn value(&self) -> Option<Value> {
        match &self.body {
            Body::Json(v) => Some(v.clone()),
            Body::Static(s) => serde_json::from_str(s).ok(),
            Body::Text(_) | Body::Bytes(_) => None,
        }
    }
}

/// What the frontend answers: a complete response, or the start of a
/// watch stream the shell then drains with [`crate::Api::watch_poll`].
#[derive(Debug)]
pub enum Reply {
    Response(HttpResponse),
    Watch(WatchCursor),
}

impl Reply {
    pub fn status(&self) -> u16 {
        match self {
            Self::Response(r) => r.status,
            Self::Watch(_) => 200,
        }
    }

    /// The response, panicking on a watch. For tests and the sim.
    pub fn response(self) -> HttpResponse {
        match self {
            Self::Response(r) => r,
            Self::Watch(_) => panic!("expected a response, got a watch"),
        }
    }
}

/// Query parameters. The last value of a repeated key wins, except
/// through [`Query::all`].
#[derive(Clone, Debug, Default)]
pub struct Query(BTreeMap<String, Vec<String>>);

impl Query {
    pub fn parse(q: &str) -> Self {
        let mut m: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for pair in q.split('&').filter(|p| !p.is_empty()) {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            m.entry(percent_decode(k))
                .or_default()
                .push(percent_decode(v));
        }
        Self(m)
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        self.0.get(k).and_then(|v| v.last()).map(String::as_str)
    }

    pub fn all(&self, k: &str) -> &[String] {
        self.0.get(k).map_or(&[], Vec::as_slice)
    }

    pub fn bool(&self, k: &str) -> ApiResult<bool> {
        match self.get(k) {
            None | Some("" | "false" | "0" | "False") => Ok(false),
            Some("true" | "1" | "True") => Ok(true),
            Some(other) => Err(ApiError::bad_request(format!(
                "invalid value for {k}: {other:?}"
            ))),
        }
    }

    pub fn int(&self, k: &str) -> ApiResult<Option<i64>> {
        match self.get(k) {
            None | Some("") => Ok(None),
            Some(v) => v
                .parse()
                .map(Some)
                .map_err(|_| ApiError::bad_request(format!("invalid value for {k}: {v:?}"))),
        }
    }
}

/// `%XX` and `+` decoding (query strings use `+` for space).
pub fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    while i < b.len() {
        match b[i] {
            b'%' if i + 2 < b.len() => match (hex(b[i + 1]), hex(b[i + 2])) {
                (Some(h), Some(l)) => {
                    out.push(h << 4 | l);
                    i += 3;
                    continue;
                }
                _ => out.push(b'%'),
            },
            b'+' => out.push(b' '),
            c => out.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// What a request path addresses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// `/api`, `/apis`.
    RootDiscovery {
        core: bool,
    },
    /// `/apis/<group>`.
    GroupDiscovery {
        group: String,
    },
    /// `/api/v1`, `/apis/<group>/<version>`.
    ResourceList {
        group: String,
        version: String,
    },
    Version,
    Health,
    OpenApiV2,
    /// `/openapi/v3` (`None`) or one document.
    OpenApiV3(Option<String>),
    Resource(ResourcePath),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourcePath {
    pub group: String,
    pub version: String,
    pub resource: String,
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub subresource: Option<String>,
    /// Through the legacy `/watch/` prefix.
    pub legacy_watch: bool,
}

pub fn parse_path(path: &str) -> Option<Target> {
    let segs: Vec<String> = path
        .trim_end_matches('/')
        .split('/')
        .skip(1)
        .map(percent_decode)
        .collect();
    let s: Vec<&str> = segs.iter().map(String::as_str).collect();
    let (group, version, rest) = match s.as_slice() {
        ["api"] => return Some(Target::RootDiscovery { core: true }),
        ["apis"] => return Some(Target::RootDiscovery { core: false }),
        ["version"] => return Some(Target::Version),
        ["healthz" | "livez" | "readyz", ..] => return Some(Target::Health),
        ["openapi", "v2"] => return Some(Target::OpenApiV2),
        ["openapi", "v3"] => return Some(Target::OpenApiV3(None)),
        ["openapi", "v3", rest @ ..] => return Some(Target::OpenApiV3(Some(rest.join("/")))),
        ["apis", g] => {
            return Some(Target::GroupDiscovery {
                group: (*g).to_owned(),
            });
        }
        ["api", v] => {
            return Some(Target::ResourceList {
                group: String::new(),
                version: (*v).to_owned(),
            });
        }
        ["apis", g, v] => {
            return Some(Target::ResourceList {
                group: (*g).to_owned(),
                version: (*v).to_owned(),
            });
        }
        ["api", v, rest @ ..] => ("", *v, rest),
        ["apis", g, v, rest @ ..] => (*g, *v, rest),
        _ => return None,
    };
    let (legacy_watch, rest) = match rest {
        ["watch", rest @ ..] => (true, rest),
        _ => (false, rest),
    };
    let mut p = ResourcePath {
        group: group.to_owned(),
        version: version.to_owned(),
        legacy_watch,
        ..ResourcePath::default()
    };
    let own = |s: &&str| Some((*s).to_owned());
    match rest {
        // Namespaces themselves, and their subresources.
        ["namespaces"] => p.resource = "namespaces".into(),
        ["namespaces", n] => {
            p.resource = "namespaces".into();
            p.name = own(n);
        }
        ["namespaces", n, sub @ ("status" | "finalize")] if group.is_empty() => {
            p.resource = "namespaces".into();
            p.name = own(n);
            p.subresource = own(sub);
        }
        ["namespaces", ns, r, tail @ ..] if tail.len() <= 2 => {
            p.namespace = own(ns);
            p.resource = (*r).to_owned();
            p.name = tail.first().and_then(own);
            p.subresource = tail.get(1).and_then(own);
        }
        [r, tail @ ..] if tail.len() <= 2 => {
            p.resource = (*r).to_owned();
            p.name = tail.first().and_then(own);
            p.subresource = tail.get(1).and_then(own);
        }
        _ => return None,
    }
    Some(Target::Resource(p))
}

/// Media types a request accepts, in order, without parameters other
/// than those named.
fn media_types(accept: Option<&str>) -> Vec<(String, BTreeMap<String, String>)> {
    accept
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|m| {
            let mut parts = m.split(';').map(str::trim);
            let ty = parts.next().unwrap_or("").to_ascii_lowercase();
            let params = parts
                .filter_map(|p| p.split_once('='))
                .map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned()))
                .collect();
            (ty, params)
        })
        .collect()
}

/// Whether plain JSON is acceptable. Requests for other encodings only
/// (protobuf, CBOR, YAML) get 406 (ADR 0010, 0014: request bodies may be
/// protobuf, responses are JSON); `as=Table` and other
/// transformations kubemu does not serve fall through to a plain-JSON
/// alternative if the client lists one.
pub fn accepts_json(accept: Option<&str>) -> ApiResult<()> {
    let types = media_types(accept);
    if types.is_empty() {
        return Ok(());
    }
    let ok = types.iter().any(|(t, p)| {
        matches!(t.as_str(), "application/json" | "*/*" | "application/*") && !p.contains_key("as")
    });
    if ok {
        Ok(())
    } else {
        Err(ApiError::not_acceptable(format!(
            "only the following media types are accepted: application/json (requested: {})",
            accept.unwrap_or("")
        )))
    }
}

/// The protobuf OpenAPI v2 media type kube-openapi answers with (gnostic
/// `openapi_v2.Document`). Clients ask for it as `...spec.v2@v1.0+protobuf`,
/// which Go's MIME parser cannot read back, so the answer uses the other.
pub const OPENAPI_V2_PROTOBUF: &str = "application/com.github.proto-openapi.spec.v2.v1.0+protobuf";

/// Whether the protobuf OpenAPI v2 is the client's first choice, by `q`
/// and then order, over JSON (kube-openapi's negotiation).
pub fn wants_openapi_v2_protobuf(accept: Option<&str>) -> bool {
    let mut best: Option<(f64, bool)> = None;
    for (t, p) in media_types(accept) {
        let pb = matches!(
            t.as_str(),
            "application/com.github.proto-openapi.spec.v2.v1.0+protobuf"
                | "application/com.github.proto-openapi.spec.v2@v1.0+protobuf"
        );
        if !pb && !matches!(t.as_str(), "application/json" | "application/*" | "*/*") {
            continue;
        }
        let q = p.get("q").and_then(|q| q.parse().ok()).unwrap_or(1.0);
        if q > 0.0 && best.is_none_or(|(bq, _)| q > bq) {
            best = Some((q, pb));
        }
    }
    best.is_some_and(|(_, pb)| pb)
}

/// Whether the client asks for aggregated discovery (`apidiscovery.k8s.io/v2`).
pub fn wants_aggregated_discovery(accept: Option<&str>) -> bool {
    media_types(accept).iter().any(|(t, p)| {
        t == "application/json"
            && p.get("g").map(String::as_str) == Some("apidiscovery.k8s.io")
            && p.get("v").map(String::as_str) == Some("v2")
            && p.get("as").map(String::as_str) == Some("APIGroupDiscoveryList")
    })
}

pub const AGGREGATED_DISCOVERY_CONTENT_TYPE: &str =
    "application/json;g=apidiscovery.k8s.io;v=v2;as=APIGroupDiscoveryList";

/// The body's media type, without parameters; JSON unless stated.
pub fn body_type(ct: Option<&str>) -> String {
    ct.unwrap_or("application/json")
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn res(path: &str) -> ResourcePath {
        match parse_path(path) {
            Some(Target::Resource(p)) => p,
            other => panic!("{path}: {other:?}"),
        }
    }

    #[test]
    fn paths() {
        let p = res("/api/v1/namespaces/default/pods/web/status");
        assert_eq!(
            (
                p.namespace.as_deref(),
                p.resource.as_str(),
                p.name.as_deref(),
                p.subresource.as_deref()
            ),
            (Some("default"), "pods", Some("web"), Some("status"))
        );
        let p = res("/api/v1/namespaces/default");
        assert_eq!(
            (p.resource.as_str(), p.name.as_deref(), p.namespace),
            ("namespaces", Some("default"), None)
        );
        let p = res("/api/v1/namespaces/kube-system/status");
        assert_eq!(p.subresource.as_deref(), Some("status"));
        let p = res("/api/v1/namespaces/kube-system/configmaps");
        assert_eq!(
            (p.namespace.as_deref(), p.name),
            (Some("kube-system"), None)
        );
        let p = res("/apis/discovery.k8s.io/v1/watch/endpointslices");
        assert!(p.legacy_watch && p.group == "discovery.k8s.io");
        let p = res("/api/v1/nodes/n1");
        assert_eq!(
            (p.resource.as_str(), p.name.as_deref()),
            ("nodes", Some("n1"))
        );
        assert_eq!(
            parse_path("/apis"),
            Some(Target::RootDiscovery { core: false })
        );
        assert_eq!(
            parse_path("/openapi/v3/api/v1"),
            Some(Target::OpenApiV3(Some("api/v1".into())))
        );
        assert_eq!(parse_path("/nope"), None);
    }

    #[test]
    fn queries() {
        let q = Query::parse("labelSelector=app%3Dweb%2Ctier+in+(a)&limit=5&watch=true&limit=7");
        assert_eq!(q.get("labelSelector"), Some("app=web,tier in (a)"));
        assert_eq!(q.int("limit").unwrap(), Some(7));
        assert!(q.bool("watch").unwrap());
        assert!(q.bool("x").is_ok());
        assert!(Query::parse("limit=x").int("limit").is_err());
    }

    #[test]
    fn negotiation() {
        assert!(accepts_json(None).is_ok());
        assert!(accepts_json(Some("application/json, */*")).is_ok());
        assert!(
            accepts_json(Some(
                "application/json;as=Table;v=v1;g=meta.k8s.io,application/json"
            ))
            .is_ok()
        );
        assert!(accepts_json(Some("application/vnd.kubernetes.protobuf")).is_err());
        let pb = "application/com.github.proto-openapi.spec.v2@v1.0+protobuf";
        assert!(wants_openapi_v2_protobuf(Some(pb)));
        assert!(wants_openapi_v2_protobuf(Some(&format!(
            "{pb}, application/json"
        ))));
        assert!(!wants_openapi_v2_protobuf(Some(&format!(
            "application/json, {pb}"
        ))));
        assert!(!wants_openapi_v2_protobuf(Some(&format!(
            "{pb}; q=0.5, application/json"
        ))));
        assert!(!wants_openapi_v2_protobuf(None));
        assert!(accepts_json(Some("application/json;as=Table;v=v1;g=meta.k8s.io")).is_err());
        assert!(wants_aggregated_discovery(Some(
            "application/json;g=apidiscovery.k8s.io;v=v2;as=APIGroupDiscoveryList,application/json"
        )));
        assert!(!wants_aggregated_discovery(Some("application/json")));
    }
}
