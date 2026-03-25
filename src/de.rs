mod cell;
mod document;
mod record;

pub use document::{Document, from_file};

use crate::Result;
use arrayvec::ArrayString;
use cell::CellType;

pub struct Field {
    name: ArrayString<11>,
    kind: CellType,
    size: usize,
}

impl Field {
    pub fn new(chunk: &[u8]) -> Result<Self> {
        assert!(chunk.len() == 32);

        let name = core::str::from_utf8(&chunk[..11])?;
        let name = ArrayString::from(name.trim_matches('\x00')).unwrap();
        let kind = CellType::new(chunk[11], chunk[17])?;
        let size = chunk[16] as usize;

        Ok(Field { name, kind, size })
    }
}
