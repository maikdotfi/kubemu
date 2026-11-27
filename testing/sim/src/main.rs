//! `kubemu-sim [--seeds N] [--start S] [--seed X] [--steps K] [--check-determinism]`

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut start = 0u64;
    let mut count = 100u64;
    let mut check_determinism = false;
    let mut cfg = kubemu_sim::Config::default();

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut num = |name: &str| -> u64 {
            args.next()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| panic!("{name} needs a numeric argument"))
        };
        match a.as_str() {
            "--seeds" => count = num("--seeds"),
            "--start" => start = num("--start"),
            "--seed" => {
                start = num("--seed");
                count = 1;
            }
            "--steps" => cfg.steps = u32::try_from(num("--steps")).expect("--steps too large"),
            "--check-determinism" => check_determinism = true,
            "-h" | "--help" => {
                println!(
                    "usage: kubemu-sim [--seeds N] [--start S] [--seed X] [--steps K] [--check-determinism]"
                );
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("unknown argument {other:?}");
                return ExitCode::from(2);
            }
        }
    }

    let mut failed = 0u64;
    for seed in start..start + count {
        let o = kubemu_sim::run(seed, &cfg);
        if !o.ok() {
            failed += 1;
            eprintln!(
                "FAIL seed={seed}: {:?}\n  replay: kubemu-sim --seed {seed} --steps {}",
                o.violations, cfg.steps
            );
        }
        if check_determinism {
            let again = kubemu_sim::run(seed, &cfg);
            if again.trace_hash != o.trace_hash {
                failed += 1;
                eprintln!(
                    "NONDETERMINISM seed={seed}: {:016x} != {:016x}",
                    o.trace_hash, again.trace_hash
                );
            }
        }
        if count == 1 {
            println!(
                "seed={seed} trace={:016x} lines={} faults={} forks={}",
                o.trace_hash, o.trace_lines, o.faults, o.forks
            );
        }
    }
    println!(
        "{count} seeds from {start}: {} passed, {failed} failed{}",
        count - failed.min(count),
        if check_determinism {
            " (determinism checked)"
        } else {
            ""
        }
    );
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
