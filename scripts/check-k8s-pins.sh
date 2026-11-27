#!/usr/bin/env bash
# ADR 0007: every Kubernetes pin targets the same minor.
set -euo pipefail
cd "$(dirname "$0")/.."

want=$(sed -n 's/^kubectl *= *"1\.\([0-9]*\)\..*/\1/p' mise.toml)
[ -n "$want" ] || { echo "no kubectl pin in mise.toml"; exit 1; }
fail=0

for mod in k8s.io/api k8s.io/apimachinery k8s.io/client-go; do
  got=$(sed -n "s|^[[:space:]]*$mod v0\.\([0-9]*\)\..*|\1|p" testing/oracle-go/go.mod | head -1)
  if [ -z "$got" ]; then
    echo "missing $mod in testing/oracle-go/go.mod"; fail=1
  elif [ "$got" != "$want" ]; then
    echo "$mod is v0.$got, want v0.$want (kubectl 1.$want in mise.toml)"; fail=1
  fi
done

# Vendored upstream OpenAPI (ADR 0012): refreshed by scripts/fetch-openapi.sh.
spec=$(cat crates/kubemu-api-types/openapi/VERSION 2>/dev/null || true)
kubectl=$(sed -n 's/^kubectl *= *"\([0-9.]*\)".*/v\1/p' mise.toml)
if [ "$spec" != "$kubectl" ]; then
  echo "vendored openapi is ${spec:-missing}, want $kubectl (run scripts/fetch-openapi.sh)"; fail=1
fi

# k8s-openapi joins in M1; check it once it is a dependency.
if grep -q '^k8s-openapi' Cargo.toml; then
  grep -q "\"v1_$want\"" Cargo.toml || { echo "k8s-openapi feature is not v1_$want"; fail=1; }
fi

[ "$fail" = 0 ] && echo "k8s pins agree on 1.$want"
exit "$fail"
