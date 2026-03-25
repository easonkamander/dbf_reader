use crate::Result;
use super::IterLend;
use std::io::Read;

pub struct BytesIter<File> {
    file: File,
    count: usize,
    buffer: Vec<u8>,
}

impl<File> BytesIter<File> {
    pub fn new(file: File, bytes: usize, count: usize) -> Self {
        BytesIter {
            file,
            count,
            buffer: vec![0; bytes],
        }
    }
}

impl<'t, File: Read> IterLend<'t> for BytesIter<File> {
    type Item = &'t [u8];

    fn next(&'t mut self) -> Result<Option<&'t [u8]>> {
        while let Some(remaining) = self.count.checked_sub(1) {
            self.count = remaining;

            self.file.read_exact(&mut self.buffer)?;
            if self.buffer.starts_with(b" ") {
                return Ok(Some(&self.buffer[1..]));
            }
        }

        Ok(None)
    }
}
