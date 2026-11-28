// Command oracle-go answers kubemu's differential-fuzzing questions using real
// Kubernetes Go code (README §1.4, §10), pinned to the target minor (ADR 0007).
//
// Protocol: one JSON request per line on stdin, one JSON response per line on
// stdout. The Rust fuzzers spawn this process and diff its answers against
// kubemu's.
//
// Each op calls the same function the apiserver's PATCH handler calls
// (k8s.io/apiserver/pkg/endpoints/handlers/patch.go):
//
//	merge      RFC 7386 merge patch    gopkg.in/evanphx/json-patch.v4 MergePatch
//	jsonpatch  RFC 6902 JSON patch     gopkg.in/evanphx/json-patch.v4 DecodePatch + Apply
//	smp        strategic merge patch   strategicpatch.StrategicMergeMapPatch
//
// Planned: ssa (structured-merge-diff), validate (validation and defaulting).
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"

	jsonpatch "gopkg.in/evanphx/json-patch.v4"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/runtime/schema"
	"k8s.io/apimachinery/pkg/util/strategicpatch"
	"k8s.io/client-go/kubernetes/scheme"
	kjson "sigs.k8s.io/json"
)

type Request struct {
	Op      string          `json:"op"`
	Payload json.RawMessage `json:"payload,omitempty"`
}

type Response struct {
	Result json.RawMessage `json:"result,omitempty"`
	Error  string          `json:"error,omitempty"`
}

// patchPayload is the input of merge, jsonpatch and smp. APIVersion and Kind
// pick the Go type whose struct tags drive SMP; the other ops ignore them.
type patchPayload struct {
	APIVersion string          `json:"apiVersion,omitempty"`
	Kind       string          `json:"kind,omitempty"`
	Original   json.RawMessage `json:"original"`
	Patch      json.RawMessage `json:"patch"`
}

// handle answers one request. A panic in the Kubernetes code becomes an
// error, as the apiserver's panic recovery turns it into a 500.
func handle(req Request) (resp Response) {
	defer func() {
		if r := recover(); r != nil {
			resp = Response{Error: fmt.Sprintf("panic: %v", r)}
		}
	}()
	switch req.Op {
	case "ping":
		return Response{Result: json.RawMessage(`"pong"`)}
	case "merge":
		return withPayload(req, func(p patchPayload) ([]byte, error) {
			return jsonpatch.MergePatch(p.Original, p.Patch)
		})
	case "jsonpatch":
		return withPayload(req, func(p patchPayload) ([]byte, error) {
			ops, err := jsonpatch.DecodePatch(p.Patch)
			if err != nil {
				return nil, err
			}
			return ops.Apply(p.Original)
		})
	case "smp":
		return withPayload(req, smp)
	default:
		return Response{Error: fmt.Sprintf("op %q not implemented", req.Op)}
	}
}

func withPayload(req Request, f func(patchPayload) ([]byte, error)) Response {
	var p patchPayload
	if err := json.Unmarshal(req.Payload, &p); err != nil {
		return Response{Error: "bad payload: " + err.Error()}
	}
	out, err := f(p)
	if err != nil {
		return Response{Error: err.Error()}
	}
	return Response{Result: out}
}

// smp mirrors strategicPatchObject: both sides decoded as the apiserver
// decodes them (case-sensitive, ints preserved), then patched as maps. The
// result is not round-tripped through the typed object; that is decoding,
// a separate concern from patch semantics.
//
// The apiserver builds the original with ToUnstructured, whose slices have
// no spare capacity. SMP appends to and edits original slices in place, so
// capacity changes results; DeepCopyJSONValue gives the same exact sizes.
func smp(p patchPayload) ([]byte, error) {
	gv, err := schema.ParseGroupVersion(p.APIVersion)
	if err != nil {
		return nil, err
	}
	ref, err := scheme.Scheme.New(gv.WithKind(p.Kind))
	if err != nil {
		return nil, err
	}
	decoded := map[string]any{}
	if err := kjson.UnmarshalCaseSensitivePreserveInts(p.Original, &decoded); err != nil {
		return nil, err
	}
	original := runtime.DeepCopyJSONValue(decoded).(map[string]any)
	patch := map[string]any{}
	if err := kjson.UnmarshalCaseSensitivePreserveInts(p.Patch, &patch); err != nil {
		return nil, err
	}
	out, err := strategicpatch.StrategicMergeMapPatch(original, patch, ref)
	if err != nil {
		return nil, err
	}
	return json.Marshal(out)
}

func main() {
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 0, 1<<20), 64<<20)
	out := bufio.NewWriter(os.Stdout)
	enc := json.NewEncoder(out)
	for in.Scan() {
		var req Request
		resp := Response{}
		if err := json.Unmarshal(in.Bytes(), &req); err != nil {
			resp.Error = "bad request: " + err.Error()
		} else {
			resp = handle(req)
		}
		if err := enc.Encode(resp); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		// Flush per response: the client waits for each answer.
		if err := out.Flush(); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
	}
	if err := in.Err(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
