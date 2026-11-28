// Command informer-go is the client-go informer harness (README §10, §12.1
// step 8). It starts `kubemu up` with a short watch log, runs shared
// informers on client-go's default settings against it while writers churn
// objects and a seeded fault injector cuts, delays and fails list and watch
// requests. Then it checks that every informer's cache converges on what
// the server lists, that the state rebuilt from its event handlers matches
// the cache, that handlers never see an object's resourceVersion go
// backwards or an add for an object they already have, and that the run
// exercised what it claims: watch-lists, resumed watches, 410 Gone followed
// by relists, and bookmarks.
//
// Usage:
//
//	go run . -kubemu ../../target/debug/kubemu [-seed N] [-duration 20s]
package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"math/rand"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	corev1 "k8s.io/api/core/v1"
	apierrors "k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/api/meta"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/types"
	"k8s.io/client-go/informers"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/cache"
	"k8s.io/client-go/tools/clientcmd"
)

var (
	kubemuBin = flag.String("kubemu", "", "path to the kubemu binary (required)")
	seed      = flag.Int64("seed", 1, "seed for the writers and the fault injector")
	duration  = flag.Duration("duration", 20*time.Second, "how long writers and faults run")
	watchLog  = flag.Int("watch-log", 200, "kubemu's watch log length; short, so resumes hit 410")
	writers   = flag.Int("writers", 4, "concurrent writers")
	settle    = flag.Duration("settle", 60*time.Second, "how long caches may take to converge after the faults stop")
)

var namespaces = []string{"churn-0", "churn-1", "churn-2"}

func main() {
	flag.Parse()
	if *kubemuBin == "" {
		fmt.Fprintln(os.Stderr, "informer-go: -kubemu is required")
		os.Exit(2)
	}
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "INFORMER HARNESS FAIL (seed %d): %v\n", *seed, err)
		os.Exit(1)
	}
}

