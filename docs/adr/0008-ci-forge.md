# 0008: Forge choice for CI

- Status: Accepted (signed off by the project owner, 2026-11-27); root trigger amended by 0004
- Date: 2026-11-27

## Context

See README §11. GitHub self-hosted runners vs Forgejo. The scaffold starts on GitHub-hosted runners for lint and unit tests.

## Options

1. **GitHub Actions with self-hosted runners on our server.** The repo, issues and Actions stay on GitHub. Root and namespace-heavy jobs go to sandboxed runners on the server.
2. **Forgejo with Forgejo Actions.** Fully self-hosted, but a second forge to run and mirror, and fewer outside contributors find the project.

## Decision

**Option 1: GitHub Actions, moving to self-hosted runners later.**

- **For now:** everything runs on GitHub-hosted public-repo runners (`ubuntu-latest`) while the free quota lasts. That covers fmt, clippy, tests, the DST sweep with the determinism check, and the Go oracle.
- **Later:** add self-hosted runners on the server when one of these happens:
  - jobs need root, namespaces or mounts (M2 youki executor, pod networking);
  - jobs need long nightly runs;
  - the hosted quota or runtime becomes a constraint.
- Self-hosted jobs follow README §11: a throwaway nspawn container or microVM per job, and no secrets inside the sandbox.

## Consequences

- Workflows target labels so jobs can move between hosted and self-hosted runners without rewrites. Hosted jobs use `ubuntu-latest`; jobs that need privileges will use `[self-hosted, linux, kubemu-sandbox]`.
- Public repo plus self-hosted runners is risky: fork PRs could run code on the server. Before adding the first self-hosted runner, restrict it to pushes on `main` (and approved workflows) and never run fork PRs on it.
- The "CI on the server" item in M0 is satisfied by hosted CI for now and revisited at M2.
- *Amended by [0004](0004-rootful-first.md):* needing root no longer by itself moves jobs to self-hosted runners. Hosted runners have passwordless `sudo` and a fresh VM per job, so privileged M2 jobs run there with `sudo`.
