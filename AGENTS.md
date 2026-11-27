# AGENTS.md

Guidance for coding agents (and humans) working in this repo. The plan lives in `README.md`; design decisions in `docs/adr/`.

## Setup

- Rust is pinned in `rust-toolchain.toml` (rustup installs it automatically). Everything else (Go, kubectl, helm, kind) comes from `mise.toml`: run `mise install`.
- Before calling work done, run `mise run ci` (fmt, clippy, tests, Go oracle, 1000-seed DST sweep with determinism check). CI runs the same steps.

## Git and GitHub

- Remote is `https://github.com/maikdotfi/kubemu` over HTTPS. Git authenticates through the GitHub CLI, so plain `git push` / `git pull` work as long as `gh auth status` is logged in.
- If a push fails with an auth error, set up the credential helper once per machine:
  ```sh
  gh auth setup-git -h github.com
  ```
  This writes `credential.https://github.com.helper = !gh auth git-credential` to `~/.gitconfig`, overriding the macOS keychain helper.
- Pushing changes under `.github/workflows/` needs the token's `workflow` scope: `gh auth refresh -h github.com -s workflow`.
- Use `gh` for PRs, issues and CI runs (`gh run list`, `gh run watch`).

## Hard rules

- **Determinism boundary.** Never use the real clock, threads, filesystem, network, process spawning or `HashMap`/`HashSet` outside `kubemu-env-real`; go through `kubemu_env::Env`. `clippy.toml` enforces this. Don't silence it with `#[allow]`. Extend `Env` instead, and add new nondeterministic APIs (e.g. `tokio::spawn`, `rand::thread_rng`) to `clippy.toml` when adding such dependencies.
- Use ordered collections (`BTreeMap`) wherever iteration order can leak into behavior or traces.
- Don't change the output stream of `Xoshiro256StarStar`: saved regression seeds depend on it.
- Any failing DST seed becomes a permanent regression test. Replay one with `mise run sim -- --seed N`.
- Cargo warns on unused dependencies: add a crate dependency only when code uses it, and register internal crates in `[workspace.dependencies]` when they gain a dependent.
- Linux is the only supported host for running workloads; the workspace must still build and test on macOS for development.
