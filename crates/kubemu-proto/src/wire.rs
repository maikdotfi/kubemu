//! The protobuf wire format, just enough to walk a message: varints,
//! fixed-width values and length-delimited fields. Groups are rejected as
//! the gogo-generated code rejects them.

use crate::Error;

/// One field occurrence, undecoded.
#[derive(Clone, Copy, Debug)]
pub enum Raw<'a> {
    Varint(u64),
    Fixed64(u64),
    /// No built-in type has a fixed32 field; kept only to skip it.
    Fixed32,
    Bytes(&'a [u8]),
}

impl Raw<'_> {
    pub fn wire_type(self) -> u8 {
        match self {
            Raw::Varint(_) => 0,
            Raw::Fixed64(_) => 1,
            Raw::Bytes(_) => 2,
            Raw::Fixed32 => 5,
        }
    }
}

pub fn varint(buf: &[u8], pos: &mut usize) -> Result<u64, Error> {
    let mut v = 0u64;
    for shift in (0..64).step_by(7) {
        let b = *buf.get(*pos).ok_or_else(eof)?;
        *pos += 1;
        v |= u64::from(b & 0x7f) << shift;
        if b < 0x80 {
            return Ok(v);
        }
    }
    Err(Error::Malformed("proto: integer overflow".into()))
}

fn eof() -> Error {
    Error::Malformed("unexpected EOF".into())
}

fn take<'a>(buf: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8], Error> {
    let end = pos
        .checked_add(n)
        .filter(|&e| e <= buf.len())
        .ok_or_else(eof)?;
    let s = &buf[*pos..end];
    *pos = end;
    Ok(s)
}

/// Every `(field number, value)` in `buf`, in wire order.
pub fn fields(buf: &[u8]) -> Result<Vec<(u32, Raw<'_>)>, Error> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos < buf.len() {
        let key = varint(buf, &mut pos)?;
        let num = u32::try_from(key >> 3)
            .ok()
            .filter(|&n| n > 0)
            .ok_or_else(|| Error::Malformed(format!("proto: illegal tag {}", key >> 3)))?;
        let raw = match key & 7 {
            0 => Raw::Varint(varint(buf, &mut pos)?),
            1 => {
                let b = take(buf, &mut pos, 8)?;
                Raw::Fixed64(u64::from_le_bytes(b.try_into().expect("8 bytes")))
            }
            2 => {
                let n = varint(buf, &mut pos)?;
                let n = usize::try_from(n).map_err(|_| eof())?;
                Raw::Bytes(take(buf, &mut pos, n)?)
            }
            5 => {
                take(buf, &mut pos, 4)?;
                Raw::Fixed32
            }
            w => {
                return Err(Error::Malformed(format!(
                    "proto: illegal wireType {w} for field {num}"
                )));
            }
        };
        out.push((num, raw));
    }
    Ok(out)
}

/// The varints of a packed repeated field.
pub fn packed(buf: &[u8]) -> Result<Vec<u64>, Error> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos < buf.len() {
        out.push(varint(buf, &mut pos)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_fields() {
        // field 1 varint 150, field 2 bytes "hi", field 3 fixed64 1
        let buf = [
            0x08, 0x96, 0x01, 0x12, 0x02, b'h', b'i', 0x19, 1, 0, 0, 0, 0, 0, 0, 0,
        ];
        let f = fields(&buf).unwrap();
        assert!(matches!(f[0], (1, Raw::Varint(150))));
        assert!(matches!(f[1], (2, Raw::Bytes(b"hi"))));
        assert!(matches!(f[2], (3, Raw::Fixed64(1))));
    }

    #[test]
    fn rejects_truncation_and_groups() {
        assert!(fields(&[0x12, 0x05, b'a']).is_err());
        assert!(fields(&[0x08]).is_err());
        assert!(fields(&[0x0b]).is_err()); // start group
        assert!(fields(&[0x00]).is_err()); // field 0
        assert!(varint(&[0xff; 11], &mut 0).is_err());
    }
}
