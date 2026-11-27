//! Deterministic [`Env`] for simulation: virtual time that only moves when
//! the harness advances it, and a seeded RNG.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use kubemu_env::{Clock, Env, MonoTime, Rng, WallTime, Xoshiro256StarStar};

/// Fixed wall-clock origin for every simulation: 2025-01-01T00:00:00Z.
pub const SIM_EPOCH_UNIX_NANOS: u64 = 1_735_689_600 * 1_000_000_000;

#[derive(Debug, Default)]
pub struct SimClock {
    nanos: AtomicU64,
}

impl SimClock {
    pub fn advance(&self, by: Duration) {
        let by = u64::try_from(by.as_nanos()).expect("advance overflow");
        self.nanos.fetch_add(by, Ordering::SeqCst);
    }
}

impl Clock for SimClock {
    fn now(&self) -> MonoTime {
        MonoTime(self.nanos.load(Ordering::SeqCst))
    }
    fn wall(&self) -> WallTime {
        WallTime(SIM_EPOCH_UNIX_NANOS + self.now().0)
    }
}

#[derive(Debug)]
pub struct SimRng(Mutex<Xoshiro256StarStar>);

impl SimRng {
    pub fn new(seed: u64) -> Self {
        Self(Mutex::new(Xoshiro256StarStar::seed_from_u64(seed)))
    }
}

impl Rng for SimRng {
    fn next_u64(&self) -> u64 {
        self.0.lock().expect("rng poisoned").next_u64()
    }
}

/// A simulated environment plus the handles only the harness may touch.
#[derive(Clone, Debug)]
pub struct SimEnv {
    pub seed: u64,
    pub env: Env,
    pub clock: Arc<SimClock>,
}

impl SimEnv {
    pub fn new(seed: u64) -> Self {
        let clock = Arc::new(SimClock::default());
        let env = Env::new(clock.clone(), Arc::new(SimRng::new(seed)));
        Self { seed, env, clock }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_stream() {
        let a = SimEnv::new(42);
        let b = SimEnv::new(42);
        for _ in 0..100 {
            assert_eq!(a.env.rng.next_u64(), b.env.rng.next_u64());
        }
    }

    #[test]
    fn time_only_moves_when_advanced() {
        let s = SimEnv::new(1);
        let t0 = s.env.clock.now();
        assert_eq!(s.env.clock.now(), t0);
        s.clock.advance(Duration::from_millis(5));
        assert_eq!(
            s.env.clock.now().saturating_since(t0),
            Duration::from_millis(5)
        );
    }
}
