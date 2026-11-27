//! DST harness (README §10). Drives the control plane with random clients
//! under a seeded [`SimEnv`], checks invariants continuously, and records a
//! trace whose hash must be identical for identical seeds.
//!
//! Today it exercises only the store: random writers on several clusters and
//! lagging watchers that mirror state from the event stream (relisting on
//! 410 Gone). Controllers, the API server and faults plug in as they land.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::Duration;

use kubemu_env_sim::SimEnv;
use kubemu_store::{ClusterStore, EventKind, ObjectKey, ResourceVersion, Store, StoreError};

pub const CLUSTERS: [&str; 3] = ["alpha", "beta", "gamma"];

#[derive(Clone, Debug)]
pub struct Config {
    pub steps: u32,
    pub log_capacity: usize,
    pub keys_per_cluster: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            steps: 500,
            log_capacity: 32,
            keys_per_cluster: 16,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub seed: u64,
    pub trace_hash: u64,
    pub trace_lines: usize,
    pub violations: Vec<String>,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        self.violations.is_empty()
    }
}

/// A client-side mirror fed by watch events, like a client-go informer.
#[derive(Debug, Default)]
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

    fn sync(&mut self, s: &ClusterStore<u64>, violations: &mut Vec<String>, cluster: &str) {
        match s.events_since(self.cursor) {
            Ok(events) => {
                for e in events {
                    if e.resource_version <= self.cursor {
                        violations.push(format!(
                            "{cluster}: non-monotonic rv {} after {}",
                            e.resource_version, self.cursor
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
            Err(StoreError::Gone { .. }) => self.relist(s),
            Err(other) => violations.push(format!("{cluster}: unexpected watch error {other}")),
        }
    }
}

pub fn run(seed: u64, cfg: &Config) -> Outcome {
    let sim = SimEnv::new(seed);
    let rng = sim.env.rng.clone();
    let mut store: Store<u64> = Store::new();
    let mut watchers: BTreeMap<&str, Watcher> = BTreeMap::new();
    for c in CLUSTERS {
        *store.add_cluster(c) = ClusterStore::new(cfg.log_capacity);
        watchers.insert(c, Watcher::default());
    }

    let mut trace = String::new();
    let mut violations = Vec::new();

    for step in 0..cfg.steps {
        sim.clock.advance(Duration::from_millis(rng.below(50)));
        let cluster = CLUSTERS[rng.below(CLUSTERS.len() as u64) as usize];
        let s = store.cluster_mut(cluster).expect("cluster exists");
        let key = ObjectKey::new(
            "configmaps",
            "default",
            format!("cm-{}", rng.below(cfg.keys_per_cluster)),
        );
        let value = rng.next_u64();

        let result = match rng.below(10) {
            0..=3 => s
                .create(key.clone(), value)
                .map(|rv| format!("create rv={rv}")),
            4..=6 => {
                // Sometimes send a stale resourceVersion to exercise conflicts.
                let expected = s
                    .get(&key)
                    .map(|o| o.resource_version - u64::from(rng.chance(0.2)));
                s.update(&key, value, expected)
                    .map(|rv| format!("update rv={rv}"))
            }
            7..=8 => s
                .delete(&key, None)
                .map(|o| format!("delete was={}", o.resource_version)),
            _ => {
                let w = watchers.get_mut(cluster).expect("watcher exists");
                w.sync(s, &mut violations, cluster);
                Ok(format!("sync cursor={} relists={}", w.cursor, w.relists))
            }
        };
        let now = sim.env.clock.now().0;
        let outcome = result.unwrap_or_else(|e| format!("err {e}"));
        let _ = writeln!(trace, "{step} t={now} {cluster} {key} {outcome}");
    }

    // Invariant: after catching up, every watcher mirrors its store exactly.
    for c in CLUSTERS {
        let s = store.cluster(c).expect("cluster exists");
        let w = watchers.get_mut(c).expect("watcher exists");
        w.sync(s, &mut violations, c);
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

    Outcome {
        seed,
        trace_hash: fnv1a(trace.as_bytes()),
        trace_lines: trace.lines().count(),
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