func run() error {
	work, err := os.MkdirTemp("", "informer-go")
	if err != nil {
		return err
	}
	defer os.RemoveAll(work)
	kubeconfig := filepath.Join(work, "kubeconfig")
	stop, err := startKubemu(kubeconfig)
	if err != nil {
		return err
	}
	defer stop()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Writers and checks use a clean client without client-side rate limits.
	admin, err := clientcmd.BuildConfigFromFlags("", kubeconfig)
	if err != nil {
		return err
	}
	admin.QPS = -1
	adminClient := kubernetes.NewForConfigOrDie(admin)
	for _, ns := range namespaces {
		_, err := adminClient.CoreV1().Namespaces().Create(ctx, &corev1.Namespace{ObjectMeta: metav1.ObjectMeta{Name: ns}}, metav1.CreateOptions{})
		if err != nil {
			return fmt.Errorf("create namespace %s: %w", ns, err)
		}
	}

	// Informers get client-go's defaults (protobuf request bodies, JSON
	// responses, default rate limits, whatever list/watch strategy the
	// reflector picks), plus the fault injector.
	cfg, err := clientcmd.BuildConfigFromFlags("", kubeconfig)
	if err != nil {
		return err
	}
	inj := &injector{rng: rand.New(rand.NewSource(*seed))}
	inj.enabled.Store(true)
	cfg.Wrap(func(rt http.RoundTripper) http.RoundTripper { inj.next = rt; return inj })
	client := kubernetes.NewForConfigOrDie(cfg)

	all := informers.NewSharedInformerFactory(client, 0)
	hot := informers.NewSharedInformerFactoryWithOptions(client, 0,
		informers.WithTweakListOptions(func(o *metav1.ListOptions) { o.LabelSelector = "hot=true" }))
	one := informers.NewSharedInformerFactoryWithOptions(client, 0, informers.WithNamespace(namespaces[0]))
	watched := []*watchedInformer{
		newWatched("configmaps", all.Core().V1().ConfigMaps().Informer(), func(ctx context.Context) ([]metav1.Object, error) {
			l, err := adminClient.CoreV1().ConfigMaps("").List(ctx, metav1.ListOptions{})
			return objects(l, err)
		}),
		newWatched("configmaps hot=true", hot.Core().V1().ConfigMaps().Informer(), func(ctx context.Context) ([]metav1.Object, error) {
			l, err := adminClient.CoreV1().ConfigMaps("").List(ctx, metav1.ListOptions{LabelSelector: "hot=true"})
			return objects(l, err)
		}),
		newWatched("configmaps in "+namespaces[0], one.Core().V1().ConfigMaps().Informer(), func(ctx context.Context) ([]metav1.Object, error) {
			l, err := adminClient.CoreV1().ConfigMaps(namespaces[0]).List(ctx, metav1.ListOptions{})
			return objects(l, err)
		}),
		newWatched("pods", all.Core().V1().Pods().Informer(), func(ctx context.Context) ([]metav1.Object, error) {
			l, err := adminClient.CoreV1().Pods("").List(ctx, metav1.ListOptions{})
			return objects(l, err)
		}),
		newWatched("namespaces", all.Core().V1().Namespaces().Informer(), func(ctx context.Context) ([]metav1.Object, error) {
			l, err := adminClient.CoreV1().Namespaces().List(ctx, metav1.ListOptions{})
			return objects(l, err)
		}),
	}
	for _, f := range []informers.SharedInformerFactory{all, hot, one} {
		f.Start(ctx.Done())
	}
	syncCtx, syncCancel := context.WithTimeout(ctx, 30*time.Second)
	defer syncCancel()
	for _, w := range watched {
		if !cache.WaitForCacheSync(syncCtx.Done(), w.inf.HasSynced) {
			return fmt.Errorf("%s: never synced", w.name)
		}
	}

	var writes atomic.Int64
	var wg sync.WaitGroup
	deadline := time.Now().Add(*duration)
	for i := 0; i < *writers; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			write(ctx, adminClient, rand.New(rand.NewSource(*seed*1000+int64(i))), deadline, &writes)
		}(i)
	}
	wg.Wait()
	inj.enabled.Store(false)

	// Converge: caches must match the server once faults stop. Pods keep
	// changing for a while (the simulated kubelet), so poll.
	var last error
	until := time.Now().Add(*settle)
	for time.Now().Before(until) {
		last = nil
		for _, w := range watched {
			if err := w.check(ctx); err != nil {
				last = err
				break
			}
		}
		if last == nil {
			break
		}
		time.Sleep(200 * time.Millisecond)
	}
	if last != nil {
		return fmt.Errorf("caches did not converge within %v: %w", *settle, last)
	}
	var violations []string
	for _, w := range watched {
		violations = append(violations, w.violations()...)
	}
	if len(violations) > 0 {
		return fmt.Errorf("event handler violations:\n  %s", strings.Join(violations, "\n  "))
	}

	s := inj.snapshot()
	syncs := s.lists + s.watchLists
	summary := fmt.Sprintf("seed %d, %d writes, %d lists, %d watches (%d watch-lists), %d relists, %d 410s, %d bookmarks (%d initial-events-end), faults: %d cut, %d delayed, %d failed",
		*seed, writes.Load(), s.lists, s.watches, s.watchLists, syncs-int64(len(watched)), s.gone, s.bookmarks, s.initialEnd, s.cut, s.delayed, s.failed)
	switch {
	case s.initialEnd == 0:
		// client-go 1.37 defaults to WatchListClient and falls back to a
		// plain list only when a watch-list fails.
		return fmt.Errorf("no watch-list completed (%s)", summary)
	case syncs <= int64(len(watched)):
		return fmt.Errorf("no relist happened (%s)", summary)
	case s.gone == 0:
		return fmt.Errorf("no watch resumed into 410 Gone (%s)", summary)
	case s.bookmarks == 0:
		return fmt.Errorf("no bookmarks (%s)", summary)
	case s.resumed == 0:
		return fmt.Errorf("no watch resumed from a resourceVersion (%s)", summary)
	}
	fmt.Printf("informer harness: ok (%s)\n", summary)
	return nil
}

// startKubemu runs `kubemu up` on a free port and waits until it serves.
func startKubemu(kubeconfig string) (func(), error) {
	cmd := exec.Command(*kubemuBin, "up", "--port", "0", "--cluster", "informer",
		"--kubeconfig", kubeconfig, "--seed", strconv.FormatInt(*seed, 10),
		"--watch-log", strconv.Itoa(*watchLog))
	stderr, err := cmd.StderrPipe()
	if err != nil {
		return nil, err
	}
	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("start %s: %w", *kubemuBin, err)
	}
	stop := func() { _ = cmd.Process.Kill(); _ = cmd.Wait() }
	lines := bufio.NewScanner(stderr)
	for lines.Scan() {
		fmt.Fprintln(os.Stderr, lines.Text())
		if strings.Contains(lines.Text(), "serving on") {
			go func() {
				for lines.Scan() {
					fmt.Fprintln(os.Stderr, lines.Text())
				}
			}()
			return stop, nil
		}
	}
	stop()
	return nil, errors.New("kubemu up exited before serving")
}

