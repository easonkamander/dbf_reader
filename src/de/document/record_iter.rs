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

impl<'h, 'r, In, D> IterLend<'r> for RecordIter<'h, In, D>
where
    In: IterLend<'r, Item = &'r [u8]>,
{
    type Item = Record<'h, 'r>;

    fn next(&'r mut self) -> Result<Option<Self::Item>> {
        let item = self.iter.next();
        item.map(|opt| opt.map(|rec| Record::new(self.head, rec)))
    }
}

impl<'h, 'r, In, D: de::DeserializeOwned> Iterator for RecordIter<'h, In, D>
where
    In: for<'t> IterLend<'t, Item = &'t [u8]>,
{
    type Item = Result<D>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = IterLend::next(self).transpose();
        item.map(|res| res.and_then(D::deserialize))
    }
}
