package main

// The service op (README §12.3 step 1): a Service create or update through
// upstream's Service REST storage (pkg/registry/core/service/storage), the
// code behind the apiserver's services endpoint once the body is decoded.
// It defaults ipFamilyPolicy and ipFamilies, allocates cluster IPs and
// validates, in the apiserver's order. Two stand-ins: the store holds one
// object in memory (the old object of an update), and the IP allocator is
// upstream's in-memory bitmap over the given CIDR with `used` reserved.
// The bitmap words a requested IP outside the CIDR differently from the
// 1.37 MetaAllocator, so callers send only IPs inside it.
//
//	payload  {"object": Service, "old": Service?, "cidr": "10.96.0.0/24", "used": ["10.96.0.1"]}
//	result   {"code": 201|200, "object": Service} or {"code": 4xx|5xx, "causes": [...]}

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"

	corev1 "k8s.io/api/core/v1"
	apierrors "k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/api/meta"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	utilnet "k8s.io/apimachinery/pkg/util/net"
	genericapirequest "k8s.io/apiserver/pkg/endpoints/request"
	"k8s.io/apiserver/pkg/registry/generic"
	apirest "k8s.io/apiserver/pkg/registry/rest"
	"k8s.io/apiserver/pkg/storage"
	"k8s.io/apiserver/pkg/storage/storagebackend"
	"k8s.io/apiserver/pkg/storage/storagebackend/factory"
	"k8s.io/client-go/tools/cache"
	"k8s.io/klog/v2"
	"k8s.io/kubernetes/pkg/api/legacyscheme"
	api "k8s.io/kubernetes/pkg/apis/core"
	"k8s.io/kubernetes/pkg/registry/core/service/ipallocator"
	"k8s.io/kubernetes/pkg/registry/core/service/portallocator"
	svcstorage "k8s.io/kubernetes/pkg/registry/core/service/storage"
	netutils "k8s.io/utils/net"
)

type servicePayload struct {
	Object json.RawMessage `json:"object"`
	Old    json.RawMessage `json:"old,omitempty"`
	CIDR   string          `json:"cidr"`
	Used   []string        `json:"used"`
}

// memStore is a storage.Interface holding at most one object. Only what
// the generic store calls for a create or an update is implemented.
type memStore struct {
	storage.Interface
	codec runtime.Codec
	obj   runtime.Object
}

func (m *memStore) Versioner() storage.Versioner { return storage.APIObjectVersioner{} }

func (m *memStore) copyInto(in, out runtime.Object) error {
	data, err := runtime.Encode(m.codec, in)
	if err != nil {
		return err
	}
	_, _, err = m.codec.Decode(data, nil, out)
	return err
}

func (m *memStore) Get(_ context.Context, key string, _ storage.GetOptions, out runtime.Object) error {
	if m.obj == nil {
		return storage.NewKeyNotFoundError(key, 0)
	}
	return m.copyInto(m.obj, out)
}

func (m *memStore) Create(_ context.Context, key string, obj, out runtime.Object, _ uint64) error {
	if m.obj != nil {
		return storage.NewKeyExistsError(key, 0)
	}
	if err := m.Versioner().UpdateObject(obj, 1); err != nil {
		return err
	}
	m.obj = obj.DeepCopyObject()
	return m.copyInto(obj, out)
}

func (m *memStore) GuaranteedUpdate(_ context.Context, key string, dest runtime.Object, _ bool,
	pre *storage.Preconditions, tryUpdate storage.UpdateFunc, _ runtime.Object) error {
	if m.obj == nil {
		return storage.NewKeyNotFoundError(key, 0)
	}
	cur := m.obj.DeepCopyObject()
	if pre != nil {
		if err := pre.Check(key, cur); err != nil {
			return err
		}
	}
	rv, err := m.Versioner().ObjectResourceVersion(cur)
	if err != nil {
		return err
	}
	updated, _, err := tryUpdate(cur, storage.ResponseMeta{ResourceVersion: rv})
	if err != nil {
		return err
	}
	if err := m.Versioner().UpdateObject(updated, rv+1); err != nil {
		return err
	}
	m.obj = updated.DeepCopyObject()
	return m.copyInto(updated, dest)
}

