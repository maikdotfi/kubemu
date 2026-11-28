//! `kubemu-bench --kubemu PATH [--runs N] [--record]`
//!
//! Measures the resource and latency budgets of README §4.4 that exist in
//! M1 and compares them with the budgets and with `baseline.json`:
//!
//! - empty-cluster-ready: from spawning `kubemu up` to its default
//!   ServiceAccount answering 200 over HTTP (the cluster settles its
//!   controllers before serving);
//! - idle RSS of `kubemu up`, a few seconds after ready;
//! - fork: `Api::fork_cluster` of a settled empty cluster and of one with
//!   10k ConfigMaps, in process. Until M4 a fork is the store fork alone
//!   (ADR 0002); databases and volumes join it then.
//!
//! Exits 1 when a metric is over budget, or idle RSS is more than
//! [`MAX_RSS_GROWTH`] over the baseline. Times are gated by budget only:
//! the baseline comes from another machine, and shared CI runners are too
//! noisy for tighter time gates. `--record` prints a new `baseline.json`.

use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{ChildStderr, ExitCode};
use std::time::Duration;

use kubemu_api::{Api, HttpRequest};
use kubemu_env_real::{net, process};
use serde_json::{Value, json};

const USAGE: &str = "usage: kubemu-bench --kubemu PATH [--runs N] [--record]";
const BASELINE: &str = include_str!("../baseline.json");
/// How long `kubemu up` sits idle before its RSS is read.
const IDLE: Duration = Duration::from_secs(2);
const MAX_RSS_GROWTH: f64 = 1.25;
const FORKS: usize = 50;
const MIB: f64 = 1024.0 * 1024.0;

#[derive(Debug)]
struct Metric {
    name: &'static str,
    unit: &'static str,
    /// README §4.4.
    budget: f64,
    /// Median over the runs.
    value: f64,
}

