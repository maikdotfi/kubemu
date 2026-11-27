//! DST harness (README §10). Drives the control plane with random clients
//! under a seeded [`SimEnv`], checks invariants continuously, and records a
//! trace whose hash must be identical for identical seeds.
//!
//! Today it exercises the store and the store fault points: random writers
//! on several clusters, clusters forked mid-run, and lagging watchers that
//! mirror state from the event stream under injected watch drops and 410s
//! (relisting on Gone). Controllers and the API server plug in as they land.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::Duration;

use kubemu_env_sim::SimEnv;
use kubemu_faults::{Fault, FaultConfig, FaultSchedule};
use kubemu_store::{ClusterStore, EventKind, ObjectKey, ResourceVersion, Store, StoreError};

pub const CLUSTERS: [&str; 3] = ["alpha", "beta", "gamma"];

#[derive(Clone, Debug)]
pub struct Config {
    pub steps: u32,
    pub log_capacity: usize,
    pub keys_per_cluster: u64,
    /// Forks stop once this many clusters exist.
    pub max_clusters: usize,
    pub faults: FaultConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            steps: 500,
            log_capacity: 32,
            keys_per_cluster: 16,
            max_clusters: 6,
            faults: FaultConfig::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub seed: u64,
    pub trace_hash: u64,
    pub trace_lines: usize,
    pub faults: usize,
    pub forks: u32,
    pub violations: Vec<String>,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        self.violations.is_empty()
    }
}

/// A client-side mirror fed by watch events, like a client-go informer.
#[derive(Clone, Debug, Default)]
struct Watcher {
    cursor: ResourceVersion,
    mirror: BTreeMap<ObjectKey, (ResourceVersion, u64)>,
    relists: u32,
}

impl Watcher {
    fn relist(&mut self, s: &ClusterStore<u64>) {
        self.mirror = s
            .list("configmaps", None)
            .0
            .into_iter()
            .map(|(k, o)| (k.clone(), (o.resource_version, o.value)))
            .collect();
        self.cursor = s.resource_version();
        self.relists += 1;
    }

    /// Reads pending events. With `faults`, the read may be cut short or
    /// answered with 410 Gone.
    fn sync(
        &mut self,
        s: &ClusterStore<u64>,
        faults: Option<&mut FaultSchedule>,
        cluster: &str,
        violations: &mut Vec<String>,
    ) {
        let events = match s.events_since(self.cursor) {
            Ok(events) => events,
            Err(StoreError::Gone { .. }) => return self.relist(s),
            Err(other) => {
                return violations.push(format!("{cluster}: unexpected watch error {other}"));
            }
        };
        let deliver = match faults.and_then(|f| f.on_watch(cluster, events.len())) {
            Some(Fault::WatchGone) => return self.relist(s),
            Some(Fault::WatchDrop { deliver }) => deliver,
            None => events.len(),
        };
        for e in events.into_iter().take(deliver) {
            // Every write is one event, so a gap means a lost event.
            if e.resource_version != self.cursor + 1 {
                violations.push(format!(
                    "{cluster}: watch skipped from rv {} to {}",
                    self.cursor, e.resource_version
                ));
            }
            self.cursor = e.resource_version;
            match e.kind {
                EventKind::Added | EventKind::Modified => {
                    self.mirror.insert(e.key, (e.resource_version, e.value));
                }
                EventKind::Deleted => {
                    self.mirror.remove(&e.key);
                }
            }
        }
    }
}