// write churns ConfigMaps (create, update, label flips in and out of the
// hot=true selection, delete) and pods until the deadline.
func write(ctx context.Context, c kubernetes.Interface, rng *rand.Rand, deadline time.Time, n *atomic.Int64) {
	for time.Now().Before(deadline) {
		ns := namespaces[rng.Intn(len(namespaces))]
		name := fmt.Sprintf("cm-%d", rng.Intn(40))
		cms := c.CoreV1().ConfigMaps(ns)
		var err error
		switch op := rng.Intn(10); {
		case op < 3:
			_, err = cms.Create(ctx, &corev1.ConfigMap{
				ObjectMeta: metav1.ObjectMeta{Name: name, Labels: map[string]string{"hot": strconv.FormatBool(rng.Intn(2) == 0)}},
				Data:       map[string]string{"n": "0"},
			}, metav1.CreateOptions{})
		case op < 5:
			var cm *corev1.ConfigMap
			if cm, err = cms.Get(ctx, name, metav1.GetOptions{}); err == nil {
				cm.Data = map[string]string{"n": strconv.Itoa(rng.Int())}
				_, err = cms.Update(ctx, cm, metav1.UpdateOptions{})
			}
		case op < 7:
			patch := fmt.Sprintf(`{"metadata":{"labels":{"hot":"%t"}}}`, rng.Intn(2) == 0)
			_, err = cms.Patch(ctx, name, types.MergePatchType, []byte(patch), metav1.PatchOptions{})
		case op < 9:
			err = cms.Delete(ctx, name, metav1.DeleteOptions{})
		default:
			pods := c.CoreV1().Pods(ns)
			pod := fmt.Sprintf("pod-%d", rng.Intn(5))
			if rng.Intn(2) == 0 {
				_, err = pods.Create(ctx, &corev1.Pod{
					ObjectMeta: metav1.ObjectMeta{Name: pod},
					Spec:       corev1.PodSpec{Containers: []corev1.Container{{Name: "c", Image: "busybox"}}},
				}, metav1.CreateOptions{})
			} else {
				err = pods.Delete(ctx, pod, metav1.DeleteOptions{})
			}
		}
		switch {
		case err == nil:
			n.Add(1)
		case apierrors.IsNotFound(err), apierrors.IsAlreadyExists(err), apierrors.IsConflict(err):
		default:
			fmt.Fprintf(os.Stderr, "informer-go: write: %v\n", err)
		}
	}
}

// watchedInformer is one informer plus the state its handlers rebuilt.
type watchedInformer struct {
	name   string
	inf    cache.SharedIndexInformer
	server func(context.Context) ([]metav1.Object, error)

	mu     sync.Mutex
	mirror map[string]uint64 // key -> resourceVersion, from handler calls
	bad    []string
}

func newWatched(name string, inf cache.SharedIndexInformer, server func(context.Context) ([]metav1.Object, error)) *watchedInformer {
	w := &watchedInformer{name: name, inf: inf, server: server, mirror: map[string]uint64{}}
	_, err := inf.AddEventHandler(cache.ResourceEventHandlerFuncs{
		AddFunc: func(obj any) {
			w.mu.Lock()
			defer w.mu.Unlock()
			key, rv := w.keyRV(obj)
			if old, ok := w.mirror[key]; ok {
				w.violate("add of %s at rv %d, already have rv %d", key, rv, old)
			}
			w.mirror[key] = rv
		},
		UpdateFunc: func(oldObj, obj any) {
			w.mu.Lock()
			defer w.mu.Unlock()
			key, rv := w.keyRV(obj)
			if _, oldRV := w.keyRV(oldObj); rv < oldRV {
				w.violate("update of %s from rv %d back to %d", key, oldRV, rv)
			}
			if have, ok := w.mirror[key]; !ok {
				w.violate("update of %s, which handlers never added", key)
			} else if rv < have {
				w.violate("update of %s to rv %d, handlers saw rv %d", key, rv, have)
			}
			w.mirror[key] = rv
		},
		DeleteFunc: func(obj any) {
			w.mu.Lock()
			defer w.mu.Unlock()
			if t, ok := obj.(cache.DeletedFinalStateUnknown); ok {
				obj = t.Obj
			}
			key, _ := w.keyRV(obj)
			if _, ok := w.mirror[key]; !ok {
				w.violate("delete of %s, which handlers never added", key)
			}
			delete(w.mirror, key)
		},
	})
	if err != nil {
		panic(err)
	}
	return w
}

