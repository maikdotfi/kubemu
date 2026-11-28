package main

// The protobuf wire table (ADR 0014): what kubemu-proto needs to turn a
// protobuf request body into the JSON Go would produce from it, read off
// the Go types themselves by reflection (`go run . wiretable`).
//
// The .proto files say how fields are numbered and encoded, but not what
// Go's JSON encoding then does: which fields are pointers, which are
// omitempty, which types marshal themselves (Time, Quantity, ...), and
// which embedded structs are inlined. The table carries all of that.
//
// Format (one message per line, so diffs stay readable):
//
//	{"kubernetes": "v1.37.3",
//	 "kinds":    {"<apiVersion>/<Kind>": "<message>", ...},
//	 "messages": {"<message>": [<field>, ...], ...}}
//
// A field is {"n": number, "j": json name ("" = inlined), "t": type} plus
// "p" (pointer), "o" (omitempty) and "z" (omitzero) when true. n is 0 for a
// JSON field with no protobuf tag: never on the wire, but Go still encodes
// its zero value. A type is a scalar name (string bool int32 int64 uint32
// uint64 double bytes), a self-marshalling type (time microtime duration
// quantity intorstring rawext fieldsv1), or {"msg": name}, {"list": type}
// (a repeated field), {"wrap": type} (a named slice with its own message,
// items in field 1, JSON as the bare list) or {"map": [key, value]}.

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"reflect"
	"runtime/debug"
	"sort"
	"strconv"
	"strings"

	"k8s.io/apimachinery/pkg/api/resource"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/util/intstr"
	"k8s.io/client-go/kubernetes/scheme"
)

type wireField struct {
	N int    `json:"n"`
	J string `json:"j"`
	T any    `json:"t"`
	P bool   `json:"p,omitempty"`
	O bool   `json:"o,omitempty"`
	Z bool   `json:"z,omitempty"`
}

var special = map[reflect.Type]string{
	reflect.TypeFor[metav1.Time]():          "time",
	reflect.TypeFor[metav1.MicroTime]():     "microtime",
	reflect.TypeFor[metav1.Duration]():      "duration",
	reflect.TypeFor[resource.Quantity]():    "quantity",
	reflect.TypeFor[intstr.IntOrString]():   "intorstring",
	reflect.TypeFor[runtime.RawExtension](): "rawext",
	reflect.TypeFor[metav1.FieldsV1]():      "fieldsv1",
}

var (
	jsonMarshaler  = reflect.TypeFor[json.Marshaler]()
	protoMarshaler = reflect.TypeFor[interface{ Marshal() ([]byte, error) }]()
)

type tableGen struct {
	messages map[string][]wireField
}

// msgName is a short, stable name for a Go struct type.
func msgName(t reflect.Type) string {
	p := t.PkgPath()
	for _, prefix := range []string{"k8s.io/api/", "k8s.io/apimachinery/pkg/apis/", "k8s.io/apimachinery/pkg/"} {
		p = strings.TrimPrefix(p, prefix)
	}
	return p + "." + t.Name()
}

// typ describes a Go type. Wire encodings follow from the Go kind, as the
// gogo-generated code encodes them; the struct tag's wire type is not
// reliable (ListMeta.remainingItemCount is tagged bytes, sent as varint).
func (g *tableGen) typ(t reflect.Type, where string) any {
	if t.Kind() == reflect.Pointer {
		panic(fmt.Sprintf("%s: pointer inside a container (%s)", where, t))
	}
	if s, ok := special[t]; ok {
		return s
	}
	if t.Kind() == reflect.Slice && t.Elem().Kind() == reflect.Uint8 {
		return "bytes"
	}
	if reflect.PointerTo(t).Implements(jsonMarshaler) || t.Implements(jsonMarshaler) {
		panic(fmt.Sprintf("%s: %s marshals its own JSON; add it to special", where, t))
	}
	if t.Kind() == reflect.Slice {
		if t.Name() != "" && reflect.PointerTo(t).Implements(protoMarshaler) {
			return map[string]any{"wrap": g.typ(t.Elem(), where)}
		}
		return map[string]any{"list": g.typ(t.Elem(), where)}
	}
	if t.Kind() == reflect.Map {
		if t.Key().Kind() != reflect.String {
			panic(fmt.Sprintf("%s: map key %s", where, t.Key()))
		}
		return map[string]any{"map": []any{"string", g.typ(t.Elem(), where)}}
	}
	switch t.Kind() {
	case reflect.String:
		return "string"
	case reflect.Bool:
		return "bool"
	case reflect.Int32:
		return "int32"
	case reflect.Int64:
		return "int64"
	case reflect.Uint32:
		return "uint32"
	case reflect.Uint64:
		return "uint64"
	case reflect.Float64:
		return "double"
	case reflect.Struct:
		g.message(t)
		return map[string]any{"msg": msgName(t)}
	}
	panic(fmt.Sprintf("%s: unsupported type %s (%s)", where, t, t.Kind()))
}

