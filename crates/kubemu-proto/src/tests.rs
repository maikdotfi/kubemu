use serde_json::json;

use super::*;

/// A minimal protobuf writer for building test bodies.
#[derive(Default)]
struct W(Vec<u8>);

impl W {
    fn varint(mut self, n: u32, mut v: u64) -> Self {
        self.key(n, 0);
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.0.push(b);
                break;
            }
            self.0.push(b | 0x80);
        }
        self
    }
    fn key(&mut self, n: u32, wt: u8) {
        let mut k = u64::from(n) << 3 | u64::from(wt);
        loop {
            let b = (k & 0x7f) as u8;
            k >>= 7;
            if k == 0 {
                self.0.push(b);
                break;
            }
            self.0.push(b | 0x80);
        }
    }
    fn bytes(mut self, n: u32, b: &[u8]) -> Self {
        self.key(n, 2);
        let mut len = W::default();
        let mut l = b.len() as u64;
        loop {
            let x = (l & 0x7f) as u8;
            l >>= 7;
            if l == 0 {
                len.0.push(x);
                break;
            }
            len.0.push(x | 0x80);
        }
        self.0.extend(len.0);
        self.0.extend_from_slice(b);
        self
    }
    fn str(self, n: u32, s: &str) -> Self {
        self.bytes(n, s.as_bytes())
    }
    fn msg(self, n: u32, m: W) -> Self {
        self.bytes(n, &m.0)
    }
    fn entry(self, n: u32, k: &str, v: W) -> Self {
        self.msg(n, W::default().str(1, k).msg(2, v))
    }
}

fn envelope(api_version: &str, kind: &str, raw: W) -> Vec<u8> {
    let tm = W::default().str(1, api_version).str(2, kind);
    let mut b = MAGIC.to_vec();
    b.extend(W::default().msg(1, tm).msg(2, raw).0);
    b
}

/// ObjectMeta as Go's marshaller writes it: value fields always present.
fn meta(name: &str) -> W {
    W::default()
        .str(1, name)
        .str(2, "")
        .str(3, "")
        .str(4, "")
        .str(5, "")
        .str(6, "")
        .varint(7, 0)
        .msg(8, W::default())
}

#[test]
fn configmap_as_kubectl_sends_it() {
    let cm = W::default()
        .msg(1, meta("x"))
        .entry(2, "a", W(b"b".to_vec()))
        .entry(3, "bin", W(vec![0, 1, 2]));
    let v = decode(&envelope("v1", "ConfigMap", cm), "", "").unwrap();
    assert_eq!(
        v,
        json!({"apiVersion": "v1", "kind": "ConfigMap", "metadata": {"name": "x"},
               "data": {"a": "b"}, "binaryData": {"bin": "AAEC"}})
    );
}

#[test]
fn secret_and_defaults_from_the_request() {
    let s = W::default()
        .msg(1, meta("sh.helm.release.v1.app.v1"))
        .entry(2, "release", W(b"H4sI".to_vec()))
        .str(3, "helm.sh/release.v1");
    // An envelope without type meta takes the expected kind.
    let mut body = MAGIC.to_vec();
    body.extend(W::default().msg(2, s).0);
    let v = decode(&body, "v1", "Secret").unwrap();
    assert_eq!(v["kind"], "Secret");
    assert_eq!(v["data"]["release"], "SDRzSQ==");
    assert_eq!(v["type"], "helm.sh/release.v1");
}

