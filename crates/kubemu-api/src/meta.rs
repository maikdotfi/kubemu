//! Object metadata: UIDs and timestamps drawn from [`Env`], and generated
//! names. The metadata rules live in `kubemu-validation`.

use kubemu_env::Env;

/// A random (v4) UUID from the Env's RNG.
pub fn uuid_v4(env: &Env) -> String {
    let (a, b) = (env.rng.next_u64(), env.rng.next_u64());
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&a.to_be_bytes());
    bytes[8..].copy_from_slice(&b.to_be_bytes());
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let h: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}

/// The Env's wall clock as RFC 3339 with second precision
/// (`metav1.Time`).
pub fn now_rfc3339(env: &Env) -> String {
    rfc3339(env.clock.wall().0 / 1_000_000_000)
}

/// Unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn rfc3339(unix: u64) -> String {
    let days = (unix / 86_400) as i64;
    let secs = unix % 86_400;
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        secs / 3600,
        secs / 60 % 60,
        secs % 60
    )
}

/// `generateName` suffix: five characters from the apiserver's alphabet
/// (no vowels, no confusable digits).
pub fn name_suffix(env: &Env) -> String {
    const ALPHABET: &[u8] = b"bcdfghjklmnpqrstvwxz2456789";
    (0..5)
        .map(|_| ALPHABET[env.rng.below(ALPHABET.len() as u64) as usize] as char)
        .collect()
}

/// The apiserver truncates `generateName` so the full name fits.
pub fn generate_name(prefix: &str, env: &Env) -> String {
    const MAX_PREFIX: usize = 63 - 5;
    let mut p = prefix;
    if p.len() > MAX_PREFIX {
        p = &p[..MAX_PREFIX];
    }
    format!("{p}{}", name_suffix(env))
}

pub use kubemu_validation::field::required;

#[cfg(test)]
mod tests {
    use super::*;
    use kubemu_env_sim::SimEnv;

    #[test]
    fn uuid_shape() {
        let env = SimEnv::new(1).env;
        let u = uuid_v4(&env);
        assert_eq!(u.len(), 36);
        assert_eq!(&u[14..15], "4");
        assert!(matches!(&u[19..20], "8" | "9" | "a" | "b"));
        assert_ne!(u, uuid_v4(&env));
    }

    #[test]
    fn timestamps() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339(1_735_689_600), "2025-01-01T00:00:00Z");
        assert_eq!(rfc3339(951_782_400 + 3661), "2000-02-29T01:01:01Z");
    }

    #[test]
    fn generated_names() {
        let env = SimEnv::new(1).env;
        let n = generate_name("web-", &env);
        assert!(n.starts_with("web-") && n.len() == 9);
        assert_eq!(generate_name(&"x".repeat(80), &env).len(), 63);
    }
}
