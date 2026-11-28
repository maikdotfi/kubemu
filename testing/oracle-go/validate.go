package main

// Defaulting, validation and quantities (ADR 0013), from the code the
// apiserver runs on a write:
//
//	quantity  resource.Quantity's JSON round trip, optionally RoundUp(milli)
//	          as SetDefaults_ResourceList does
//	default   typed decode (case-sensitive, as the apiserver's codec), then
//	          the scheme's defaulters; returns both so kubemu can be checked
//	          against the defaulting alone, not the typed round trip
//	validate  default, convert to the internal type, then rest.BeforeCreate
//	          (or BeforeUpdate) with the resource's registry strategy:
//	          PrepareForCreate, strategy validation, object meta validation
//	          and declarative validation, exactly as a create or update
//	strategy  per-kind registry strategy answers kubemu guessed (README §12.2)

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	apierrors "k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/api/meta"
	"k8s.io/apimachinery/pkg/api/resource"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/runtime/schema"
	"k8s.io/apimachinery/pkg/types"
	genericapirequest "k8s.io/apiserver/pkg/endpoints/request"
	"k8s.io/apiserver/pkg/registry/rest"
	"k8s.io/kubernetes/pkg/api/legacyscheme"
	"k8s.io/kubernetes/pkg/registry/core/configmap"
	"k8s.io/kubernetes/pkg/registry/core/endpoint"
	"k8s.io/kubernetes/pkg/registry/core/event"
	"k8s.io/kubernetes/pkg/registry/core/namespace"
	"k8s.io/kubernetes/pkg/registry/core/node"
	"k8s.io/kubernetes/pkg/registry/core/pod"
	"k8s.io/kubernetes/pkg/registry/core/secret"
	"k8s.io/kubernetes/pkg/registry/core/service"
	"k8s.io/kubernetes/pkg/registry/core/serviceaccount"
	"k8s.io/kubernetes/pkg/registry/discovery/endpointslice"
	kjson "sigs.k8s.io/json"

	_ "k8s.io/kubernetes/pkg/apis/core/install"
	_ "k8s.io/kubernetes/pkg/apis/discovery/install"
)

// strategy is what the apiserver's generic store calls on writes.
type strategy interface {
	rest.RESTCreateStrategy
	rest.RESTUpdateStrategy
}

// strategies are keyed by "group/Kind" for the kinds kubemu stores.
var strategies = map[string]struct {
	plural string
	s      strategy
}{
	"/ConfigMap":                     {"configmaps", configmap.Strategy},
	"/Endpoints":                     {"endpoints", endpoint.Strategy},
	"/Event":                         {"events", event.Strategy},
	"/Namespace":                     {"namespaces", namespace.Strategy},
	"/Node":                          {"nodes", node.Strategy},
	"/Pod":                           {"pods", pod.Strategy},
	"/Secret":                        {"secrets", secret.Strategy},
	"/Service":                       {"services", service.Strategy},
	"/ServiceAccount":                {"serviceaccounts", serviceaccount.Strategy},
	"discovery.k8s.io/EndpointSlice": {"endpointslices", endpointslice.Strategy},
}

type quantityPayload struct {
	// The JSON value as sent: a string or a number.
	Value   json.RawMessage `json:"value"`
	RoundUp bool            `json:"roundUp,omitempty"`
}

func quantity(raw json.RawMessage) ([]byte, error) {
	var p quantityPayload
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, err
	}
	var q resource.Quantity
	if err := q.UnmarshalJSON(p.Value); err != nil {
		return nil, err
	}
	if p.RoundUp {
		q.RoundUp(-3)
	}
	return q.MarshalJSON()
}

// decodeTyped decodes a versioned object as the apiserver's JSON codec
// does, without defaulting.
func decodeTyped(raw json.RawMessage) (runtime.Object, schema.GroupVersionKind, error) {
	var tm metav1.TypeMeta
	if err := json.Unmarshal(raw, &tm); err != nil {
		return nil, schema.GroupVersionKind{}, err
	}
	gvk := tm.GroupVersionKind()
	obj, err := legacyscheme.Scheme.New(gvk)
	if err != nil {
		return nil, gvk, err
	}
	if err := kjson.UnmarshalCaseSensitivePreserveInts(raw, obj); err != nil {
		return nil, gvk, err
	}
	return obj, gvk, nil
}

