//! `resource.Quantity` (`k8s.io/apimachinery/pkg/api/resource`): parsing,
//! the canonical form it serializes to, and `RoundUp`.
//!
//! Upstream keeps an `int64` fast path next to an `inf.Dec` slow path. The
//! value they hold is the same; what differs is observable only through the
//! original string, which the fast path keeps (and serializes verbatim) when
//! it is already canonical-looking (`+512` stays `+512`). This port holds
//! the value as an exact decimal and remembers what the fast path would
//! have kept. The oracle fuzz (`quantity` op) checks it.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};

/// `resource.ErrFormatWrong`.
pub const ERR_FORMAT_WRONG: &str =
    "quantities must match the regular expression '^([+-]?[0-9.]+)([eEinumkKMGTP]*[-+]?[0-9]*)$'";
/// `resource.ErrNumeric`.
pub const ERR_NUMERIC: &str = "unable to parse numeric part of quantity";
/// `resource.ErrSuffix`.
pub const ERR_SUFFIX: &str = "unable to parse quantity's suffix";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    DecimalExponent,
    BinarySI,
    DecimalSI,
}

/// An exact decimal: `unscaled * 10^-scale`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Dec {
    unscaled: BigInt,
    scale: i64,
}

impl Dec {
    fn int(v: BigInt) -> Self {
        Self {
            unscaled: v,
            scale: 0,
        }
    }

    /// Rounds to `scale` digits after the point, away from zero
    /// (`inf.RoundUp`). Returns whether that was exact.
    fn round_up(&self, scale: i64) -> (Self, bool) {
        if self.scale <= scale {
            return (self.clone(), true);
        }
        let div = pow10(self.scale - scale);
        let (q, r) = self.unscaled.div_rem(&div);
        let exact = r.is_zero();
        let q = if exact {
            q
        } else if self.unscaled.is_negative() {
            q - 1
        } else {
            q + 1
        };
        (Self { unscaled: q, scale }, exact)
    }

    /// Compares with an integer.
    fn cmp_int(&self, y: i64) -> std::cmp::Ordering {
        if self.scale <= 0 {
            (&self.unscaled * pow10(-self.scale)).cmp(&BigInt::from(y))
        } else {
            self.unscaled.cmp(&(BigInt::from(y) * pow10(self.scale)))
        }
    }
}

fn pow10(n: i64) -> BigInt {
    num_traits::pow(BigInt::from(10), usize::try_from(n).unwrap_or(0))
}

/// A parsed quantity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quantity {
    value: Dec,
    format: Format,
    /// The string the fast path keeps and serializes as is.
    cached: Option<String>,
    /// The fast path's `int64Amount` scale, if it took the fast path.
    fast_scale: Option<i64>,
}

struct Parts<'a> {
    positive: bool,
    /// Sign, digits and point, without the suffix.
    value: &'a str,
    num: &'a str,
    denom: &'a str,
    suffix: &'a str,
}