pub fn run(seed: u64, cfg: &Config) -> Outcome {
    let sim = SimEnv::new(seed);
    let rng = sim.env.rng.clone();
    let mut faults = FaultSchedule::new(sim.env.clone(), cfg.faults.clone());
    let mut store: Store<u64> = Store::new();
    let mut watchers: BTreeMap<String, Watcher> = BTreeMap::new();
    for c in CLUSTERS {
        *store.add_cluster(c) = ClusterStore::new(cfg.log_capacity);
        watchers.insert(c.to_owned(), Watcher::default());
    }
    let mut forks = 0u32;

    let mut trace = String::new();
    let mut violations = Vec::new();

    for step in 0..cfg.steps {
        sim.clock.advance(Duration::from_millis(rng.below(50)));
        // `watchers` has the same keys as the store, in the same order.
        let names: Vec<String> = watchers.keys().cloned().collect();
        let cluster = names[rng.below(names.len() as u64) as usize].as_str();
        let key = ObjectKey::new(
            "configmaps",
            "default",
            format!("cm-{}", rng.below(cfg.keys_per_cluster)),
        );
        let value = rng.next_u64();
        let faults_before = faults.injected().len();

        let result = match rng.below(20) {
            0..=7 => store
                .cluster_mut(cluster)
                .and_then(|s| s.create(key.clone(), value))
                .map(|rv| format!("create {key} rv={rv}")),
            8..=13 => store.cluster_mut(cluster).and_then(|s| {
                // Sometimes send a stale resourceVersion to exercise conflicts.
                let expected = s
                    .get(&key)
                    .map(|o| o.resource_version - u64::from(rng.chance(0.2)));
                s.update(&key, value, expected)
                    .map(|rv| format!("update {key} rv={rv}"))
            }),
            14..=16 => store
                .cluster_mut(cluster)
                .and_then(|s| s.delete(&key, None))
                .map(|o| format!("delete {key} was={}", o.resource_version)),
            17 if names.len() < cfg.max_clusters => {
                forks += 1;
                let to = format!("{cluster}.{forks}");
                store.fork(cluster, to.clone()).map(|s| {
                    // The fork's informer resumes from the parent's cursor.
                    let w = watchers[cluster].clone();
                    let out = format!(
                        "fork -> {to} rv={} cursor={}",
                        s.resource_version(),
                        w.cursor
                    );
                    watchers.insert(to, w);
                    out
                })
            }
            _ => {
                let s = store.cluster(cluster).expect("cluster exists");
                let w = watchers.get_mut(cluster).expect("watcher exists");
                w.sync(s, Some(&mut faults), cluster, &mut violations);
                Ok(format!("sync cursor={} relists={}", w.cursor, w.relists))
            }
        };
        let now = sim.env.clock.now().0;
        for f in &faults.injected()[faults_before..] {
            let _ = writeln!(trace, "{step} {f}");
        }
        let outcome = result.unwrap_or_else(|e| format!("err {e}"));
        let _ = writeln!(trace, "{step} t={now} {cluster} {outcome}");
    }

    // Invariant: after catching up without faults, every watcher mirrors its
    // store exactly.
    for (c, w) in &mut watchers {
        let s = store.cluster(c).expect("cluster exists");
        w.sync(s, None, c, &mut violations);
        let truth: BTreeMap<_, _> = s
            .list("configmaps", None)
            .0
            .into_iter()
            .map(|(k, o)| (k.clone(), (o.resource_version, o.value)))
            .collect();
        if w.mirror != truth {
            violations.push(format!("{c}: watcher mirror diverged from store"));
        }
        let _ = writeln!(
            trace,
            "final {c} rv={} objects={} relists={}",
            s.resource_version(),
            s.len(),
            w.relists
        );
    }
    if !store
        .cluster_names()
        .eq(watchers.keys().map(String::as_str))
    {
        violations.push("store clusters and watchers diverged".into());
    }

    Outcome {
        seed,
        trace_hash: fnv1a(trace.as_bytes()),
        trace_lines: trace.lines().count(),
        faults: faults.injected().len(),
        forks,
        violations,
    }
}

/// FNV-1a 64: stable, dependency-free trace fingerprint.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_trace() {
        let cfg = Config::default();
        for seed in 0..20 {
            assert_eq!(
                run(seed, &cfg).trace_hash,
                run(seed, &cfg).trace_hash,
                "seed {seed}"
            );
        }
    }

    #[test]
    fn forks_and_faults_happen() {
        let cfg = Config::default();
        let o = run(0, &cfg);
        assert!(o.faults > 0, "default config should inject faults");
        assert!(o.forks > 0, "default config should fork clusters");
        let quiet = Config {
            faults: FaultConfig::NONE,
            ..Config::default()
        };
        assert_eq!(run(0, &quiet).faults, 0);
    }

    #[test]
    fn different_seeds_diverge() {
        let cfg = Config::default();
        assert_ne!(run(1, &cfg).trace_hash, run(2, &cfg).trace_hash);
    }

    #[test]
    fn invariants_hold_across_seeds() {
        let cfg = Config::default();
        for seed in 0..200 {
            let o = run(seed, &cfg);
            assert!(o.ok(), "seed {seed}: {:?}", o.violations);
        }
    }
}
