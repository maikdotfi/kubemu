//! The determinism boundary (README §1.2, §3.1).
//!
//! Every source of nondeterminism is reached through [`Env`]. Code outside
//! `kubemu-env-real` is linted (see `clippy.toml`) so it cannot touch the
//! real clock, RNG, filesystem, network or process table directly.
//!
//! Core crates are synchronous and runtime-free; tokio lives only in shell
//! crates (ADR 0001). `Env` is [`Clock`] and [`Rng`] for now; narrow effect
//! traits (files, a Turso `IO` adapter) are added when a core crate needs one.

use std::fmt;
use std::sync::Arc;

mod prng;
pub use prng::Xoshiro256StarStar;

/// Monotonic time in nanoseconds since an arbitrary, per-Env origin.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonoTime(pub u64);

/// Wall-clock time in nanoseconds since the Unix epoch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallTime(pub u64);

impl MonoTime {
    pub fn saturating_since(self, earlier: MonoTime) -> std::time::Duration {
        std::time::Duration::from_nanos(self.0.saturating_sub(earlier.0))
    }
}

/// Monotonic and wall clock.
pub trait Clock: Send + Sync + fmt::Debug {
    fn now(&self) -> MonoTime;
    fn wall(&self) -> WallTime;
}

/// Source of randomness. Sim implementations are seeded and replayable.
pub trait Rng: Send + Sync + fmt::Debug {
    fn next_u64(&self) -> u64;

    /// Uniform value in `0..bound` (`bound > 0`), using Lemire's method.
    fn below(&self, bound: u64) -> u64 {
        assert!(bound > 0, "bound must be positive");
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let m = u128::from(self.next_u64()) * u128::from(bound);
            if (m as u64) >= threshold {
                return (m >> 64) as u64;
            }
        }
    }

    /// `true` with probability `p` (clamped to `0.0..=1.0`).
    fn chance(&self, p: f64) -> bool {
        let p = p.clamp(0.0, 1.0);
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64) < p
    }
}

/// The handle every component receives. Cheap to clone.
#[derive(Clone, Debug)]
pub struct Env {
    pub clock: Arc<dyn Clock>,
    pub rng: Arc<dyn Rng>,
}

impl Env {
    pub fn new(clock: Arc<dyn Clock>, rng: Arc<dyn Rng>) -> Self {
        Self { clock, rng }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct Fixed(Mutex<Xoshiro256StarStar>);
    impl Rng for Fixed {
        fn next_u64(&self) -> u64 {
            self.0.lock().unwrap().next_u64()
        }
    }

    #[test]
    fn below_stays_in_range() {
        let r = Fixed(Mutex::new(Xoshiro256StarStar::seed_from_u64(7)));
        for bound in [1, 2, 3, 10, 1000, u64::MAX] {
            for _ in 0..1000 {
                assert!(r.below(bound) < bound);
            }
        }
    }

    #[test]
    fn chance_extremes() {
        let r = Fixed(Mutex::new(Xoshiro256StarStar::seed_from_u64(7)));
        assert!((0..1000).all(|_| !r.chance(0.0)));
        assert!((0..1000).all(|_| r.chance(1.0)));
    }
}