fn main() -> ExitCode {
    let mut kubemu = None;
    let mut runs = 5usize;
    let mut record = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--kubemu" => kubemu = args.next().map(PathBuf::from),
            "--runs" => match args.next().and_then(|n| n.parse().ok()).filter(|&n| n > 0) {
                Some(n) => runs = n,
                None => return usage("--runs needs a positive number"),
            },
            "--record" => record = true,
            other => return usage(&format!("unknown argument {other:?}")),
        }
    }
    let Some(kubemu) = kubemu else {
        return usage("--kubemu is required");
    };

    let (env, _) = kubemu_env_real::real_env(Some(1));
    let ms = |d: Duration| d.as_secs_f64() * 1e3;
    let mut ready = Vec::new();
    let mut rss = Vec::new();
    for _ in 0..runs {
        let start = env.clock.now();
        let mut child = match process::spawn_stderr_piped(
            &kubemu,
            &["up", "--port", "0", "--cluster", "bench"],
        ) {
            Ok(c) => c,
            Err(e) => return fail(&format!("cannot start {}: {e}", kubemu.display())),
        };
        // Held until the kill: a closed pipe would fail kubemu's later writes.
        let mut stderr = child.stderr.take().map(BufReader::new);
        let measured = wait_ready(stderr.as_mut()).map(|_| {
            let r = ms(env.clock.now().saturating_since(start));
            process::sleep(IDLE);
            (r, process::rss_bytes(child.id()))
        });
        let _ = child.kill();
        let _ = child.wait();
        drop(stderr);
        match measured {
            Ok((r, Ok(bytes))) => {
                ready.push(r);
                rss.push(bytes as f64 / MIB);
            }
            Ok((_, Err(e))) => return fail(&format!("cannot read RSS: {e}")),
            Err(e) => return fail(&e),
        }
    }
    let fork_us = |objects| {
        let times = fork_times(objects);
        median(times.into_iter().map(|d| d.as_secs_f64() * 1e6).collect())
    };
    let metrics = [
        Metric {
            name: "idle_rss_mib",
            unit: "MiB",
            budget: 20.0,
            value: median(rss),
        },
        Metric {
            name: "empty_cluster_ready_ms",
            unit: "ms",
            budget: 100.0,
            value: median(ready),
        },
        Metric {
            name: "fork_empty_us",
            unit: "us",
            budget: 50_000.0,
            value: fork_us(0),
        },
        Metric {
            name: "fork_10k_objects_us",
            unit: "us",
            budget: 50_000.0,
            value: fork_us(10_000),
        },
    ];

    if record {
        let m: serde_json::Map<String, Value> = metrics
            .iter()
            .map(|m| (m.name.to_owned(), json!((m.value * 1e3).round() / 1e3)))
            .collect();
        let out = json!({ "runs": runs, "metrics": m });
        println!(
            "{}",
            serde_json::to_string_pretty(&out).expect("JSON serializes")
        );
        return ExitCode::SUCCESS;
    }

    let baseline: Value = serde_json::from_str(BASELINE).expect("baseline.json is JSON");
    let mut ok = true;
    println!(
        "{:<24} {:>10} {:>10} {:>10}  result",
        "metric", "value", "budget", "baseline"
    );
    for m in &metrics {
        let base = baseline["metrics"][m.name].as_f64();
        let mut verdict = "ok";
        if m.value > m.budget {
            verdict = "OVER BUDGET";
        } else if m.name == "idle_rss_mib" && base.is_some_and(|b| m.value > b * MAX_RSS_GROWTH) {
            verdict = "REGRESSED";
        }
        ok &= verdict == "ok";
        println!(
            "{:<24} {:>10.3} {:>10} {:>10}  {verdict}",
            m.name,
            m.value,
            format!("{} {}", m.budget, m.unit),
            base.map_or("-".into(), |b| format!("{b:.3}")),
        );
    }
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Reads `kubemu up`'s stderr up to its address, then probes the default
/// ServiceAccount until it answers 200.
fn wait_ready(stderr: Option<&mut BufReader<ChildStderr>>) -> Result<SocketAddr, String> {
    let stderr = stderr.ok_or("no stderr pipe")?;
    let addr: SocketAddr = loop {
        let mut line = String::new();
        if stderr.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
            return Err("kubemu up exited before serving".into());
        }
        if let Some(url) = line.split("serving on http://").nth(1) {
            let addr = url.split_whitespace().next().unwrap_or_default();
            break addr
                .parse()
                .map_err(|_| format!("bad address in {line:?}"))?;
        }
    };
    for _ in 0..5000 {
        match net::http_get_status(addr, "/api/v1/namespaces/default/serviceaccounts/default") {
            Ok(200) => return Ok(addr),
            Ok(_) | Err(_) => process::sleep(Duration::from_millis(1)),
        }
    }
    Err("kubemu up never became ready".into())
}

/// Times [`FORKS`] forks of a settled cluster holding `objects` extra
/// ConfigMaps.
fn fork_times(objects: usize) -> Vec<Duration> {
    let (env, _) = kubemu_env_real::real_env(Some(1));
    let mut controllers = kubemu_controllers::standard(&env, "bench");
    controllers.push(Box::new(kubemu_scheduler::Scheduler::new(env.clone())));
    let mut manager = kubemu_controllers::Manager::new(controllers);
    let mut api = Api::new(env.clone());
    api.add_cluster("bench");
    manager.settle(&mut api, "bench", 100);
    for i in 0..objects {
        let body = json!({"apiVersion": "v1", "kind": "ConfigMap",
            "metadata": {"name": format!("cm-{i}")}, "data": {"k": "v"}});
        let req = HttpRequest::new("POST", "/api/v1/namespaces/default/configmaps").json(&body);
        assert_eq!(api.handle("bench", &req).status(), 201, "create cm-{i}");
    }
    (0..FORKS)
        .map(|i| {
            let t = env.clock.now();
            api.fork_cluster("bench", &format!("fork-{i}"))
                .expect("fork succeeds");
            env.clock.now().saturating_since(t)
        })
        .collect()
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn usage(msg: &str) -> ExitCode {
    eprintln!("kubemu-bench: {msg}\n{USAGE}");
    ExitCode::from(2)
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("kubemu-bench: {msg}");
    ExitCode::FAILURE
}
