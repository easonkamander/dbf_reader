use super::Record;
use crate::{Error, Result};
use fallible_streaming_iterator::FallibleStreamingIterator;
use serde::de;
use std::io::Read;

pub fn from_file<File: Read>(file: File) -> Result<Document<File>> {
    Document::new(file)
}

/// A .dbf file as it is being streamed.
pub struct Document<File> {
    record: Record,
    count: usize,
    file: File,
}

impl<File: Read> Document<File> {
    pub fn new(mut file: File) -> Result<Self> {
        let mut header = [0; 32];
        file.read_exact(&mut header)?;

        let count = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let header_size = u16::from_le_bytes(header[8..10].try_into().unwrap()) as usize;
        let record_size = u16::from_le_bytes(header[10..12].try_into().unwrap()) as usize;
        let header_size = header_size.checked_sub(32).ok_or(Error::HeaderLength)?;

        let mut header = vec![0; header_size];
        file.read_exact(&mut header)?;
        let record = Record::parse(header)?;

        if record.buffer.len() == record_size {
            Ok(Self {
                record,
                count,
                file,
            })
        } else {
            Err(Error::RecordLength(record.buffer.len(), record_size))
        }
    }

    pub fn as_iter<'a, 'de, D: de::Deserialize<'de>>(
        &'a mut self,
    ) -> impl Iterator<Item = Result<D>> {
        use crate::map_clone::WithMapClone;
        self.map_clone(|r| D::deserialize(r?))
    }
}

impl<File: Read> FallibleStreamingIterator for Document<File> {
    type Item = Record;
    type Error = std::io::Error;

    fn advance(&mut self) -> core::result::Result<(), std::io::Error> {
        while let Some(count) = self.count.checked_sub(1) {
            self.count = count;

            self.file.read_exact(&mut self.record.buffer)?;

            if self.record.alive() {
                break;
            }
        }

        Ok(())
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.record.alive() {
            Some(&self.record)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}