func (g *tableGen) message(t reflect.Type) {
	name := msgName(t)
	if _, done := g.messages[name]; done {
		return
	}
	g.messages[name] = nil // cycles (JSONSchemaProps-style) stop here
	var fields []wireField
	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		if !f.IsExported() {
			continue
		}
		where := name + "." + f.Name
		jtag := f.Tag.Get("json")
		if jtag == "-" {
			continue
		}
		jname, opts, _ := strings.Cut(jtag, ",")
		inline := f.Anonymous && jname == ""
		if !inline && jname == "" {
			jname = f.Name
		}
		ptag := f.Tag.Get("protobuf")
		ft := f.Type
		wf := wireField{J: jname}
		if inline {
			wf.J = ""
		}
		for _, o := range strings.Split(opts, ",") {
			wf.O = wf.O || o == "omitempty"
			wf.Z = wf.Z || o == "omitzero"
		}
		if ft.Kind() == reflect.Pointer {
			wf.P = true
			ft = ft.Elem()
			if inline {
				panic(where + ": inlined pointer")
			}
		}
		if ptag != "" {
			parts := strings.Split(ptag, ",")
			n, err := strconv.Atoi(parts[1])
			if err != nil {
				panic(where + ": bad protobuf tag " + ptag)
			}
			wf.N = n
		} else if inline {
			// TypeMeta: apiVersion and kind come from the envelope.
			if ft != reflect.TypeFor[metav1.TypeMeta]() {
				panic(where + ": inlined struct with no protobuf tag")
			}
			continue
		}
		wf.T = g.typ(ft, where)
		if inline {
			if _, isMsg := wf.T.(map[string]any)["msg"]; !isMsg {
				panic(where + ": inlined non-struct")
			}
		}
		fields = append(fields, wf)
	}
	sort.SliceStable(fields, func(a, b int) bool { return fields[a].N < fields[b].N })
	g.messages[name] = fields
}

func kubernetesVersion() string {
	bi, ok := debug.ReadBuildInfo()
	if ok {
		for _, d := range bi.Deps {
			if d.Path == "k8s.io/api" {
				return strings.Replace(d.Version, "v0.", "v1.", 1)
			}
		}
	}
	return "unknown"
}

// wireTable renders the table for every type client-go's scheme knows.
func wireTable(version string) []byte {
	g := &tableGen{messages: map[string][]wireField{}}
	kinds := map[string]string{}
	for gvk, t := range scheme.Scheme.AllKnownTypes() {
		// Only types with a protobuf encoding (not InternalEvent and the
		// like, which never go on the wire).
		if t.Kind() != reflect.Struct || !reflect.PointerTo(t).Implements(protoMarshaler) {
			continue
		}
		g.message(t)
		kinds[gvk.GroupVersion().String()+"/"+gvk.Kind] = msgName(t)
	}
	var b bytes.Buffer
	line := func(prefix string, keys []string, val func(string) any, last bool) {
		for i, k := range keys {
			kj, _ := json.Marshal(k)
			vj, err := json.Marshal(val(k))
			if err != nil {
				panic(err)
			}
			sep := ","
			if i == len(keys)-1 {
				sep = ""
			}
			fmt.Fprintf(&b, "%s%s: %s%s\n", prefix, kj, vj, sep)
		}
		if last {
			b.WriteString(" }\n")
		} else {
			b.WriteString(" },\n")
		}
	}
	fmt.Fprintf(&b, "{\"kubernetes\": %q,\n \"kinds\": {\n", version)
	line("  ", sortedKeys(kinds), func(k string) any { return kinds[k] }, false)
	b.WriteString(" \"messages\": {\n")
	line("  ", sortedKeys(g.messages), func(k string) any {
		if g.messages[k] == nil {
			return []wireField{}
		}
		return g.messages[k]
	}, true)
	b.WriteString("}\n")
	return b.Bytes()
}

func sortedKeys[V any](m map[string]V) []string {
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	return keys
}

func runWireTable() {
	os.Stdout.Write(wireTable(kubernetesVersion()))
}
