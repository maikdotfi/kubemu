// Command oracle-go answers kubemu's differential-fuzzing questions using real
// Kubernetes Go code (README §1.4, §10).
//
// Protocol: one JSON request per line on stdin, one JSON response per line on
// stdout. The Rust fuzzers spawn this process and diff its answers against
// kubemu's.
//
// Planned ops (k8s.io/apimachinery and sigs.k8s.io/structured-merge-diff get
// pinned to the version chosen in ADR 0007):
//
//	smp        strategic merge patch
//	jsonpatch  RFC 6902 JSON patch
//	merge      RFC 7386 merge patch
//	ssa        server-side apply with managed fields
//	validate   built-in validation and defaulting
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
)

type Request struct {
	Op      string          `json:"op"`
	Payload json.RawMessage `json:"payload,omitempty"`
}

type Response struct {
	Result json.RawMessage `json:"result,omitempty"`
	Error  string          `json:"error,omitempty"`
}

func handle(req Request) Response {
	switch req.Op {
	case "ping":
		return Response{Result: json.RawMessage(`"pong"`)}
	default:
		return Response{Error: fmt.Sprintf("op %q not implemented", req.Op)}
	}
}

func main() {
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 0, 1<<20), 64<<20)
	out := json.NewEncoder(os.Stdout)
	for in.Scan() {
		var req Request
		resp := Response{}
		if err := json.Unmarshal(in.Bytes(), &req); err != nil {
			resp.Error = "bad request: " + err.Error()
		} else {
			resp = handle(req)
		}
		if err := out.Encode(resp); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
	}
	if err := in.Err(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
