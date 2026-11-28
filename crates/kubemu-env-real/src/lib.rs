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

/// Processes and pauses for real-mode tools such as the budget benchmarks
/// (`testing/bench`), which measure `kubemu up` from outside.
pub mod process {
    /// Spawns `program` with null stdin and stdout and piped stderr.
    pub fn spawn_stderr_piped(
        program: &std::path::Path,
        args: &[&str],
    ) -> std::io::Result<std::process::Child> {
        std::process::Command::new(program)
            .args(args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
    }

    /// Resident set size of a live process in bytes, from `/proc` (Linux).
    pub fn rss_bytes(pid: u32) -> std::io::Result<u64> {
        let status = std::fs::read_to_string(format!("/proc/{pid}/status"))?;
        status
            .lines()
            .find_map(|l| l.strip_prefix("VmRSS:"))
            .and_then(|v| v.trim().strip_suffix("kB"))
            .and_then(|kb| kb.trim().parse::<u64>().ok())
            .map(|kb| kb * 1024)
            .ok_or_else(|| std::io::Error::other("no VmRSS in /proc status"))
    }

    pub fn sleep(d: std::time::Duration) {
        std::thread::sleep(d);
    }
}

/// File access for build scripts (`build.rs`), which run at compile time
/// and so sit outside the runtime determinism boundary, but are linted like
/// everything else. Not for runtime code.
pub mod build_script {
    use std::path::{Path, PathBuf};

    /// `OUT_DIR`, as Cargo sets it for a running build script.
    pub fn out_dir() -> PathBuf {
        std::env::var_os("OUT_DIR")
            .map(PathBuf::from)
            .expect("OUT_DIR is set for build scripts")
    }

    /// Reads a file and tells Cargo to rerun the build script when it changes.
    pub fn read_tracked(path: &Path) -> std::io::Result<Vec<u8>> {
        println!("cargo::rerun-if-changed={}", path.display());
        std::fs::read(path)
    }

    /// Writes `contents` unless the file already holds exactly that, so
    /// dependents are not rebuilt for nothing.
    pub fn write_if_changed(path: &Path, contents: &[u8]) -> std::io::Result<()> {
        if std::fs::read(path).is_ok_and(|old| old == contents) {
            return Ok(());
        }
        std::fs::write(path, contents)
    }
}

/// Sockets for shell crates. They wrap the returned std sockets into
/// their async runtime (`tokio::net::TcpListener::from_std`).
pub mod net {
    /// A bound, non-blocking TCP listener, handed to a shell crate.
    #[derive(Debug)]
    pub struct Listener(std::net::TcpListener);

    impl Listener {
        pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
            self.0.local_addr()
        }

        /// For `tokio::net::TcpListener::from_std`.
        pub fn into_std(self) -> std::net::TcpListener {
            self.0
        }
    }

    pub fn bind_tcp(addr: &str) -> std::io::Result<Listener> {
        let l = std::net::TcpListener::bind(addr)?;
        l.set_nonblocking(true)?;
        Ok(Listener(l))
    }

    /// Connects a non-blocking TCP stream (shell tests, clients).
    pub fn connect_tcp(addr: std::net::SocketAddr) -> std::io::Result<std::net::TcpStream> {
        let s = std::net::TcpStream::connect(addr)?;
        s.set_nonblocking(true)?;
        Ok(s)
    }

    /// A blocking `GET` over a fresh connection; returns the status code
    /// (readiness probes in real-mode tools).
    pub fn http_get_status(addr: std::net::SocketAddr, path: &str) -> std::io::Result<u16> {
        use std::io::{Read, Write};
        let mut s = std::net::TcpStream::connect(addr)?;
        write!(
            s,
            "GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
        )?;
        let mut head = [0u8; 12];
        s.read_exact(&mut head)?;
        std::str::from_utf8(&head[9..12])
            .ok()
            .and_then(|c| c.parse().ok())
            .ok_or_else(|| std::io::Error::other("malformed HTTP status line"))
    }
}

/// Files the CLI writes for the user (kubeconfigs).
pub mod fs {
    pub fn write(path: &std::path::Path, contents: &[u8]) -> std::io::Result<()> {
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(path, contents)
    }
}
