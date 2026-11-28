//! The thin tokio HTTP shell over `kubemu-api` (ADR 0011).
//!
//! It parses nothing it can leave to the frontend: method, path, query,
//! two headers and the body go in, the frontend's reply comes out. The
//! core sits behind a [`Mutex`] and is called inline; the lock is never
//! held across an `.await`. Watches keep a cursor, wait on a [`Notify`]
//! that every write wakes, and stream newline-delimited JSON events.
//!
//! Plain HTTP, no authentication: every caller is cluster-admin (ADR
//! 0011). One cluster per listener.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri, header};
use axum::response::Response;
use kubemu_api::{Api, HttpRequest, HttpResponse, Reply, WatchCursor};
use tokio::sync::Notify;

/// How often a quiet watch re-polls, for bookmarks and its deadline.
const WATCH_TICK: Duration = Duration::from_secs(1);

/// Largest request body accepted.
const MAX_BODY: usize = 64 << 20;

/// The frontend shared by all connections of one listener.
#[derive(Debug)]
pub struct Shared {
    api: Mutex<Api>,
    changed: Notify,
    cluster: String,
}

impl Shared {
    pub fn new(api: Api, cluster: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            api: Mutex::new(api),
            changed: Notify::new(),
            cluster: cluster.into(),
        })
    }

    fn call(&self, req: &HttpRequest) -> Reply {
        self.api
            .lock()
            .expect("api lock poisoned")
            .handle(&self.cluster, req)
    }
}

pub fn router(shared: Arc<Shared>) -> Router {
    Router::new()
        .fallback(handle)
        .layer(DefaultBodyLimit::max(MAX_BODY))
        .with_state(shared)
}

/// Serves `shared` on `listener` (bound through `kubemu_env_real::net`)
/// until `shutdown` resolves.
pub async fn serve(
    listener: kubemu_env_real::net::Listener,
    shared: Arc<Shared>,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::from_std(listener.into_std())?;
    axum::serve(listener, router(shared))
        .with_graceful_shutdown(shutdown)
        .await
}

/// A kubeconfig for a cluster served at `server` (`http://host:port`).
pub fn kubeconfig(cluster: &str, server: &str) -> String {
    let n = format!("kubemu-{cluster}");
    format!(
        "apiVersion: v1
kind: Config
clusters:
- name: {n}
  cluster:
    server: {server}
contexts:
- name: {n}
  context:
    cluster: {n}
    user: {n}
current-context: {n}
users:
- name: {n}
  user: {{}}
preferences: {{}}
"
    )
}

pub fn server_url(addr: SocketAddr) -> String {
    format!("http://{addr}")
}

async fn handle(
    State(s): State<Arc<Shared>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let header = |h: header::HeaderName| {
        headers
            .get(h)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
    };
    let req = HttpRequest {
        method: method.as_str().to_owned(),
        path: uri.path().to_owned(),
        query: uri.query().unwrap_or("").to_owned(),
        content_type: header(header::CONTENT_TYPE),
        accept: header(header::ACCEPT),
        body: body.to_vec(),
    };
    let reply = s.call(&req);
    if method != Method::GET {
        s.changed.notify_waiters();
    }
    match reply {
        Reply::Response(r) => response(r),
        Reply::Watch(c) => watch(s, c),
    }
}

fn response(r: HttpResponse) -> Response {
    let mut resp = Response::new(Body::from(r.body.to_bytes()));
    *resp.status_mut() =
        StatusCode::from_u16(r.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let h = resp.headers_mut();
    if let Ok(ct) = HeaderValue::from_str(&r.content_type) {
        h.insert(header::CONTENT_TYPE, ct);
    }
    for w in r.warnings {
        let v = format!("299 - {}", serde_json::Value::String(w));
        if let Ok(v) = HeaderValue::from_str(&v) {
            h.append(header::WARNING, v);
        }
    }
    resp
}

/// Streams a watch: one JSON event per line, flushed per poll.
fn watch(s: Arc<Shared>, cursor: WatchCursor) -> Response {
    let stream = futures_util::stream::unfold((s, cursor, false), |(s, mut c, done)| async move {
        if done {
            return None;
        }
        loop {
            // Register for the wake-up before polling, so a write between
            // the poll and the wait is not missed.
            let waker = Arc::clone(&s);
            let mut notified = pin!(waker.changed.notified());
            notified.as_mut().enable();
            let batch = s.api.lock().expect("api lock poisoned").watch_poll(&mut c);
            if !batch.events.is_empty() || batch.closed {
                let mut buf = Vec::new();
                for e in &batch.events {
                    buf.extend_from_slice(e.to_string().as_bytes());
                    buf.push(b'\n');
                }
                let closed = batch.closed;
                return Some((Ok::<_, Infallible>(Bytes::from(buf)), (s, c, closed)));
            }
            tokio::select! {
                () = notified => {}
                () = tokio::time::sleep(WATCH_TICK) => {}
            }
        }
    });
    let mut resp = Response::new(Body::from_stream(stream));
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    resp
}

#[cfg(test)]
mod tests;
