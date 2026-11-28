//! Reads: point-in-time views of a resource, list with selectors and
//! pagination, and the resourceVersion rules for both.
//!
//! The store keeps current objects plus a bounded event log. A list at an
//! older resourceVersion (`resourceVersionMatch=Exact`, or a `continue`
//! token) rolls the current state back through the log; once that history
//! is compacted away the answer is 410, as from etcd.

use std::collections::BTreeMap;

use kubemu_store::{ClusterStore, EventKind, ObjectKey, ResourceVersion, StoreError};
use serde_json::{Value, json};

use crate::http::Query;
use crate::object::{Obj, labels};
use crate::selector::{FieldSelector, LabelSelector};
use crate::status::{ApiError, ApiResult};

/// A resource's objects (optionally in one namespace) at `rv`.
pub struct Snapshot {
    pub rv: ResourceVersion,
    pub objects: BTreeMap<ObjectKey, Obj>,
}

fn too_old(rv: ResourceVersion, oldest: ResourceVersion) -> ApiError {
    ApiError::gone(format!("too old resource version: {rv} ({oldest})"))
}

/// The objects of `resource` in `namespace` (all if `None`) at `at`, or
/// now. `at` newer than the store is a 504 the client retries.
pub fn snapshot(
    cs: &ClusterStore<Obj>,
    resource: &str,
    namespace: Option<&str>,
    at: Option<ResourceVersion>,
) -> ApiResult<Snapshot> {
    let current = cs.resource_version();
    let mut objects: BTreeMap<ObjectKey, Obj> = cs
        .list(resource, namespace)
        .0
        .into_iter()
        .map(|(k, o)| (k.clone(), o.value.clone()))
        .collect();
    let Some(at) = at.filter(|&at| at != current) else {
        return Ok(Snapshot {
            rv: current,
            objects,
        });
    };
    if at > current {
        return Err(ApiError::too_large_resource_version(at, current));
    }
    let events = cs.events_since(at).map_err(|e| match e {
        StoreError::Gone { requested, oldest } => too_old(requested, oldest),
        other => ApiError::internal(other.to_string()),
    })?;
    for e in events.iter().rev() {
        if e.key.resource != resource || namespace.is_some_and(|ns| e.key.namespace != ns) {
            continue;
        }
        match e.kind {
            EventKind::Added => {
                objects.remove(&e.key);
            }
            EventKind::Modified => {
                let prev = e.prev.clone().expect("Modified events carry prev");
                objects.insert(e.key.clone(), prev);
            }
            EventKind::Deleted => {
                objects.insert(e.key.clone(), e.value.clone());
            }
        }
    }
    Ok(Snapshot { rv: at, objects })
}

/// `resourceVersion` as a number, or `None` for "" (most recent).
pub fn parse_rv(s: Option<&str>) -> ApiResult<Option<ResourceVersion>> {
    match s {
        None | Some("") => Ok(None),
        Some(v) => v
            .parse()
            .map(Some)
            .map_err(|_| ApiError::bad_request(format!("invalid resource version {v:?}"))),
    }
}

/// The selectors a list or watch filters by.
#[derive(Clone, Debug, Default)]
pub struct Filter {
    pub labels: LabelSelector,
    pub fields: FieldSelector,
}

impl Filter {
    pub fn parse(q: &Query, group: &str, resource: &str) -> ApiResult<Self> {
        Ok(Self {
            labels: LabelSelector::parse(q.get("labelSelector").unwrap_or(""))?,
            fields: FieldSelector::parse(q.get("fieldSelector").unwrap_or(""), group, resource)?,
        })
    }

    pub fn matches(&self, v: &Value) -> bool {
        self.labels.matches(labels(v)) && self.fields.matches(v)
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty() && self.fields.is_empty()
    }
}

/// A decoded `continue` token: the list's resourceVersion and the last
/// key returned.
#[derive(Debug, PartialEq, Eq)]
pub struct Continue {
    pub rv: ResourceVersion,
    pub after: (String, String),
}

impl Continue {
    /// Shaped like the apiserver's: base64url JSON, opaque to clients.
    pub fn encode(&self) -> String {
        let start = if self.after.0.is_empty() {
            format!("{}\u{0}", self.after.1)
        } else {
            format!("{}/{}\u{0}", self.after.0, self.after.1)
        };
        let j = json!({"v": "meta.k8s.io/v1", "rv": self.rv, "start": start});
        base64url(j.to_string().as_bytes())
    }

    pub fn decode(token: &str) -> ApiResult<Self> {
        let bad = || {
            ApiError::bad_request(
                "continue key is not valid: incorrect encoded start resourceVersion (version meta.k8s.io/v1)",
            )
        };
        let bytes = unbase64url(token).ok_or_else(bad)?;
        let j: Value = serde_json::from_slice(&bytes).map_err(|_| bad())?;
        if j["v"] != "meta.k8s.io/v1" {
            return Err(bad());
        }
        let rv = j["rv"].as_u64().filter(|&rv| rv > 0).ok_or_else(bad)?;
        let start = j["start"].as_str().ok_or_else(bad)?;
        let start = start.strip_suffix('\u{0}').ok_or_else(bad)?;
        let after = match start.split_once('/') {
            Some((ns, n)) => (ns.to_owned(), n.to_owned()),
            None => (String::new(), start.to_owned()),
        };
        Ok(Self { rv, after })
    }
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64url(data: &[u8]) -> String {
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
        for i in 0..=chunk.len() {
            out.push(B64[(n >> (18 - 6 * i) & 63) as usize] as char);
        }
    }
    out
}

