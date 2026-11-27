# AGENTS.md

## Rules
- commit directly to main, always, no PRs
  - pull first from remote
- all ADRs must be signed off by a human before accepted and the work can be done
- ask questions if something important is unclear
- run `mise run ci` before committing
- no real clock, threads, fs, net, processes or `HashMap` outside `kubemu-env-real`
  - go through `kubemu_env::Env`
  - never silence the clippy determinism lints
- `BTreeMap` where order leaks into behavior
- never change the `Xoshiro256StarStar` output stream
- failing DST seed = new regression test
- add deps only when code uses them

## Setup
- `mise install` (Go, kubectl, helm, kind)
- Rust via `rust-toolchain.toml`
- plan: `README.md`, decisions: `docs/adr/`

## Git
- plain `git push` works via `gh`
- auth broken: `gh auth setup-git -h github.com`
