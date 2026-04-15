use crate::Result;
use super::IterLend;
use std::io::Read;

pub struct BytesIter<'n, File> {
    file: File,
    count: &'n mut usize,
    buffer: Vec<u8>,
}

impl<'n, File> BytesIter<'n, File> {
    pub fn new(file: File, bytes: usize, count: &'n mut usize) -> Self {
        BytesIter {
            file,
            count,
            buffer: vec![0; bytes],
        }
    }
}

impl<'n, 't, File: Read> IterLend<'t> for BytesIter<'n, File> {
    type Item = &'t [u8];

    fn next(&'t mut self) -> Result<Option<&'t [u8]>> {
        while let Some(remaining) = self.count.checked_sub(1) {
            *self.count = remaining;

            self.file.read_exact(&mut self.buffer)?;
            if self.buffer.starts_with(b" ") {
                return Ok(Some(&self.buffer[1..]));
            }
        }

        Ok(None)
    }
}
