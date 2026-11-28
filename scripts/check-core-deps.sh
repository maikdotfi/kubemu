#!/usr/bin/env bash
# ADR 0001: core crates are synchronous state machines. None of them may
# depend, even transitively, on an async runtime or HTTP stack.
set -euo pipefail
cd "$(dirname "$0")/.."

CORE=(kubemu-env kubemu-env-sim kubemu-store kubemu-api-types kubemu-patch kubemu-proto kubemu-validation kubemu-api
      kubemu-controllers kubemu-scheduler kubemu-faults kubemu-snapshot kubemu-net)
BANNED='^(tokio|async-std|smol|hyper|axum|mio) '
fail=0
for c in "${CORE[@]}"; do
  tree=$(cargo tree -q -p "$c" -e normal,build --prefix none)
  hits=$(grep -E "$BANNED" <<<"$tree" | sort -u || true)
  if [ -n "$hits" ]; then
    echo "core crate $c depends on: $hits"; fail=1
  fi
done
[ "$fail" = 0 ] && echo "core crates are runtime-free (${#CORE[@]} checked)"
exit "$fail"
