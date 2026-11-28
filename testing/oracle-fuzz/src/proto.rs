//! Protobuf request bodies (ADR 0014) against the oracle's `proto` op: Go
//! encodes a typed object as a typed client would send it, and answers with
//! the JSON the apiserver makes of those bytes. kubemu-proto must produce
//! the same JSON from the same bytes.

use serde_json::{Value, json};

use crate::validation::{stored_kind, typed};
use crate::{CaseResult, Gen, Oracle, same_json};

/// Standard base64 (Go's JSON encoding of `[]byte`).
pub(crate) fn unbase64(s: &str) -> Option<Vec<u8>> {
    let val = |c: u8| -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        } as u32)
    };
    let s = s.trim_end_matches('=').as_bytes();
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    for c in s.chunks(4) {
        let mut n = 0u32;
        for (i, &b) in c.iter().enumerate() {
            n |= val(b)? << (18 - 6 * i);
        }
        out.extend_from_slice(&n.to_be_bytes()[1..c.len()]);
    }
    Some(out)
}

pub fn proto_case(oracle: &mut Oracle, seed: u64) -> CaseResult {
    let mut g = Gen::new(seed);
    let r = stored_kind(&mut g);
    let mut obj = typed(&mut g, r.schema(), 7).unwrap_or_else(|| json!({}));
    obj["apiVersion"] = json!(r.api_version());
    obj["kind"] = json!(r.kind);
    if g.chance(3, 4) {
        obj["metadata"]["name"] = json!(*g.pick(&["a", "b-1"]));
    }
    if g.chance(1, 2) {
        obj["metadata"]["creationTimestamp"] = json!("2026-01-01T00:00:00Z");
    }
    let theirs = match oracle.call("proto", json!({"object": obj}))? {
        Ok(v) => v,
        // Go's typed decode refused the input: nothing to send.
        Err(_) => return Ok(Ok(false)),
    };
    let bytes = theirs["protobuf"]
        .as_str()
        .and_then(unbase64)
        .ok_or_else(|| std::io::Error::other("oracle: bad protobuf field"))?;
    let ours = kubemu_proto::decode(&bytes, "", "");
    Ok(match ours {
        Ok(v) if same_json(&v, &theirs["json"]) => Ok(true),
        other => Err(format!(
            "proto {} mismatch\n  input:  {obj}\n  kubemu: {other:?}\n  oracle: {}",
            r.kind, theirs["json"]
        )),
    })
}

/// The served OpenAPI v2 as Go converts it to protobuf.
pub fn openapi_v2_pb(oracle: &mut Oracle) -> std::io::Result<Vec<u8>> {
    let doc: Value =
        serde_json::from_str(kubemu_api_types::OPENAPI_V2).map_err(std::io::Error::other)?;
    match oracle.call("openapi-v2-pb", doc)? {
        Ok(v) => v
            .as_str()
            .and_then(unbase64)
            .ok_or_else(|| std::io::Error::other("oracle: bad openapi-v2-pb result")),
        Err(e) => Err(std::io::Error::other(e)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unbase64() {
        assert_eq!(super::unbase64("").unwrap(), b"");
        assert_eq!(super::unbase64("Zg==").unwrap(), b"f");
        assert_eq!(super::unbase64("Zm8=").unwrap(), b"fo");
        assert_eq!(super::unbase64("Zm9v").unwrap(), b"foo");
        assert_eq!(super::unbase64("//4=").unwrap(), [0xff, 0xfe]);
    }
}
