package main

// openapi-v2-pb: an OpenAPI v2 document (the payload, as JSON) as the
// apiserver serves it to clients asking for
// application/com.github.proto-openapi.spec.v2@v1.0+protobuf (ADR 0014):
// kube-openapi's ToProtoBinary, gnostic's Document marshalled. The result
// is the bytes (base64, as encoding/json writes []byte).

import (
	"encoding/json"

	openapi_v2 "github.com/google/gnostic-models/openapiv2"
	"google.golang.org/protobuf/proto"
)

func openAPIV2PB(raw json.RawMessage) ([]byte, error) {
	doc, err := openapi_v2.ParseDocument(raw)
	if err != nil {
		return nil, err
	}
	out, err := proto.MarshalOptions{Deterministic: true}.Marshal(doc)
	if err != nil {
		return nil, err
	}
	return json.Marshal(out)
}