type objectPayload struct {
	Object json.RawMessage `json:"object"`
	// Old is the stored object for an update; absent for a create.
	Old json.RawMessage `json:"old,omitempty"`
}

func defaults(raw json.RawMessage) ([]byte, error) {
	var p objectPayload
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, err
	}
	obj, _, err := decodeTyped(p.Object)
	if err != nil {
		return nil, err
	}
	baseline, err := json.Marshal(obj)
	if err != nil {
		return nil, err
	}
	legacyscheme.Scheme.Default(obj)
	defaulted, err := json.Marshal(obj)
	if err != nil {
		return nil, err
	}
	return json.Marshal(map[string]json.RawMessage{"baseline": baseline, "defaulted": defaulted})
}

// internal decodes, defaults and converts to the internal version, which
// is what strategies and validation see.
func internal(raw json.RawMessage) (runtime.Object, schema.GroupVersionKind, error) {
	obj, gvk, err := decodeTyped(raw)
	if err != nil {
		return nil, gvk, err
	}
	legacyscheme.Scheme.Default(obj)
	in, err := legacyscheme.Scheme.ConvertToVersion(obj, schema.GroupVersion{Group: gvk.Group, Version: runtime.APIVersionInternal})
	if err != nil {
		return nil, gvk, err
	}
	return in, gvk, nil
}

// cause is one field error, as the apiserver reports it in a Status.
type cause struct {
	Reason  string `json:"reason"`
	Field   string `json:"field"`
	Message string `json:"message"`
}

func validate(raw json.RawMessage) ([]byte, error) {
	var p objectPayload
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, err
	}
	obj, gvk, err := internal(p.Object)
	if err != nil {
		return nil, err
	}
	st, ok := strategies[gvk.Group+"/"+gvk.Kind]
	if !ok {
		return nil, fmt.Errorf("no strategy for %s", gvk)
	}
	s := st.s
	m, err := meta.Accessor(obj)
	if err != nil {
		return nil, err
	}
	verb := "create"
	if p.Old != nil {
		verb = "update"
	}
	ctx := genericapirequest.WithNamespace(context.Background(), m.GetNamespace())
	ctx = genericapirequest.WithRequestInfo(ctx, &genericapirequest.RequestInfo{
		IsResourceRequest: true,
		Verb:              verb,
		APIPrefix:         "api",
		APIGroup:          gvk.Group,
		APIVersion:        gvk.Version,
		Namespace:         m.GetNamespace(),
		Resource:          st.plural,
		Name:              m.GetName(),
	})
	if p.Old == nil {
		// The generic store fills these before BeforeCreate.
		m.SetUID(types.UID("00000000-0000-4000-8000-000000000000"))
		m.SetCreationTimestamp(metav1.Unix(1, 0))
		err = rest.BeforeCreate(s, ctx, obj)
	} else {
		var old runtime.Object
		if old, _, err = internal(p.Old); err != nil {
			return nil, fmt.Errorf("old: %w", err)
		}
		err = rest.BeforeUpdate(s, ctx, obj, old)
	}
	causes := []cause{}
	if err != nil {
		var se apierrors.APIStatus
		if !errors.As(err, &se) || se.Status().Reason != metav1.StatusReasonInvalid {
			return nil, err
		}
		if d := se.Status().Details; d != nil {
			for _, c := range d.Causes {
				causes = append(causes, cause{string(c.Type), c.Field, c.Message})
			}
		}
	}
	return json.Marshal(causes)
}

type strategyPayload struct {
	Group string `json:"group"`
	Kind  string `json:"kind"`
}

func strategyInfo(raw json.RawMessage) ([]byte, error) {
	var p strategyPayload
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, err
	}
	st, ok := strategies[p.Group+"/"+p.Kind]
	if !ok {
		return nil, fmt.Errorf("no strategy for %s/%s", p.Group, p.Kind)
	}
	s := st.s
	return json.Marshal(map[string]any{
		"namespaceScoped":          s.NamespaceScoped(),
		"allowCreateOnUpdate":      s.AllowCreateOnUpdate(context.Background()),
		"allowUnconditionalUpdate": s.AllowUnconditionalUpdate(context.Background()),
	})
}
