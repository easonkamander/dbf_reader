mod iter_lend;
use iter_lend::IterLend;

mod bytes_iter;
use bytes_iter::BytesIter;

mod record_iter;
use record_iter::RecordIter;

use super::Field;
use crate::{Error, Result};
use std::io::Read;

pub struct Document<File> {
    head: Vec<Field>,
    file: File,
    record_bytes: usize,
    record_count: usize,
}

impl<File: Read> Document<File> {
    pub fn new(mut file: File) -> Result<Self> {
        let mut header = [0; 32];
        file.read_exact(&mut header)?;

        let record_count = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let header_bytes = u16::from_le_bytes(header[8..10].try_into().unwrap()) as usize;
        let record_bytes = u16::from_le_bytes(header[10..12].try_into().unwrap()) as usize;
        let header_bytes = header_bytes.checked_sub(32).ok_or(Error::HeaderLength)?;

        let mut header = vec![0; header_bytes];
        file.read_exact(&mut header)?;

        let chunks = header.chunks_exact(32);
        let remain = chunks.remainder();
        if remain != b"\x0D" {
            return Err(Error::HeaderRemain(remain.to_vec()));
        }

        let head = chunks.map(Field::new).collect::<Result<Vec<_>>>()?;
        if head.iter().map(|f| f.size).sum::<usize>() + 1 != record_bytes {
            let sizes = head.iter().map(|f| f.size).collect();
            return Err(Error::RecordLength(sizes, record_bytes));
        }

        Ok(Self {
            head,
            file,
            record_bytes,
            record_count,
        })
    }
}

impl<File> Document<File> {
    pub fn records<'de, De>(&'de mut self) -> RecordIter<'de, BytesIter<&'de mut File>, De> {
        RecordIter::new(
            &self.head,
            BytesIter::new(&mut self.file, self.record_bytes, self.record_count),
        )
    }
}

pub fn from_file<File: Read>(file: File) -> Result<Document<File>> {
    Document::new(file)
}
