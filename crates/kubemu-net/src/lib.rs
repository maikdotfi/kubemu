//! Cluster networking (README §6, ADR 0003).
//!
//! So far only the deterministic core, IPAM: each cluster's pod and Service
//! CIDRs ([`ClusterNetwork`]) and picking free addresses in them
//! ([`allocate`]). The API allocates Service cluster IPs with it, the
//! kubelet pod IPs. What is allocated is not recorded here: callers derive
//! the addresses in use from the objects in the store (Services'
//! `clusterIPs`, live pods' `podIPs`), so snapshots and forks carry them.
//!
//! This crate is checked as a core crate (`scripts/check-core-deps.sh`):
//! the privileged parts (bridge, veths, proxy, DNS; README §12.3 step 6)
//! must live in a shell crate, not here.
//!
//! IPv4 only, single-stack: a cluster has one pod and one Service CIDR.

use std::collections::BTreeSet;
use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

use kubemu_env::Rng;

/// An IPv4 network: address and prefix length, host bits zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cidr {
    net: u32,
    prefix: u8,
}

impl Cidr {
    fn mask(&self) -> u32 {
        u32::MAX << (32 - self.prefix)
    }

    pub fn network(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.net)
    }

    pub fn broadcast(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.net | !self.mask())
    }

    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        u32::from(ip) & self.mask() == self.net
    }

    /// The first host address: the pod bridge's gateway (ADR 0003), the
    /// `kubernetes` Service's cluster IP.
    pub fn first_host(&self) -> Ipv4Addr {
        Ipv4Addr::from(self.net + 1)
    }

    /// The last host address (the one before broadcast).
    pub fn last_host(&self) -> Ipv4Addr {
        Ipv4Addr::from((self.net | !self.mask()) - 1)
    }

    pub fn overlaps(&self, other: &Cidr) -> bool {
        self.contains(other.network()) || other.contains(self.network())
    }
}

impl fmt::Display for Cidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.network(), self.prefix)
    }
}

impl FromStr for Cidr {
    type Err = String;

    /// `a.b.c.d/n` with `n` in 8..=30 and no host bits set.
    fn from_str(s: &str) -> Result<Self, String> {
        let (ip, prefix) = s
            .split_once('/')
            .ok_or_else(|| format!("{s:?} is not a CIDR (want e.g. 10.96.0.0/16)"))?;
        if ip.contains(':') {
            return Err(format!("{s:?}: IPv6 is not supported yet"));
        }
        let ip: Ipv4Addr = ip
            .parse()
            .map_err(|_| format!("{s:?} is not a CIDR (want e.g. 10.96.0.0/16)"))?;
        let prefix: u8 = prefix
            .parse()
            .ok()
            .filter(|p| (8..=30).contains(p))
            .ok_or_else(|| format!("{s:?}: the prefix length must be between 8 and 30"))?;
        let c = Cidr {
            net: u32::from(ip),
            prefix,
        };
        if c.net & !c.mask() != 0 {
            let fixed = Cidr {
                net: c.net & c.mask(),
                prefix,
            };
            return Err(format!("{s:?} has host bits set (did you mean {fixed}?)"));
        }
        Ok(c)
    }
}

/// One cluster's address plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClusterNetwork {
    pub pod_cidr: Cidr,
    pub service_cidr: Cidr,
}

impl ClusterNetwork {
    pub fn new(pod_cidr: Cidr, service_cidr: Cidr) -> Result<Self, String> {
        if pod_cidr.overlaps(&service_cidr) {
            return Err(format!(
                "the pod CIDR {pod_cidr} and the Service CIDR {service_cidr} overlap"
            ));
        }
        Ok(Self {
            pod_cidr,
            service_cidr,
        })
    }

    /// The defaults for the `n`th cluster of a process (ADR 0003):
    /// `10.<64+n>.0.0/16` for pods, `10.<128+n>.0.0/16` for Services, so
    /// clusters in one process never overlap. `None` past 64 clusters.
    pub fn default_for(n: usize) -> Option<Self> {
        let n = u8::try_from(n).ok().filter(|&n| n < 64)?;
        let cidr = |second: u8| Cidr {
            net: u32::from(Ipv4Addr::new(10, second, 0, 0)),
            prefix: 16,
        };
        Some(Self {
            pod_cidr: cidr(64 + n),
            service_cidr: cidr(128 + n),
        })
    }
}

