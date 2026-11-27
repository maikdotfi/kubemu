//! Seeded fault schedule (README §3.3, ADR 0001).
//!
//! Faults are data: every decision is drawn from [`Env::rng`], returned as a
//! typed [`Fault`] for the driver to carry out, and recorded with its logical
//! time and target. The same seed and inputs give the same schedule.
//!
//! M0 covers the store injection points (watch drop, resourceVersion too
//! old). The `FaultSchedule` CRD, serialization, replay files and the other
//! injection points (API, proxies, DNS, runtime, Turso) arrive in M4.

use std::fmt;

use kubemu_env::{Env, MonoTime};

/// One injected fault, as the driver must carry it out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The watch stream breaks after delivering `deliver` of the pending
    /// events. The client keeps its cursor and resumes later.
    WatchDrop { deliver: usize },
    /// The server answers 410 Gone although the history is retained, like a
    /// watch cache that restarted. The client must relist.
    WatchGone,
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WatchDrop { deliver } => write!(f, "watch-drop deliver={deliver}"),
            Self::WatchGone => write!(f, "watch-gone"),
        }
    }
}

/// Per-injection-point probabilities.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultConfig {
    pub watch_drop: f64,
    pub watch_gone: f64,
}

impl FaultConfig {
    pub const NONE: Self = Self {
        watch_drop: 0.0,
        watch_gone: 0.0,
    };
}

impl Default for FaultConfig {
    fn default() -> Self {
        Self {
            watch_drop: 0.1,
            watch_gone: 0.03,
        }
    }
}

/// A fault that fired: what, where and when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Injected {
    pub at: MonoTime,
    pub target: String,
    pub fault: Fault,
}

impl fmt::Display for Injected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fault t={} {} {}", self.at.0, self.target, self.fault)
    }
}

/// Decides faults at injection points and keeps the log of what fired.
#[derive(Debug)]
pub struct FaultSchedule {
    env: Env,
    cfg: FaultConfig,
    injected: Vec<Injected>,
}

impl FaultSchedule {
    pub fn new(env: Env, cfg: FaultConfig) -> Self {
        Self {
            env,
            cfg,
            injected: Vec::new(),
        }
    }

    /// Injection point: a watcher on `target` is about to read `pending`
    /// events. At most one fault fires per read.
    pub fn on_watch(&mut self, target: &str, pending: usize) -> Option<Fault> {
        let rng = &self.env.rng;
        let fault = if rng.chance(self.cfg.watch_gone) {
            Fault::WatchGone
        } else if pending > 0 && rng.chance(self.cfg.watch_drop) {
            Fault::WatchDrop {
                deliver: usize::try_from(rng.below(pending as u64)).expect("fits usize"),
            }
        } else {
            return None;
        };
        self.record(target, fault);
        Some(fault)
    }

    pub fn injected(&self) -> &[Injected] {
        &self.injected
    }

    fn record(&mut self, target: &str, fault: Fault) {
        self.injected.push(Injected {
            at: self.env.clock.now(),
            target: target.to_owned(),
            fault,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kubemu_env_sim::SimEnv;

    fn schedule(seed: u64, cfg: FaultConfig) -> Vec<Injected> {
        let sim = SimEnv::new(seed);
        let mut s = FaultSchedule::new(sim.env.clone(), cfg);
        for i in 0..1000 {
            sim.clock.advance(std::time::Duration::from_millis(1));
            s.on_watch("c", i % 7);
        }
        s.injected().to_vec()
    }

    #[test]
    fn same_seed_same_schedule() {
        let a = schedule(7, FaultConfig::default());
        assert!(!a.is_empty());
        assert_eq!(a, schedule(7, FaultConfig::default()));
        assert_ne!(a, schedule(8, FaultConfig::default()));
    }

    #[test]
    fn none_never_fires_and_drops_stay_in_range() {
        assert!(schedule(1, FaultConfig::NONE).is_empty());
        let always = FaultConfig {
            watch_drop: 1.0,
            watch_gone: 0.0,
        };
        let sim = SimEnv::new(3);
        let mut s = FaultSchedule::new(sim.env, always);
        assert_eq!(s.on_watch("c", 0), None, "nothing pending, nothing to drop");
        for pending in 1..50 {
            match s.on_watch("c", pending) {
                Some(Fault::WatchDrop { deliver }) => assert!(deliver < pending),
                other => panic!("unexpected {other:?}"),
            }
        }
    }
}
