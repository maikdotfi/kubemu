//! `kubemu-oracle-fuzz --oracle PATH [--seed S] [--cases N]`
//!
//! Build the oracle first: `mise run fuzz` does both.

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str =
    "usage: kubemu-oracle-fuzz --oracle PATH [--op merge|jsonpatch|smp] [--seed S] [--cases N]";

type Case = fn(&mut kubemu_oracle_fuzz::Oracle, u64) -> kubemu_oracle_fuzz::CaseResult;

const OPS: &[(&str, Case)] = &[
    ("merge", kubemu_oracle_fuzz::merge_case),
    ("jsonpatch", kubemu_oracle_fuzz::jsonpatch_case),
    ("smp", kubemu_oracle_fuzz::smp_case),
];

fn main() -> ExitCode {
    let mut oracle = None;
    let mut start = 0u64;
    let mut cases = 10_000u64;
    let mut only: Option<String> = None;

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
            "--op" => only = args.next(),
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

    if let Some(op) = &only
        && !OPS.iter().any(|(name, _)| name == op)
    {
        eprintln!("unknown op {op:?}\n{USAGE}");
        return ExitCode::from(2);
    }

    let mut any_failed = false;
    for (name, case) in OPS {
        if only.as_ref().is_some_and(|o| o != name) {
            continue;
        }
        let (mut failed, mut applied) = (0u64, 0u64);
        for seed in start..start + cases {
            match case(&mut o, seed) {
                Ok(Ok(ok)) => applied += u64::from(ok),
                Ok(Err(diff)) => {
                    failed += 1;
                    eprintln!(
                        "FAIL {name} seed={seed}: {diff}\n  replay: mise run fuzz -- --op {name} --seed {seed} --cases 1"
                    );
                }
                Err(e) => {
                    eprintln!("oracle transport error at {name} seed={seed}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        }
        // "applied": both sides accepted the patch (the rest were rejected
        // by both); a fuzz that only agrees on errors proves little.
        println!(
            "{name}: {cases} cases from {start}: {} passed ({applied} applied), {failed} failed",
            cases - failed
        );
        any_failed |= failed > 0;
    }
    if any_failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