/// A free address in `first..=last`, not in `used`: a random start, then
/// the next free address up, wrapping around. `None` when all are used.
/// Looks at no more than `used.len() + 1` candidates.
pub fn allocate(
    first: Ipv4Addr,
    last: Ipv4Addr,
    used: &BTreeSet<Ipv4Addr>,
    rng: &dyn Rng,
) -> Option<Ipv4Addr> {
    let (lo, hi) = (u32::from(first), u32::from(last));
    if hi < lo {
        return None;
    }
    let size = u64::from(hi - lo) + 1;
    if used.range(first..=last).count() as u64 >= size {
        return None;
    }
    let start = rng.below(size);
    (0..size)
        .map(|i| Ipv4Addr::from(lo + ((start + i) % size) as u32))
        .find(|ip| !used.contains(ip))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct Seq(Mutex<u64>);
    impl Rng for Seq {
        fn next_u64(&self) -> u64 {
            let mut s = self.0.lock().unwrap();
            *s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *s
        }
    }

    #[test]
    fn parses_and_rejects() {
        let c: Cidr = "10.96.0.0/12".parse().unwrap();
        assert_eq!(c.to_string(), "10.96.0.0/12");
        assert_eq!(c.first_host(), Ipv4Addr::new(10, 96, 0, 1));
        assert_eq!(c.last_host(), Ipv4Addr::new(10, 111, 255, 254));
        assert!(c.contains(Ipv4Addr::new(10, 111, 255, 255)));
        assert!(!c.contains(Ipv4Addr::new(10, 112, 0, 0)));
        let err = |s: &str| s.parse::<Cidr>().unwrap_err();
        assert!(err("10.96.0.1/12").contains("did you mean 10.96.0.0/12"));
        assert!(err("fd00::/108").contains("IPv6"));
        assert!(err("10.0.0.0/31").contains("between 8 and 30"));
        assert!(err("10.0.0.0").contains("not a CIDR"));
    }

    #[test]
    fn defaults_differ_per_cluster_and_never_overlap() {
        let all: Vec<ClusterNetwork> = (0..64)
            .map(|n| ClusterNetwork::default_for(n).unwrap())
            .collect();
        assert_eq!(all[0].pod_cidr.to_string(), "10.64.0.0/16");
        assert_eq!(all[1].service_cidr.to_string(), "10.129.0.0/16");
        let cidrs: Vec<Cidr> = all
            .iter()
            .flat_map(|n| [n.pod_cidr, n.service_cidr])
            .collect();
        for (i, a) in cidrs.iter().enumerate() {
            for b in &cidrs[i + 1..] {
                assert!(!a.overlaps(b), "{a} overlaps {b}");
            }
        }
        assert!(ClusterNetwork::default_for(64).is_none());
        let c = |s: &str| s.parse::<Cidr>().unwrap();
        assert!(ClusterNetwork::new(c("10.0.0.0/8"), c("10.96.0.0/12")).is_err());
    }

    #[test]
    fn allocates_every_free_address_then_none() {
        let rng = Seq(Mutex::new(7));
        let c: Cidr = "10.0.0.0/29".parse().unwrap();
        let mut used = BTreeSet::from([c.first_host()]);
        // 6 hosts, one taken.
        for _ in 0..5 {
            let ip = allocate(c.first_host(), c.last_host(), &used, &rng).unwrap();
            assert!(c.contains(ip) && ip != c.network() && ip != c.broadcast());
            assert!(used.insert(ip), "{ip} handed out twice");
        }
        assert_eq!(allocate(c.first_host(), c.last_host(), &used, &rng), None);
        // Addresses outside the range do not count as taken.
        let outside = BTreeSet::from([Ipv4Addr::new(192, 168, 0, 1)]);
        assert!(allocate(c.first_host(), c.first_host(), &outside, &rng).is_some());
    }
}