/// `parseQuantityString`.
fn parse_parts(s: &str) -> Result<Parts<'_>, &'static str> {
    let b = s.as_bytes();
    let end = b.len();
    let mut positive = true;
    let mut pos = 0;
    if pos < end {
        match b[0] {
            b'-' => {
                positive = false;
                pos += 1;
            }
            b'+' => pos += 1,
            _ => {}
        }
    }
    let done = |positive, value, num, denom| {
        Ok(Parts {
            positive,
            value,
            num,
            denom,
            suffix: "",
        })
    };
    // Leading zeros.
    loop {
        if pos >= end {
            return done(positive, "0", "0", "");
        }
        if b[pos] == b'0' {
            pos += 1;
        } else {
            break;
        }
    }
    // Numerator.
    let start = pos;
    while pos < end && b[pos].is_ascii_digit() {
        pos += 1;
    }
    if pos >= end {
        return done(positive, s, &s[start..], "");
    }
    let num = if pos == start { "0" } else { &s[start..pos] };
    let mut denom = "";
    if b[pos] == b'.' {
        pos += 1;
        let ds = pos;
        while pos < end && b[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos >= end {
            return done(positive, s, num, &s[ds..]);
        }
        denom = &s[ds..pos];
    }
    let value = &s[..pos];
    let suffix_start = pos;
    while pos < end && b"eEinumkKMGTP".contains(&b[pos]) {
        pos += 1;
    }
    if pos < end && matches!(b[pos], b'-' | b'+') {
        pos += 1;
    }
    while pos < end && b[pos].is_ascii_digit() {
        pos += 1;
    }
    if pos < end {
        return Err(ERR_FORMAT_WRONG);
    }
    Ok(Parts {
        positive,
        value,
        num,
        denom,
        suffix: &s[suffix_start..],
    })
}

/// `quantitySuffixer.interpret`: base, exponent and format.
fn interpret(suffix: &str) -> Option<(u32, i64, Format)> {
    let dec = |e| Some((10, e, Format::DecimalSI));
    let bin = |e| Some((2, e, Format::BinarySI));
    match suffix {
        "" => dec(0),
        "n" => dec(-9),
        "u" => dec(-6),
        "m" => dec(-3),
        "k" => dec(3),
        "M" => dec(6),
        "G" => dec(9),
        "T" => dec(12),
        "P" => dec(15),
        "E" => dec(18),
        "Ki" => bin(10),
        "Mi" => bin(20),
        "Gi" => bin(30),
        "Ti" => bin(40),
        "Pi" => bin(50),
        "Ei" => bin(60),
        _ if suffix.len() > 1 && suffix.starts_with(['e', 'E']) => {
            // strconv.ParseInt(.., 10, 64), then an int32 conversion.
            let e: i64 = suffix[1..].parse().ok()?;
            Some((10, i64::from(e as i32), Format::DecimalExponent))
        }
        _ => None,
    }
}

impl Quantity {
    /// `resource.ParseQuantity`.
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        if s.is_empty() {
            return Err(ERR_FORMAT_WRONG);
        }
        if s == "0" {
            return Ok(Self {
                value: Dec::int(BigInt::zero()),
                format: Format::DecimalSI,
                cached: Some(s.to_owned()),
                fast_scale: Some(0),
            });
        }
        let p = parse_parts(s)?;
        let (base, exponent, format) = interpret(p.suffix).ok_or(ERR_SUFFIX)?;
        let num_len = p.num.len() as i64;
        let denom_len = p.denom.len() as i64;

        // The int64 fast path.
        let (mut scale, mantissa, precision) = match format {
            Format::DecimalExponent | Format::DecimalSI => {
                (exponent, 1i64, 18 - num_len - denom_len)
            }
            Format::BinarySI if denom_len == 0 => {
                (0, 1i64 << exponent, 15 - num_len - exponent * 3 / 10 - 1)
            }
            Format::BinarySI => (0, 1, -1),
        };
        if precision >= 0 {
            scale -= denom_len;
            if scale >= -9 {
                let shifted = format!("{}{}", p.num, p.denom);
                let value: i64 = shifted.parse().map_err(|_| ERR_NUMERIC)?;
                if let Some(result) = value.checked_mul(mantissa) {
                    let result = if p.positive { result } else { -result };
                    let canonical_looking = match format {
                        Format::BinarySI => exponent % 10 == 0 && value & 0x07 != 0,
                        _ => {
                            scale % 3 == 0 && !shifted.ends_with("000") && !shifted.starts_with('0')
                        }
                    };
                    return Ok(Self {
                        value: Dec {
                            unscaled: BigInt::from(result),
                            scale: -scale,
                        },
                        format,
                        cached: canonical_looking.then(|| s.to_owned()),
                        fast_scale: Some(scale),
                    });
                }
            }
        }

        // The inf.Dec slow path.
        if !p.value.bytes().any(|c| c.is_ascii_digit()) {
            return Err(ERR_NUMERIC);
        }
        let digits = format!("{}{}", p.num, p.denom);
        let mut unscaled: BigInt = digits.parse().map_err(|_| ERR_NUMERIC)?;
        let mut dscale = denom_len;
        if base == 10 {
            dscale -= exponent;
        } else {
            unscaled <<= exponent as usize;
        }
        let mut amount = Dec {
            unscaled,
            scale: dscale,
        };
        if !amount.unscaled.is_zero() {
            amount = amount.round_up(9).0;
        }
        let mut format = format;
        if format == Format::BinarySI && amount.cmp_int(i64::MAX).is_gt() {
            amount = Dec::int(BigInt::from(i64::MAX));
        }
        if format == Format::BinarySI && amount.cmp_int(1).is_lt() && amount.cmp_int(0).is_gt() {
            format = Format::DecimalSI;
        }
        if !p.positive {
            amount.unscaled = -amount.unscaled;
        }
        Ok(Self {
            value: amount,
            format,
            cached: None,
            fast_scale: None,
        })
    }

    /// `UnmarshalJSON` for a JSON string (quotes removed).
    pub fn from_json_str(s: &str) -> Result<Self, &'static str> {
        Self::parse(s.trim())
    }

    /// `RoundUp(scale)`: `scale` is the decimal exponent (-3 for milli).
    pub fn round_up(&mut self, scale: i64) {
        if let Some(fs) = self.fast_scale
            && fs >= scale
        {
            return;
        }
        self.cached = None;
        self.value = self.value.round_up(-scale).0;
    }

    fn is_zero(&self) -> bool {
        self.value.unscaled.is_zero()
    }

    /// `CanonicalizeBytes` and `MarshalJSON`: the string it serializes to.
    pub fn canonical(&self) -> String {
        if let Some(s) = &self.cached {
            return s.clone();
        }
        if self.is_zero() {
            return "0".into();
        }
        let mut format = self.format;
        let mut rounded = None;
        if format == Format::BinarySI {
            if self.value.cmp_int(-1024).is_gt() && self.value.cmp_int(1024).is_lt() {
                format = Format::DecimalSI;
            } else {
                let (r, exact) = self.value.round_up(0);
                if exact {
                    rounded = Some(r);
                } else {
                    format = Format::DecimalSI;
                }
            }
        }
        if let Some(r) = rounded {
            // r.scale is 0 or negative: make it an integer.
            let mut amount = &r.unscaled * pow10(-r.scale);
            let mut times = 0;
            let k = BigInt::from(1024);
            while !amount.is_zero() && (&amount % &k).is_zero() {
                amount /= &k;
                times += 1;
            }
            let suffix = match times {
                1 => "Ki",
                2 => "Mi",
                3 => "Gi",
                4 => "Ti",
                5 => "Pi",
                6 => "Ei",
                _ => "",
            };
            return format!("{amount}{suffix}");
        }
        let mut amount = self.value.unscaled.clone();
        let mut exponent = -self.value.scale;
        let ten = BigInt::from(10);
        while !amount.is_zero() && (&amount % &ten).is_zero() {
            amount /= &ten;
            exponent += 1;
        }
        // Go's `%` truncates, as Rust's does.
        while exponent % 3 != 0 {
            amount *= &ten;
            exponent -= 1;
        }
        let suffix = match format {
            Format::DecimalExponent if exponent == 0 => String::new(),
            Format::DecimalExponent => format!("e{}", exponent as i32),
            // Upstream finds no suffix past E (or below n) and prints none.
            _ => match exponent {
                -9 => "n",
                -6 => "u",
                -3 => "m",
                3 => "k",
                6 => "M",
                9 => "G",
                12 => "T",
                15 => "P",
                18 => "E",
                _ => "",
            }
            .into(),
        };
        format!("{amount}{suffix}")
    }

    /// The value in milli-units, saturating (for comparisons).
    pub fn milli_value(&self) -> i64 {
        let (r, _) = self.value.round_up(3);
        let v = r.unscaled * pow10(3 - r.scale.min(3));
        v.to_i64()
            .unwrap_or(if v.is_negative() { i64::MIN } else { i64::MAX })
    }
}