func init() {
	// The allocator logs every allocation; stdout is the protocol and
	// stderr is the fuzzer's report.
	klog.LogToStderr(false)
	klog.SetOutput(io.Discard)
}

func serviceOp(raw json.RawMessage) ([]byte, error) {
	var p servicePayload
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, err
	}
	obj, _, err := internal(p.Object)
	if err != nil {
		return nil, err
	}
	_, cidr, err := netutils.ParseCIDRSloppy(p.CIDR)
	if err != nil {
		return nil, err
	}
	ips, err := ipallocator.NewInMemory(cidr)
	if err != nil {
		return nil, err
	}
	reserve := func(ip string) error {
		if ip == "" || ip == api.ClusterIPNone {
			return nil
		}
		if err := ips.Allocate(netutils.ParseIPSloppy(ip)); err != nil {
			return fmt.Errorf("reserve %s: %w", ip, err)
		}
		return nil
	}
	for _, ip := range p.Used {
		if err := reserve(ip); err != nil {
			return nil, err
		}
	}
	ports, err := portallocator.NewInMemory(utilnet.PortRange{Base: 30000, Size: 2768})
	if err != nil {
		return nil, err
	}
	store := &memStore{codec: legacyscheme.Codecs.LegacyCodec(corev1.SchemeGroupVersion)}
	if p.Old != nil {
		old, _, err := internal(p.Old)
		if err != nil {
			return nil, fmt.Errorf("old: %w", err)
		}
		for _, ip := range old.(*api.Service).Spec.ClusterIPs {
			if err := reserve(ip); err != nil {
				return nil, err
			}
		}
		store.obj = old
	}
	opts := generic.RESTOptions{
		StorageConfig: &storagebackend.ConfigForResource{
			GroupResource: api.Resource("services"),
			Config:        storagebackend.Config{Codec: store.codec},
		},
		Decorator: func(*storagebackend.ConfigForResource, string, func(runtime.Object) (string, error),
			func() runtime.Object, func() runtime.Object, storage.AttrFunc, storage.IndexerFuncs,
			*cache.Indexers) (storage.Interface, factory.DestroyFunc, error) {
			return store, func() {}, nil
		},
		ResourcePrefix: "services",
	}
	rest, _, _, err := svcstorage.NewREST(opts, api.IPv4Protocol,
		map[api.IPFamily]ipallocator.Interface{api.IPv4Protocol: ips}, ports, nil, nil, nil)
	if err != nil {
		return nil, err
	}
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
		IsResourceRequest: true, Verb: verb, APIPrefix: "api", APIVersion: "v1",
		Namespace: m.GetNamespace(), Resource: "services", Name: m.GetName(),
	})
	var out runtime.Object
	code := 201
	if p.Old == nil {
		out, err = rest.Create(ctx, obj, nil, &metav1.CreateOptions{})
	} else {
		code = 200
		out, _, err = rest.Update(ctx, m.GetName(), apirest.DefaultUpdatedObjectInfo(obj), nil, nil, false, &metav1.UpdateOptions{})
	}
	if err != nil {
		var se apierrors.APIStatus
		if !errors.As(err, &se) {
			return nil, err
		}
		st := se.Status()
		causes := []cause{}
		if st.Reason == metav1.StatusReasonInvalid && st.Details != nil {
			for _, c := range st.Details.Causes {
				causes = append(causes, cause{string(c.Type), c.Field, c.Message})
			}
		}
		return json.Marshal(map[string]any{"code": st.Code, "causes": causes, "message": st.Message})
	}
	v1, err := legacyscheme.Scheme.ConvertToVersion(out, corev1.SchemeGroupVersion)
	if err != nil {
		return nil, err
	}
	return json.Marshal(map[string]any{"code": code, "object": v1})
}