#[test]
fn service_ports_intorstring_and_zero_values() {
    let port = |p: u64, target: W| {
        W::default()
            .str(1, "")
            .str(2, "TCP")
            .varint(3, p)
            .msg(4, target)
            .varint(5, 0)
    };
    let spec = W::default()
        .msg(
            1,
            port(80, W::default().varint(1, 0).varint(2, 8080).str(3, "")),
        )
        .msg(
            1,
            port(443, W::default().varint(1, 1).varint(2, 0).str(3, "https")),
        )
        .str(3, "")
        .varint(13, 0);
    let svc = W::default()
        .msg(1, meta("web"))
        .msg(2, spec)
        .msg(3, W::default().msg(1, W::default()));
    let v = decode(&envelope("v1", "Service", svc), "", "").unwrap();
    assert_eq!(
        v["spec"],
        json!({"ports": [{"protocol": "TCP", "port": 80, "targetPort": 8080},
                         {"protocol": "TCP", "port": 443, "targetPort": "https"}]})
    );
    // Non-pointer structs stay, even empty (no omitzero on them).
    assert_eq!(v["status"], json!({"loadBalancer": {}}));
}

#[test]
fn pod_pointers_quantities_times_and_negative_ints() {
    let res = W::default().entry(1, "cpu", W::default().str(1, "100m"));
    let c = W::default()
        .str(1, "app")
        .str(2, "nginx")
        .msg(8, res)
        .msg(10, W::default().varint(4, (-1i64) as u64)); // livenessProbe
    let spec = W::default()
        .msg(2, c)
        .varint(4, 30) // terminationGracePeriodSeconds (*int64)
        .varint(11, 0) // hostNetwork false: omitted
        .varint(21, 0); // automountServiceAccountToken (*bool) false: kept
    let mut m = meta("p");
    m = m.msg(9, W::default().varint(1, 1_735_689_600)); // deletionTimestamp
    let pod = W::default().msg(1, m).msg(2, spec);
    let v = decode(&envelope("v1", "Pod", pod), "", "").unwrap();
    assert_eq!(v["metadata"]["deletionTimestamp"], "2025-01-01T00:00:00Z");
    assert!(v["metadata"].get("creationTimestamp").is_none());
    let spec = &v["spec"];
    assert_eq!(spec["terminationGracePeriodSeconds"], 30);
    assert_eq!(spec["automountServiceAccountToken"], false);
    assert!(spec.get("hostNetwork").is_none());
    let c = &spec["containers"][0];
    assert_eq!(c["resources"], json!({"limits": {"cpu": "100m"}}));
    // Probe field 4 is periodSeconds (int32): sign-extended varint.
    assert_eq!(c["livenessProbe"]["periodSeconds"], -1);
}

#[test]
fn errors() {
    assert!(matches!(
        decode(b"{}", "v1", "Pod"),
        Err(Error::Malformed(_))
    ));
    let e = decode(&envelope("example.com/v1", "Widget", W::default()), "", "").unwrap_err();
    assert!(matches!(e, Error::UnknownKind { .. }));
    assert_eq!(
        e.to_string(),
        "no kind \"Widget\" is registered for version \"example.com/v1\" in scheme"
    );
    let mut truncated = envelope("v1", "ConfigMap", W::default().msg(1, meta("x")));
    truncated.pop();
    assert!(decode(&truncated, "", "").is_err());
    // A string field sent as a varint.
    let bad = W::default().msg(1, W::default().varint(1, 5));
    assert!(matches!(
        decode(&envelope("v1", "ConfigMap", bad), "", ""),
        Err(Error::Malformed(_))
    ));
}

#[test]
fn time_and_base64() {
    assert_eq!(format_time(0, 0, false), "1970-01-01T00:00:00Z");
    assert_eq!(
        format_time(951_782_400 + 3661, 0, false),
        "2000-02-29T01:01:01Z"
    );
    assert_eq!(format_time(-1, 0, false), "1969-12-31T23:59:59Z");
    assert_eq!(
        format_time(0, 1_234_000, true),
        "1970-01-01T00:00:00.001234Z"
    );
    assert_eq!(format_time(1, -1_000, true), "1970-01-01T00:00:00.999999Z");
    assert_eq!(base64(b""), "");
    assert_eq!(base64(b"f"), "Zg==");
    assert_eq!(base64(b"fo"), "Zm8=");
    assert_eq!(base64(b"foo"), "Zm9v");
    assert_eq!(base64(&[0xff, 0xfe]), "//4=");
}
