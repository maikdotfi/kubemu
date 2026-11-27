package main

import "testing"

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
