use crate::{Error, Result};
use core::ops::Range;

/// The data type of a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Integer,
    Decimal,
    Text,
    Bool,
    Date,
    Memo,
}

impl CellKind {
    pub fn parse(code: u8, dots: u8) -> Result<Self> {
        let dot_free = match code {
            b'N' => None,
            b'C' => Some(CellKind::Text),
            b'L' => Some(CellKind::Bool),
            b'D' => Some(CellKind::Date),
            b'M' => Some(CellKind::Memo),
            c => return Err(Error::UnknownCellType(c)),
        };

        match (dot_free, dots) {
            (Some(kind), 0) => Ok(kind),
            (Some(kind), 1) => Err(Error::ContainsDots(kind)),
            (None, 0) => Ok(CellKind::Integer),
            (None, 1) => Ok(CellKind::Decimal),
            (_, n) => Err(Error::MultipleDots(n)),
        }
    }
}

/// The metadata for a field.
pub struct Column {
    pub name: String,
    pub kind: CellKind,
    pub zone: Range<usize>,
}

impl Column {
    pub fn parse(chunk: &[u8], index: &mut usize) -> Result<Self> {
        assert_eq!(chunk.len(), 32);

        let prefix = match core::ffi::CStr::from_bytes_until_nul(&chunk[..11]) {
            Err(_) => core::str::from_utf8(&chunk[..11]),
            Ok(xs) => xs.to_str(),
        };

        let zone_lhs = *index;
        *index += chunk[16] as usize;
        let zone_rhs = *index;

        Ok(Column {
            name: prefix?.to_owned(),
            kind: CellKind::parse(chunk[11], chunk[17])?,
            zone: zone_lhs..zone_rhs,
        })
    }
}
