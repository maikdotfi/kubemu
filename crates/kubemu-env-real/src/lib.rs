//! Real [`Env`]: the one crate allowed across the determinism boundary.
#![allow(clippy::disallowed_methods, clippy::disallowed_types)]

use std::hash::{BuildHasher, Hasher};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use kubemu_env::{Clock, Env, MonoTime, Rng, WallTime, Xoshiro256StarStar};

#[derive(Debug)]
pub struct RealClock {
    origin: Instant,
}

impl Default for RealClock {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Clock for RealClock {
    fn now(&self) -> MonoTime {
        MonoTime(u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX))
    }
    fn wall(&self) -> WallTime {
        let d = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        WallTime(u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
    }
}

/// Non-cryptographic RNG seeded from process entropy. The seed is exposed so
/// a real-mode run can still be logged and replayed.
#[derive(Debug)]
pub struct RealRng {
    seed: u64,
    inner: Mutex<Xoshiro256StarStar>,
}

impl RealRng {
    pub fn from_entropy() -> Self {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
        );
        Self::from_seed(h.finish())
    }

    pub fn from_seed(seed: u64) -> Self {
        Self {
            seed,
            inner: Mutex::new(Xoshiro256StarStar::seed_from_u64(seed)),
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl Rng for RealRng {
    fn next_u64(&self) -> u64 {
        self.inner.lock().expect("rng poisoned").next_u64()
    }
}

/// Build a real Env. With `seed = None` the RNG is seeded from entropy.
pub fn real_env(seed: Option<u64>) -> (Env, u64) {
    let rng = seed.map_or_else(RealRng::from_entropy, RealRng::from_seed);
    let seed = rng.seed();
    (
        Env::new(Arc::new(RealClock::default()), Arc::new(rng)),
        seed,
    )
}

/// Spawns `program` with piped stdin and stdout and inherited stderr. All
/// process creation goes through this crate (ADR 0001); callers only use the
/// handles they are given.
pub fn spawn_piped(
    program: &std::path::Path,
    args: &[&str],
) -> std::io::Result<std::process::Child> {
    std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
}
