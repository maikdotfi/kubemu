//! The kubemu binary. Commands are wired up milestone by milestone (README §9, §12).

use std::process::ExitCode;

const COMMANDS: &[(&str, &str, &str)] = &[
    ("up", "start clusters from kubemu.yaml", "M1"),
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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
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
