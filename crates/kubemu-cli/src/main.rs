//! The kubemu binary. Commands are wired up milestone by milestone (README §9, §12).

use std::process::ExitCode;

const COMMANDS: &[(&str, &str, &str)] = &[
    ("up", "serve one cluster until interrupted", "M1"),
    ("down", "stop clusters and clean up", "M1"),
    ("kubeconfig", "print a kubeconfig for a cluster", "M1"),
    (
        "snapshot",
        "save | fork | export | import cluster snapshots",
        "M4",
    ),
    ("faults", "apply a FaultSchedule", "M4"),
    ("replay", "replay a run from its seed and trace", "M4"),
    (
        "report",
        "render a test report (JSON, JUnit, summary)",
        "M4",
    ),
];

fn usage() {
    println!(
        "kubemu {}: emulated Kubernetes for tests\n",
        env!("CARGO_PKG_VERSION")
    );
    println!("usage: kubemu <command>\n\ncommands:");
    println!("  version      print version");
    for (name, help, milestone) in COMMANDS {
        println!("  {name:<12} {help} [{milestone}]");
    }
}

const UP_USAGE: &str =
    "usage: kubemu up [--cluster NAME] [--host HOST] [--port PORT] [--kubeconfig FILE] [--seed N]";
const KUBECONFIG_USAGE: &str =
    "usage: kubemu kubeconfig [--cluster NAME] [--host HOST] [--port PORT]";

/// Options shared by `up` and `kubeconfig`.
#[derive(Debug)]
struct Opts {
    cluster: String,
    host: String,
    port: u16,
    kubeconfig: Option<std::path::PathBuf>,
    seed: Option<u64>,
}

fn parse_opts(args: &[String], allow_up: bool) -> Result<Opts, String> {
    let mut o = Opts {
        cluster: "kubemu".into(),
        host: "127.0.0.1".into(),
        port: 6443,
        kubeconfig: None,
        seed: None,
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().ok_or(format!("{a} needs a value"));
        match a.as_str() {
            "--cluster" => o.cluster = val()?,
            "--host" => o.host = val()?,
            "--port" => {
                o.port = val()?
                    .parse()
                    .map_err(|_| "--port needs a number".to_owned())?
            }
            "--kubeconfig" if allow_up => o.kubeconfig = Some(val()?.into()),
            "--seed" if allow_up => {
                o.seed = Some(
                    val()?
                        .parse()
                        .map_err(|_| "--seed needs a number".to_owned())?,
                )
            }
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    Ok(o)
}

/// `kubemu up`: one cluster, served until Ctrl-C.
fn up(o: Opts) -> ExitCode {
    let (env, seed) = kubemu_env_real::real_env(o.seed);
    let mut api = kubemu_api::Api::new(env);
    api.add_cluster(&o.cluster);
    let listener = match kubemu_env_real::net::bind_tcp(&format!("{}:{}", o.host, o.port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("kubemu up: cannot listen on {}:{}: {e}", o.host, o.port);
            return ExitCode::FAILURE;
        }
    };
    let addr = listener
        .local_addr()
        .expect("bound listener has an address");
    let url = kubemu_apiserver::server_url(addr);
    if let Some(path) = &o.kubeconfig {
        let k = kubemu_apiserver::kubeconfig(&o.cluster, &url);
        if let Err(e) = kubemu_env_real::fs::write(path, k.as_bytes()) {
            eprintln!("kubemu up: cannot write {}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    }
    // Principle 1.6: say loudly what is emulated or missing.
    eprintln!(
        "kubemu: cluster {:?} serving on {url} (seed {seed})",
        o.cluster
    );
    eprintln!(
        "kubemu: WARNING: plain HTTP and no authentication; every client is cluster-admin (ADR 0011)"
    );
    eprintln!(
        "kubemu: WARNING: no controllers yet (M1 in progress): nothing reconciles, pods are never scheduled"
    );
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("kubemu up: cannot start the runtime: {e}");
            return ExitCode::FAILURE;
        }
    };
    let shared = kubemu_apiserver::Shared::new(api, o.cluster);
    let result = rt.block_on(kubemu_apiserver::serve(listener, shared, async {
        let _ = tokio::signal::ctrl_c().await;
        eprintln!("kubemu: shutting down");
    }));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kubemu up: {e}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("up") => match parse_opts(&args[1..], true) {
            Ok(o) => up(o),
            Err(e) => {
                eprintln!("kubemu up: {e}\n{UP_USAGE}");
                ExitCode::from(2)
            }
        },
        Some("kubeconfig") => match parse_opts(&args[1..], false) {
            Ok(o) => {
                let url = format!("http://{}:{}", o.host, o.port);
                print!("{}", kubemu_apiserver::kubeconfig(&o.cluster, &url));
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("kubemu kubeconfig: {e}\n{KUBECONFIG_USAGE}");
                ExitCode::from(2)
            }
        },
        None | Some("-h" | "--help" | "help") => {
            usage();
            ExitCode::SUCCESS
        }
        Some("version" | "--version" | "-V") => {
            println!("kubemu {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(cmd) => match COMMANDS.iter().find(|(name, ..)| *name == cmd) {
            Some((_, _, milestone)) => {
                eprintln!("kubemu {cmd}: not implemented yet (planned for {milestone})");
                ExitCode::from(64)
            }
            None => {
                eprintln!("kubemu: unknown command {cmd:?}\n");
                usage();
                ExitCode::from(2)
            }
        },
    }
}