func (w *watchedInformer) violate(format string, args ...any) {
	if len(w.bad) < 20 {
		w.bad = append(w.bad, w.name+": "+fmt.Sprintf(format, args...))
	}
}

func (w *watchedInformer) violations() []string {
	w.mu.Lock()
	defer w.mu.Unlock()
	return append([]string(nil), w.bad...)
}

func (w *watchedInformer) keyRV(obj any) (string, uint64) {
	m, err := meta.Accessor(obj)
	if err != nil {
		panic(err)
	}
	rv, err := strconv.ParseUint(m.GetResourceVersion(), 10, 64)
	if err != nil {
		w.violate("object %s/%s has resourceVersion %q", m.GetNamespace(), m.GetName(), m.GetResourceVersion())
	}
	return key(m), rv
}

// check compares the cache with a fresh list, and the handlers' state with
// the cache.
func (w *watchedInformer) check(ctx context.Context) error {
	objs, err := w.server(ctx)
	if err != nil {
		return fmt.Errorf("%s: list: %w", w.name, err)
	}
	want := map[string]string{}
	for _, o := range objs {
		want[key(o)] = o.GetResourceVersion()
	}
	got := map[string]string{}
	for _, obj := range w.inf.GetStore().List() {
		m, _ := meta.Accessor(obj)
		got[key(m)] = m.GetResourceVersion()
	}
	if d := diff(want, got); d != "" {
		return fmt.Errorf("%s: cache differs from the server (server vs cache): %s", w.name, d)
	}
	w.mu.Lock()
	defer w.mu.Unlock()
	mirror := map[string]string{}
	for k, rv := range w.mirror {
		mirror[k] = strconv.FormatUint(rv, 10)
	}
	if d := diff(got, mirror); d != "" {
		return fmt.Errorf("%s: handler state differs from the cache (cache vs handlers): %s", w.name, d)
	}
	return nil
}

func key(m metav1.Object) string {
	if m.GetNamespace() == "" {
		return m.GetName()
	}
	return m.GetNamespace() + "/" + m.GetName()
}

func diff(a, b map[string]string) string {
	var out []string
	for k, v := range a {
		if b[k] != v {
			out = append(out, fmt.Sprintf("%s %q vs %q", k, v, b[k]))
		}
	}
	for k, v := range b {
		if _, ok := a[k]; !ok {
			out = append(out, fmt.Sprintf("%s %q vs %q", k, "", v))
		}
	}
	sort.Strings(out)
	if len(out) > 5 {
		out = append(out[:5], fmt.Sprintf("and %d more", len(out)-5))
	}
	return strings.Join(out, ", ")
}

func objects(l runtime.Object, err error) ([]metav1.Object, error) {
	if err != nil {
		return nil, err
	}
	items, err := meta.ExtractList(l)
	if err != nil {
		return nil, err
	}
	out := make([]metav1.Object, len(items))
	for i, it := range items {
		if out[i], err = meta.Accessor(it); err != nil {
			return nil, err
		}
	}
	return out, nil
}

// injector is the fault injector on the informers' transport. It counts
// what the reflectors do and, while enabled, fails some requests, delays
// some watches (so they resume past the short watch log and get 410) and
// cuts watch streams after a while.
type injector struct {
	next    http.RoundTripper
	enabled atomic.Bool

	mu    sync.Mutex
	rng   *rand.Rand
	stats stats
}

type stats struct {
	lists, watches, watchLists, resumed int64
	gone, bookmarks, initialEnd         int64
	cut, delayed, failed                int64
}

func (f *injector) snapshot() stats {
	f.mu.Lock()
	defer f.mu.Unlock()
	return f.stats
}

// roll draws from the seeded source. Faults depend on the seed, but the
// interleaving with the server does not, so runs are not exactly replayable.
func (f *injector) roll(fn func(r *rand.Rand, s *stats)) {
	f.mu.Lock()
	defer f.mu.Unlock()
	fn(f.rng, &f.stats)
}

