use super::{Field, IterLend};
use crate::Result;
use crate::de::record::Record;
use core::marker::PhantomData;
use serde::de;

pub struct RecordIter<'h, In, De> {
    head: &'h [Field],
    iter: In,
    _der: PhantomData<De>,
}

impl<'h, In, De> RecordIter<'h, In, De> {
    pub const fn new(head: &'h [Field], iter: In) -> Self {
        Self {
            head,
            iter,
            _der: PhantomData,
        }
    }
}

impl<'h, 'r, In, De> IterLend<'r> for RecordIter<'h, In, De>
where
    In: IterLend<'r, Item = &'r [u8]>,
    De: de::Deserialize<'r>,
{
    type Item = De;

    fn next(&'r mut self) -> Result<Option<Self::Item>> {
        if let Some(item) = self.iter.next()? {
            let record = Record::new(self.head, item);
            let custom = De::deserialize(record)?;
            Ok(Some(custom))
        } else {
            Ok(None)
        }
    }
}

impl<'h, In, De> Iterator for RecordIter<'h, In, De>
where
    In: for<'r> IterLend<'r, Item = &'r [u8]>,
    De: de::DeserializeOwned,
{
    type Item = Result<De>;

    fn next(&mut self) -> Option<Self::Item> {
        IterLend::next(self).transpose()
    }
}
