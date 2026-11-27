package main

import (
	"encoding/json"
	"testing"
)

func call(t *testing.T, op string, p patchPayload) Response {
	t.Helper()
	raw, err := json.Marshal(p)
	if err != nil {
		t.Fatal(err)
	}
	return handle(Request{Op: op, Payload: raw})
}

func assertJSON(t *testing.T, got Response, want string) {
	t.Helper()
	if got.Error != "" {
		t.Fatalf("unexpected error: %s", got.Error)
	}
	var g, w any
	if err := json.Unmarshal(got.Result, &g); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal([]byte(want), &w); err != nil {
		t.Fatal(err)
	}
	gb, _ := json.Marshal(g)
	wb, _ := json.Marshal(w)
	if string(gb) != string(wb) {
		t.Fatalf("got %s, want %s", gb, wb)
	}
}

func TestPing(t *testing.T) {
	if got := handle(Request{Op: "ping"}); got.Error != "" || string(got.Result) != `"pong"` {
		t.Fatalf("unexpected response: %+v", got)
	}
}

func TestUnknownOp(t *testing.T) {
	if got := handle(Request{Op: "nope"}); got.Error == "" {
		t.Fatal("expected an error for an unknown op")
	}
}

func TestMerge(t *testing.T) {
	got := call(t, "merge", patchPayload{
		Original: json.RawMessage(`{"a":1,"b":{"c":2,"d":3},"e":[1]}`),
		Patch:    json.RawMessage(`{"a":null,"b":{"c":null,"x":{"y":null}},"e":[null]}`),
	})
	assertJSON(t, got, `{"b":{"d":3,"x":{}},"e":[null]}`)
}

func TestMergeScalarPatchIsAnError(t *testing.T) {
	got := call(t, "merge", patchPayload{Original: json.RawMessage(`{}`), Patch: json.RawMessage(`1`)})
	if got.Error != "Invalid JSON Patch" {
		t.Fatalf("got %+v", got)
	}
}

func TestJSONPatch(t *testing.T) {
	got := call(t, "jsonpatch", patchPayload{
		Original: json.RawMessage(`{"a":[1,2]}`),
		Patch:    json.RawMessage(`[{"op":"add","path":"/a/1","value":9},{"op":"remove","path":"/a/0"}]`),
	})
	assertJSON(t, got, `{"a":[9,2]}`)
}

// Containers merge by name: the SMP behaviour a plain merge patch lacks.
func TestSMPMergesContainersByName(t *testing.T) {
	got := call(t, "smp", patchPayload{
		APIVersion: "v1",
		Kind:       "Pod",
		Original:   json.RawMessage(`{"spec":{"containers":[{"name":"a","image":"x"},{"name":"b","image":"y"}]}}`),
		Patch:      json.RawMessage(`{"spec":{"containers":[{"name":"b","image":"z"}]}}`),
	})
	assertJSON(t, got, `{"spec":{"containers":[{"name":"a","image":"x"},{"name":"b","image":"z"}]}}`)
}

func TestSMPDeleteDirective(t *testing.T) {
	got := call(t, "smp", patchPayload{
		APIVersion: "apps/v1",
		Kind:       "Deployment",
		Original:   json.RawMessage(`{"spec":{"template":{"spec":{"containers":[{"name":"a"},{"name":"b"}]}}}}`),
		Patch:      json.RawMessage(`{"spec":{"template":{"spec":{"containers":[{"name":"a","$patch":"delete"}]}}}}`),
	})
	assertJSON(t, got, `{"spec":{"template":{"spec":{"containers":[{"name":"b"}]}}}}`)
}

func TestSMPUnknownKind(t *testing.T) {
	got := call(t, "smp", patchPayload{APIVersion: "v1", Kind: "Nope", Original: json.RawMessage(`{}`), Patch: json.RawMessage(`{}`)})
	if got.Error == "" {
		t.Fatal("expected an error for an unknown kind")
	}
}
