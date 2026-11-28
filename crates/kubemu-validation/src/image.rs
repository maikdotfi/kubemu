//! Image references, as `github.com/distribution/reference`'s
//! `ParseNormalizedNamed` reads them; only far enough to find the tag,
//! which decides `imagePullPolicy`'s default.

/// The tag `parsers.ParseImageName` reports: the reference's tag, `latest`
/// when it has neither tag nor digest, empty for a digest alone. `None`
/// when the reference does not parse.
pub fn tag(image: &str) -> Option<String> {
    if image.len() == 64
        && image
            .bytes()
            .all(|c| matches!(c, b'a'..=b'f' | b'0'..=b'9'))
    {
        return None;
    }
    let (domain, remainder) = split_docker_domain(image);
    let remote = remainder.split(':').next().unwrap_or("");
    if remote.to_lowercase() != remote {
        return None;
    }
    let full = format!("{domain}/{remainder}");
    let (tag, digest) = parse(&full)?;
    Some(match (tag, digest) {
        ("", false) => "latest".into(),
        (t, _) => t.into(),
    })
}

fn split_docker_domain(name: &str) -> (String, String) {
    const DEFAULT: &str = "docker.io";
    let Some((maybe_domain, maybe_remote)) = name.split_once('/') else {
        return (DEFAULT.into(), format!("library/{name}"));
    };
    let (domain, mut remote) = if maybe_domain == "localhost"
        || maybe_domain.contains(['.', ':'])
        || maybe_domain.to_lowercase() != maybe_domain
    {
        if maybe_domain == "index.docker.io" {
            (DEFAULT.to_owned(), maybe_remote.to_owned())
        } else {
            (maybe_domain.to_owned(), maybe_remote.to_owned())
        }
    } else {
        (DEFAULT.to_owned(), name.to_owned())
    };
    if domain == DEFAULT && !remote.contains('/') {
        remote = format!("library/{remote}");
    }
    (domain, remote)
}

/// `reference.Parse`: the tag and whether a digest is present.
fn parse(s: &str) -> Option<(&str, bool)> {
    let (rest, digest) = match s.split_once('@') {
        Some((r, d)) => (r, Some(d)),
        None => (s, None),
    };
    if let Some(d) = digest
        && !valid_digest(d)
    {
        return None;
    }
    let last_slash = rest.rfind('/').map_or(0, |i| i + 1);
    let (name, tag) = match rest[last_slash..].find(':') {
        Some(i) => (&rest[..last_slash + i], &rest[last_slash + i + 1..]),
        None => (rest, ""),
    };
    if !tag.is_empty() || rest.len() > name.len() {
        let b = tag.as_bytes();
        let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
        if b.is_empty()
            || b.len() > 128
            || !word(b[0])
            || !b.iter().all(|&c| word(c) || c == b'.' || c == b'-')
        {
            return None;
        }
    }
    let segs: Vec<&str> = name.split('/').collect();
    let all_path = segs.iter().all(|s| path_component(s));
    let with_domain =
        segs.len() > 1 && domain_and_port(segs[0]) && segs[1..].iter().all(|s| path_component(s));
    if !all_path && !with_domain {
        return None;
    }
    // The name regexp takes a domain when it can; the rest is the path.
    let path_len = if with_domain {
        name.len() - segs[0].len() - 1
    } else {
        name.len()
    };
    if path_len > 255 {
        return None;
    }
    Some((tag, digest.is_some()))
}

/// `[a-z0-9]+(?:(?:[._]|__|[-]+)[a-z0-9]+)*`.
fn path_component(s: &str) -> bool {
    let b = s.as_bytes();
    let an = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit();
    let mut i = 0;
    let run = |i: &mut usize| {
        let start = *i;
        while *i < b.len() && an(b[*i]) {
            *i += 1;
        }
        *i > start
    };
    if !run(&mut i) {
        return false;
    }
    while i < b.len() {
        match b[i] {
            b'.' => i += 1,
            b'_' => {
                i += 1;
                if i < b.len() && b[i] == b'_' {
                    i += 1;
                }
            }
            b'-' => {
                while i < b.len() && b[i] == b'-' {
                    i += 1;
                }
            }
            _ => return false,
        }
        if !run(&mut i) {
            return false;
        }
    }
    true
}

/// `(?:domainName|\[[a-fA-F0-9:]+\])(?::[0-9]+)?`.
fn domain_and_port(s: &str) -> bool {
    let (host, port) = if s.starts_with('[') {
        match s.find(']') {
            Some(i) => (&s[..=i], &s[i + 1..]),
            None => return false,
        }
    } else {
        match s.find(':') {
            Some(i) => (&s[..i], &s[i..]),
            None => (s, ""),
        }
    };
    let port_ok = port.is_empty()
        || (port.len() > 1
            && port.starts_with(':')
            && port[1..].bytes().all(|c| c.is_ascii_digit()));
    let host_ok = if let Some(inner) = host.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
        !inner.is_empty() && inner.bytes().all(|c| c.is_ascii_hexdigit() || c == b':')
    } else {
        host.split('.').all(|c| {
            let b = c.as_bytes();
            !b.is_empty()
                && b[0].is_ascii_alphanumeric()
                && b[b.len() - 1].is_ascii_alphanumeric()
                && b.iter().all(|&x| x.is_ascii_alphanumeric() || x == b'-')
        })
    };
    port_ok && host_ok
}

/// The reference grammar's digest, then `go-digest`'s `Parse`: a known
/// algorithm and lowercase hex of its length.
fn valid_digest(d: &str) -> bool {
    let Some((alg, enc)) = d.split_once(':') else {
        return false;
    };
    let alg_ok = alg.split(['-', '_', '+', '.']).all(|p| {
        let b = p.as_bytes();
        !b.is_empty() && b[0].is_ascii_alphabetic() && b.iter().all(u8::is_ascii_alphanumeric)
    });
    if !alg_ok || enc.len() < 32 || !enc.bytes().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    let want = match alg {
        "sha256" => 64,
        "sha384" => 96,
        "sha512" => 128,
        _ => return false,
    };
    enc.len() == want && enc.bytes().all(|c| matches!(c, b'a'..=b'f' | b'0'..=b'9'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags() {
        let sha = "a".repeat(64);
        for (i, want) in [
            ("nginx", Some("latest")),
            ("nginx:1.25", Some("1.25")),
            ("nginx:latest", Some("latest")),
            ("localhost:5000/app", Some("latest")),
            ("localhost:5000/app:v1", Some("v1")),
            ("registry.k8s.io/pause:3.10", Some("3.10")),
            ("Nginx", None),
            ("", None),
            ("nginx:", None),
            ("a__b", Some("latest")),
            ("a___b", None),
        ] {
            assert_eq!(tag(i).as_deref(), want, "{i}");
        }
        assert_eq!(tag(&format!("nginx@sha256:{sha}")).as_deref(), Some(""));
        assert_eq!(tag(&format!("nginx:1@sha256:{sha}")).as_deref(), Some("1"));
        assert_eq!(tag(&sha), None);
        assert_eq!(tag("nginx@sha256:abc"), None);
    }
}
