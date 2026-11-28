//! `kubemu-oracle-fuzz --oracle PATH [--seed S] [--cases N]`
//!
//! Build the oracle first: `mise run fuzz` does both.
//!
//! `--openapi-v2-pb check|write` instead checks the committed protobuf
//! OpenAPI v2 against Go's conversion of the served JSON, or writes Go's
//! conversion to stdout (ADR 0014; `mise run openapi-v2-pb`).

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: kubemu-oracle-fuzz --oracle PATH [--op merge|jsonpatch|smp|quantity|default|validate|strategy|proto|service] [--seed S] [--cases N] | --oracle PATH --openapi-v2-pb check|write";

type Case = fn(&mut kubemu_oracle_fuzz::Oracle, u64) -> kubemu_oracle_fuzz::CaseResult;

const OPS: &[(&str, Case)] = &[
    ("merge", kubemu_oracle_fuzz::merge_case),
    ("jsonpatch", kubemu_oracle_fuzz::jsonpatch_case),
    ("smp", kubemu_oracle_fuzz::smp_case),
    ("quantity", kubemu_oracle_fuzz::quantity_case),
    ("default", kubemu_oracle_fuzz::default_case),
    ("validate", kubemu_oracle_fuzz::validate_case),
    ("strategy", kubemu_oracle_fuzz::strategy_case),
    ("proto", kubemu_oracle_fuzz::proto_case),
    ("service", kubemu_oracle_fuzz::service_case),
];

fn main() -> ExitCode {
    let mut oracle = None;
    let mut start = 0u64;
    let mut cases = 10_000u64;
    let mut only: Option<String> = None;
    let mut openapi_pb: Option<String> = None;

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
            "--openapi-v2-pb" => openapi_pb = args.next(),
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

    if let Some(mode) = openapi_pb {
        return openapi_v2_pb(&mut o, &mode);
    }

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

fn openapi_v2_pb(o: &mut kubemu_oracle_fuzz::Oracle, mode: &str) -> ExitCode {
    let pb = match kubemu_oracle_fuzz::openapi_v2_pb(o) {
        Ok(pb) => pb,
        Err(e) => {
            eprintln!("oracle openapi-v2-pb: {e}");
            return ExitCode::FAILURE;
        }
    };
    match mode {
        "write" => {
            use std::io::Write;
            let mut out = std::io::stdout().lock();
            if out.write_all(&pb).and_then(|()| out.flush()).is_err() {
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        "check" if pb == kubemu_api_types::OPENAPI_V2_PB => {
            println!(
                "openapi-v2-pb: committed protobuf matches the served JSON ({} bytes)",
                pb.len()
            );
            ExitCode::SUCCESS
        }
        "check" => {
            eprintln!(
                "openapi-v2-pb: crates/kubemu-api-types/openapi-v2.pb is stale; run `mise run openapi-v2-pb`"
            );
            ExitCode::FAILURE
        }
        other => {
            eprintln!("--openapi-v2-pb wants check or write, not {other:?}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
