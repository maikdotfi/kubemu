#!/usr/bin/env bash
# M1 exit criterion (README §12.1): kubectl get/apply/delete/watch against
# `kubemu up`, plus the typed clients that send protobuf bodies (kubectl's
# typed commands, a Helm install; ADR 0014). Uses the kubectl and helm
# pinned in mise.toml.
set -euo pipefail
cd "$(dirname "$0")/.."

KUBECTL=${KUBECTL:-$(mise which kubectl)}
HELM=${HELM:-$(mise which helm)}
cargo build -q -p kubemu-cli
work=$(mktemp -d)
log="$work/up.log"
export KUBECONFIG="$work/kubeconfig"
target/debug/kubemu up --port 0 --cluster smoke --kubeconfig "$KUBECONFIG" 2>"$log" &
pid=$!
trap 'kill $pid 2>/dev/null || true; rm -rf "$work"' EXIT
for _ in $(seq 100); do grep -q serving "$log" && break; sleep 0.1; done
grep -q serving "$log" || { cat "$log"; exit 1; }

k() { "$KUBECTL" "$@"; }
fail() { echo "SMOKE FAIL: $*"; cat "$log"; exit 1; }
expect() { [ "$1" = "$2" ] || fail "$3: got '$1', want '$2'"; }

k version >/dev/null
grep -q '^pods ' <<<"$(k api-resources)" || fail "api-resources lists pods"
expect "$(k get ns -o name | sort | tr '\n' ' ')" "namespace/default namespace/kube-node-lease namespace/kube-public namespace/kube-system " "system namespaces"

cat >"$work/app.yaml" <<'YAML'
apiVersion: v1
kind: Namespace
metadata: {name: smoke}
---
apiVersion: v1
kind: ConfigMap
metadata: {name: settings, namespace: smoke}
data: {mode: fast}
---
apiVersion: v1
kind: Pod
metadata: {name: web, namespace: smoke, labels: {app: web}}
spec:
  containers: [{name: web, image: nginx, ports: [{name: http, containerPort: 80}]}]
---
apiVersion: v1
kind: Service
metadata: {name: web, namespace: smoke}
spec:
  selector: {app: web}
  ports: [{port: 8080, targetPort: http}]
YAML
k apply -f "$work/app.yaml" >/dev/null
sed -i.bak 's/mode: fast/mode: slow/' "$work/app.yaml"
expect "$(k apply -f "$work/app.yaml" | grep settings)" "configmap/settings configured" "re-apply"
expect "$(k -n smoke get cm settings -o jsonpath='{.data.mode}')" "slow" "apply updated data"

# Watch while writing.
k -n smoke get cm -w --output-watch-events --no-headers >"$work/watch" 2>&1 &
wpid=$!
sleep 1
k -n smoke patch cm settings --type=merge -p '{"data":{"mode":"patched"}}' >/dev/null
k -n smoke get cm settings -o json | sed 's/"patched"/"replaced"/' | k replace -f - >/dev/null
k -n smoke delete cm settings >/dev/null
for _ in $(seq 50); do grep -q DELETED "$work/watch" && break; sleep 0.1; done
kill $wpid 2>/dev/null || true
expect "$(awk '{print $1}' "$work/watch" | tr '\n' ' ')" "ADDED MODIFIED MODIFIED DELETED " "watch events"

# Controllers: the node, default ServiceAccounts, scheduling, the
# simulated kubelet and endpoints.
expect "$(k get nodes -o name)" "node/smoke-node" "logical node"
k -n smoke wait --for=condition=Ready pod/web --timeout=10s >/dev/null
expect "$(k -n smoke get sa default -o name)" "serviceaccount/default" "default ServiceAccount"
ip=$(k -n smoke get pod web -o jsonpath='{.status.podIP}')
expect "$(k -n smoke get endpoints web -o jsonpath='{.subsets[0].addresses[0].ip}:{.subsets[0].ports[0].port}')" "$ip:80" "endpoints"
expect "$(k -n smoke get endpointslices -l kubernetes.io/service-name=web -o jsonpath='{.items[0].endpoints[0].addresses[0]}')" "$ip" "endpointslice"

k -n smoke label pod web tier=fe >/dev/null
expect "$(k -n smoke get pods -l tier=fe -o name)" "pod/web" "label selector"
k -n smoke delete pod web >/dev/null
expect "$(k -n smoke get pods -o name)" "" "pod deleted"
# Typed commands send protobuf bodies (ADR 0014).
k -n smoke create configmap typed --from-literal=a=b >/dev/null
expect "$(k -n smoke get cm typed -o jsonpath='{.data.a}')" "b" "typed create configmap"
k -n smoke create secret generic typed --from-literal=p=q >/dev/null
expect "$(k -n smoke get secret typed -o jsonpath='{.data.p}')" "cQ==" "typed create secret"
expect "$(k auth can-i create pods)" "yes" "auth can-i"
# --validate=false: for create-only kinds kubectl's validation lists CRDs,
# which kubemu does not serve yet (M3).
expect "$(printf '%s' '{"apiVersion":"authorization.k8s.io/v1","kind":"SelfSubjectAccessReview","spec":{"resourceAttributes":{"verb":"create","resource":"pods"}}}' \
  | k create --validate=false -f - -o jsonpath='{.status.allowed}')" "true" "self-subject access review"
k get cm -A --field-selector metadata.namespace=kube-system >/dev/null
# Helm stores its release in a Secret through the typed client (protobuf).
# --server-side=false: server-side apply is M3.
mkdir -p "$work/chart/templates"
printf 'apiVersion: v2\nname: smoke\nversion: 0.1.0\n' >"$work/chart/Chart.yaml"
printf 'apiVersion: v1\nkind: ConfigMap\nmetadata: {name: from-helm}\ndata: {a: b}\n' >"$work/chart/templates/cm.yaml"
"$HELM" install rel "$work/chart" -n smoke --server-side=false >/dev/null || fail "helm install"
expect "$(k -n smoke get cm from-helm -o name)" "configmap/from-helm" "helm installed"
expect "$("$HELM" list -n smoke -q)" "rel" "helm list"
"$HELM" uninstall rel -n smoke >/dev/null || fail "helm uninstall"
expect "$(k -n smoke get cm from-helm --ignore-not-found -o name)" "" "helm uninstalled"
# Namespace deletion completes (namespace controller, GC of the slice).
k delete ns smoke --timeout=10s >/dev/null
expect "$(k get ns smoke --ignore-not-found -o name)" "" "namespace deleted"
echo "kubectl smoke: ok ($("$KUBECTL" version --client -o json | grep -m1 gitVersion | tr -d ' ",'), helm $("$HELM" version --short))"