func (f *injector) RoundTrip(req *http.Request) (*http.Response, error) {
	q := req.URL.Query()
	watch := q.Get("watch") == "true" || q.Get("watch") == "1"
	on := f.enabled.Load()
	var fail bool
	var delay, cutAfter time.Duration
	var clean bool
	f.roll(func(r *rand.Rand, s *stats) {
		if req.Method != http.MethodGet {
			return
		}
		switch {
		case !watch:
			s.lists++
		case q.Get("sendInitialEvents") == "true":
			s.watches++
			s.watchLists++
		default:
			s.watches++
			if rv := q.Get("resourceVersion"); rv != "" && rv != "0" {
				s.resumed++
			}
		}
		if !on {
			return
		}
		fail = r.Intn(10) == 0
		if watch && !fail {
			if r.Intn(3) == 0 {
				delay = time.Duration(300+r.Intn(1200)) * time.Millisecond
				s.delayed++
			}
			cutAfter = time.Duration(200+r.Intn(1800)) * time.Millisecond
			clean = r.Intn(2) == 0
		}
		if fail {
			s.failed++
		}
	})
	if fail {
		return nil, errors.New("informer-go: injected connection failure")
	}
	if delay > 0 {
		select {
		case <-time.After(delay):
		case <-req.Context().Done():
			return nil, req.Context().Err()
		}
	}
	resp, err := f.next.RoundTrip(req)
	if err != nil || !watch {
		return resp, err
	}
	t := &tap{rc: resp.Body, f: f}
	if cutAfter > 0 {
		t.timer = time.AfterFunc(cutAfter, func() {
			if f.enabled.Load() {
				f.roll(func(_ *rand.Rand, s *stats) { s.cut++ })
				t.cut(clean)
			}
		})
	}
	resp.Body = t
	return resp, nil
}

// tap reads a JSON watch stream through, one whole event (line) at a time,
// and counts BOOKMARK and 410 ERROR events. A cut either breaks the
// connection mid-stream (the reflector sees an error and starts over) or
// ends the stream cleanly after a whole event, like a server-side timeout
// (the reflector resumes from its last resourceVersion).
type tap struct {
	rc    io.ReadCloser
	f     *injector
	timer *time.Timer
	in    []byte // read, not yet a whole line
	out   []byte // whole lines, not yet handed out
	ended atomic.Bool
}

func (t *tap) cut(clean bool) {
	if clean {
		t.ended.Store(true)
	}
	_ = t.rc.Close()
}

func (t *tap) Read(p []byte) (int, error) {
	for len(t.out) == 0 {
		chunk := make([]byte, 32<<10)
		n, err := t.rc.Read(chunk)
		t.in = append(t.in, chunk[:n]...)
		if i := bytes.LastIndexByte(t.in, '\n'); i >= 0 {
			t.count(t.in[:i+1])
			t.out = append(t.out, t.in[:i+1]...)
			t.in = append([]byte(nil), t.in[i+1:]...)
		}
		if err != nil && len(t.out) == 0 {
			if t.ended.Load() {
				return 0, io.EOF
			}
			return 0, err
		}
	}
	n := copy(p, t.out)
	t.out = t.out[n:]
	return n, nil
}

func (t *tap) count(lines []byte) {
	for _, line := range bytes.Split(bytes.TrimSuffix(lines, []byte("\n")), []byte("\n")) {
		var ev struct {
			Type   string `json:"type"`
			Object struct {
				Code     int `json:"code"`
				Metadata struct {
					Annotations map[string]string `json:"annotations"`
				} `json:"metadata"`
			} `json:"object"`
		}
		if json.Unmarshal(line, &ev) != nil {
			continue
		}
		t.f.roll(func(_ *rand.Rand, s *stats) {
			switch {
			case ev.Type == "BOOKMARK":
				s.bookmarks++
				if ev.Object.Metadata.Annotations["k8s.io/initial-events-end"] == "true" {
					s.initialEnd++
				}
			case ev.Type == "ERROR" && ev.Object.Code == http.StatusGone:
				s.gone++
			}
		})
	}
}

func (t *tap) Close() error {
	if t.timer != nil {
		t.timer.Stop()
	}
	return t.rc.Close()
}
