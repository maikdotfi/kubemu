#!/usr/bin/env bash
# ADR 0012: vendor upstream api/openapi-spec at the tag pinned by ADR 0007
# (the kubectl pin in mise.toml). The files are committed; builds never fetch.
# kubemu-api-types/build.rs filters them to the served resources.
#
# To serve a new group-version, add its v3 file to V3 and rerun.
set -euo pipefail
cd "$(dirname "$0")/.."

tag="v$(sed -n 's/^kubectl *= *"\([0-9.]*\)".*/\1/p' mise.toml)"
[ "$tag" != "v" ] || { echo "no kubectl pin in mise.toml"; exit 1; }
V3=(
  api__v1
  apis__discovery.k8s.io__v1
)

out=crates/kubemu-api-types/openapi
base="https://raw.githubusercontent.com/kubernetes/kubernetes/$tag/api/openapi-spec"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

mkdir -p "$tmp/v3"
curl -fsSL "$base/swagger.json" -o "$tmp/swagger.json"
for gv in "${V3[@]}"; do
  curl -fsSL "$base/v3/${gv}_openapi.json" -o "$tmp/v3/${gv}_openapi.json"
done
echo "$tag" > "$tmp/VERSION"

rm -rf "$out"
mv "$tmp" "$out"
trap - EXIT
echo "vendored api/openapi-spec $tag into $out"
