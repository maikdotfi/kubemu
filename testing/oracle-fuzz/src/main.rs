//! `kubemu-oracle-fuzz --oracle PATH [--seed S] [--cases N]`
//!
//! Build the oracle first: `mise run fuzz` does both.

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: kubemu-oracle-fuzz --oracle PATH [--seed S] [--cases N]";

fn main() -> ExitCode {
    let mut oracle = None;
    let mut start = 0u64;
    let mut cases = 10_000u64;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut num = |name: &str| -> u64 {
            args.next()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| panic!("{name} needs a numeric argument"))
        };
        match a.as_str() {
            "--seed" => start = num("--seed"),
            "--cases" => cases = num("--cases"),
            "--oracle" => oracle = args.next().map(PathBuf::from),
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("unknown argument {other:?}\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(oracle) = oracle else {
        eprintln!("--oracle is required\n{USAGE}");
        return ExitCode::from(2);
    };

    let mut o = match kubemu_oracle_fuzz::Oracle::spawn(&oracle) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("cannot start oracle {}: {e}", oracle.display());
            return ExitCode::FAILURE;
        }
    };

    let mut failed = 0u64;
    for seed in start..start + cases {
        match kubemu_oracle_fuzz::merge_case(&mut o, seed) {
            Ok(Ok(())) => {}
            Ok(Err(diff)) => {
                failed += 1;
                eprintln!(
                    "FAIL seed={seed}: {diff}\n  replay: kubemu-oracle-fuzz --oracle {} --seed {seed} --cases 1",
                    oracle.display()
                );
            }
            Err(e) => {
                eprintln!("oracle transport error at seed={seed}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    println!(
        "merge: {cases} cases from {start}: {} passed, {failed} failed",
        cases - failed
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
