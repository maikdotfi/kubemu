//! Shell tests: real HTTP over a real socket against an in-memory core
//! (ADR 0011). Everything else is tested in `kubemu-api`.

use std::net::SocketAddr;

use kubemu_api::Api;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use super::*;

async fn start() -> (SocketAddr, tokio::sync::oneshot::Sender<()>) {
    let (env, _) = kubemu_env_real::real_env(Some(1));
    let mut api = Api::new(env);
    api.add_cluster("test");
    let l = kubemu_env_real::net::bind_tcp("127.0.0.1:0").unwrap();
    let addr = l.local_addr().unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(serve(l, Shared::new(api, "test"), async {
        let _ = rx.await;
    }));
    (addr, tx)
}

async fn connect(addr: SocketAddr) -> TcpStream {
    TcpStream::from_std(kubemu_env_real::net::connect_tcp(addr).unwrap()).unwrap()
}

/// One HTTP/1.1 request; returns status, headers and body.
async fn call(
    addr: SocketAddr,
    method: &str,
    path: &str,
    ct: &str,
    body: &str,
) -> (u16, String, Value) {
    let mut s = connect(addr).await;
    let req = format!(
        "{method} {path} HTTP/1.1\r\nHost: x\r\nConnection: close\r\nContent-Type: {ct}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    s.write_all(req.as_bytes()).await.unwrap();
    let mut raw = String::new();
    s.read_to_string(&mut raw).await.unwrap();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap();
    let status = head[9..12].parse().unwrap();
    let body = if head
        .to_ascii_lowercase()
        .contains("transfer-encoding: chunked")
    {
        dechunk(body)
    } else {
        body.to_owned()
    };
    (
        status,
        head.to_owned(),
        serde_json::from_str(&body).unwrap_or(Value::Null),
    )
}

fn dechunk(mut s: &str) -> String {
    let mut out = String::new();
    while let Some((len, rest)) = s.split_once("\r\n") {
        let n = usize::from_str_radix(len.trim(), 16).unwrap_or(0);
        if n == 0 {
            break;
        }
        out.push_str(&rest[..n]);
        s = &rest[n + 2..];
    }
    out
}

#[tokio::test]
async fn crud_over_http() {
    let (addr, _stop) = start().await;
    let (s, _, v) = call(addr, "GET", "/api/v1/namespaces/default", "", "").await;
    assert_eq!((s, v["metadata"]["name"].as_str()), (200, Some("default")));

    let cm = json!({"metadata": {"name": "a"}, "data": {"k": "v"}, "bogus": 1}).to_string();
    let (s, head, v) = call(
        addr,
        "POST",
        "/api/v1/namespaces/default/configmaps",
        "application/json",
        &cm,
    )
    .await;
    assert_eq!(s, 201);
    assert_eq!(v["data"]["k"], "v");
    assert!(
        head.to_ascii_lowercase()
            .contains("warning: 299 - \"unknown field \\\"bogus\\\"\""),
        "{head}"
    );

    let (s, _, v) = call(
        addr,
        "GET",
        "/api/v1/namespaces/default/configmaps/nope",
        "",
        "",
    )
    .await;
    assert_eq!(
        (s, v["kind"].as_str(), v["reason"].as_str()),
        (404, Some("Status"), Some("NotFound"))
    );
    let (s, _, _) = call(
        addr,
        "GET",
        "/api/v1/namespaces/default/configmaps?labelSelector=a%20in%20(b)",
        "",
        "",
    )
    .await;
    assert_eq!(s, 200);
}

#[tokio::test]
async fn watch_streams_changes() {
    let (addr, _stop) = start().await;
    let mut w = connect(addr).await;
    w.write_all(b"GET /api/v1/namespaces/default/configmaps?watch=1&timeoutSeconds=30 HTTP/1.1\r\nHost: x\r\n\r\n")
        .await
        .unwrap();
    let mut r = BufReader::new(w);
    let mut line = String::new();
    loop {
        line.clear();
        r.read_line(&mut line).await.unwrap();
        if line == "\r\n" {
            break;
        }
    }
    let cm = json!({"metadata": {"name": "w"}}).to_string();
    call(
        addr,
        "POST",
        "/api/v1/namespaces/default/configmaps",
        "application/json",
        &cm,
    )
    .await;
    // Chunked: a size line, then the event line.
    let event = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            line.clear();
            r.read_line(&mut line).await.unwrap();
            if line.starts_with('{') {
                return serde_json::from_str::<Value>(line.trim()).unwrap();
            }
        }
    })
    .await
    .expect("the watch delivers the write");
    assert_eq!(event["type"], "ADDED");
    assert_eq!(event["object"]["metadata"]["name"], "w");
}

#[test]
fn kubeconfig_names_the_server() {
    let k = kubeconfig("dev", "http://127.0.0.1:6443");
    assert!(k.contains("server: http://127.0.0.1:6443"));
    assert!(k.contains("current-context: kubemu-dev"));
}