fn unbase64url(s: &str) -> Option<Vec<u8>> {
    let s = s.trim_end_matches('=');
    let mut out = Vec::new();
    let (mut acc, mut bits) = (0u32, 0);
    for c in s.bytes() {
        let v = B64.iter().position(|&b| b == c)? as u32;
        acc = acc << 6 | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

/// One page of a list.
pub struct Page {
    pub rv: ResourceVersion,
    pub items: Vec<Obj>,
    pub cont: Option<String>,
    pub remaining: Option<u64>,
}

/// List with the apiserver's resourceVersion rules:
/// - `resourceVersion` unset or `""`: the most recent state;
/// - `"0"`: any state (we serve the most recent);
/// - `N` with `resourceVersionMatch` `NotOlderThan` (the default): the most
///   recent, or 504 if `N` is in the future;
/// - `N` with `Exact`: the state at `N`, or 410 if compacted;
/// - `continue`: the next page of the list it came from, at its version.
pub fn list(
    cs: &ClusterStore<Obj>,
    resource: &str,
    namespace: Option<&str>,
    q: &Query,
    filter: &Filter,
) -> ApiResult<Page> {
    let rv = q.get("resourceVersion");
    let rv_match = q.get("resourceVersionMatch").unwrap_or("");
    let cont = q.get("continue").filter(|c| !c.is_empty());
    let limit = q.int("limit")?.filter(|&l| l > 0);

    if cont.is_some() && rv.is_some_and(|r| !r.is_empty() && r != "0") {
        return Err(ApiError::bad_request(
            "specifying resource version is not allowed when using continue",
        ));
    }
    if !rv_match.is_empty() && rv.unwrap_or("").is_empty() {
        return Err(ApiError::bad_request(
            "resourceVersionMatch is forbidden unless resourceVersion is provided",
        ));
    }
    let at = match (parse_rv(rv)?, rv_match) {
        (_, m) if !matches!(m, "" | "Exact" | "NotOlderThan") => {
            return Err(ApiError::bad_request(format!(
                "resourceVersionMatch: Unsupported value: {m:?}"
            )));
        }
        (Some(0), "Exact") => {
            return Err(ApiError::bad_request(
                "resourceVersionMatch \"exact\" is forbidden for resourceVersion \"0\"",
            ));
        }
        (Some(n), "Exact") => Some(n),
        (Some(n), _) if n > cs.resource_version() => {
            return Err(ApiError::too_large_resource_version(
                n,
                cs.resource_version(),
            ));
        }
        _ => None,
    };
    let (at, after) = match cont.map(Continue::decode).transpose()? {
        Some(c) => (Some(c.rv), Some(c.after)),
        None => (at, None),
    };
    let snap = snapshot(cs, resource, namespace, at).map_err(|e| {
        if after.is_some() && e.code == 410 {
            ApiError::gone("The provided continue parameter is too old to display a consistent list result. You can start a new list without the continue parameter.")
        } else {
            e
        }
    })?;

    let matching: Vec<(&ObjectKey, &Obj)> = snap
        .objects
        .iter()
        .filter(|(k, _)| {
            after.as_ref().is_none_or(|(ns, n)| {
                (k.namespace.as_str(), k.name.as_str()) > (ns.as_str(), n.as_str())
            })
        })
        .filter(|(_, v)| filter.matches(v))
        .collect();
    let take = limit.map_or(matching.len(), |l| (l as usize).min(matching.len()));
    let more = take < matching.len();
    let cont = more.then(|| {
        let (k, _) = matching[take - 1];
        Continue {
            rv: snap.rv,
            after: (k.namespace.clone(), k.name.clone()),
        }
        .encode()
    });
    let remaining = (more && filter.is_empty()).then(|| (matching.len() - take) as u64);
    Ok(Page {
        rv: snap.rv,
        items: matching[..take].iter().map(|(_, v)| (*v).clone()).collect(),
        cont,
        remaining,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue_tokens_round_trip() {
        for after in [
            ("ns".to_owned(), "a".to_owned()),
            (String::new(), "node-1".to_owned()),
        ] {
            let c = Continue { rv: 42, after };
            assert_eq!(Continue::decode(&c.encode()).unwrap(), c);
        }
        assert!(Continue::decode("!!").is_err());
        assert!(Continue::decode(&base64url(b"{}")).is_err());
    }

    #[test]
    fn base64_matches_rfc4648() {
        assert_eq!(base64url(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64url(b"fo"), "Zm8");
        assert_eq!(unbase64url("Zm8").unwrap(), b"fo");
        assert_eq!(unbase64url("Zm9vYg==").unwrap(), b"foob");
    }
}
