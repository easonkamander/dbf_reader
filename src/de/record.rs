mod cell_peek;
mod deser;
use super::Field;
use crate::de::cell::Cell;

pub struct Record<'h, 'r> {
    schema: &'h [Field],
    record: &'r [u8],
}

impl<'h, 'r> Record<'h, 'r> {
    pub const fn new(schema: &'h [Field], record: &'r [u8]) -> Self {
        Self { schema, record }
    }
}

impl<'h, 'r> Iterator for Record<'h, 'r> {
    type Item = (&'h str, Cell<'r>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(field) = self.schema.get(0) {
            self.schema = &self.schema[1..];

            let data = &self.record[..field.size];
            self.record = &self.record[field.size..];

            Some((&field.name, Cell::new(field.kind, data)))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.schema.len();
        (size, Some(size))
    }
}

impl<'h, 'r> core::iter::FusedIterator for Record<'h, 'r> {}

impl<'h, 'r> ExactSizeIterator for Record<'h, 'r> {}