/// A quantity's JSON value (string or number), canonicalized as a typed
/// round trip does; with `round_up`, rounded up to milli units first, as
/// `SetDefaults_ResourceList` does.
pub fn canonicalize_json(v: &serde_json::Value, round_up: bool) -> Result<String, &'static str> {
    let mut q = match v {
        serde_json::Value::String(s) => Quantity::from_json_str(s)?,
        // Go parses the number's JSON text; serde_json keeps integers
        // exactly, and floats as their shortest round-trip text.
        serde_json::Value::Number(n) => Quantity::parse(&n.to_string())?,
        _ => return Err(ERR_FORMAT_WRONG),
    };
    if round_up {
        q.round_up(-3);
    }
    Ok(q.canonical())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(s: &str) -> String {
        Quantity::parse(s)
            .map(|q| q.canonical())
            .unwrap_or_else(|e| e.into())
    }

    fn r(s: &str) -> String {
        let mut q = Quantity::parse(s).unwrap();
        q.round_up(-3);
        q.canonical()
    }

    #[test]
    fn canonical_forms() {
        for (i, want) in [
            ("1000m", "1"),
            ("+512", "+512"),
            ("0512", "0512"),
            ("1000E", "1"),
            (".", "0"),
            ("-", "0"),
            ("1.5Gi", "1536Mi"),
            ("1024Ki", "1Mi"),
            ("1Ki", "1Ki"),
            ("1025Ki", "1025Ki"),
            ("0.5", "500m"),
            ("100m", "100m"),
            ("1e3", "1e3"),
            ("1E3", "1E3"),
            ("10e3", "10e3"),
            ("1000e0", "1e3"),
            ("0.5Ki", "512"),
            ("9Ei", "9223372036854775807"),
            ("1.5n", "2n"),
            (".e-12", ERR_NUMERIC),
            ("", ERR_FORMAT_WRONG),
            ("1x", ERR_FORMAT_WRONG),
            ("1kk", ERR_SUFFIX),
            ("-1Ki", "-1Ki"),
            ("2000k", "2M"),
        ] {
            assert_eq!(c(i), want, "{i}");
        }
    }

    #[test]
    fn rounding() {
        assert_eq!(r("1n"), "1m");
        assert_eq!(r("-1n"), "-1m");
        assert_eq!(r("1.0001"), "1001m");
        assert_eq!(r("+512"), "+512");
        assert_eq!(r("100m"), "100m");
        assert_eq!(r("1500u"), "2m");
    }

    #[test]
    fn milli_values() {
        assert_eq!(Quantity::parse("1").unwrap().milli_value(), 1000);
        assert_eq!(Quantity::parse("1500u").unwrap().milli_value(), 2);
        assert_eq!(Quantity::parse("1Ki").unwrap().milli_value(), 1_024_000);
    }
}
