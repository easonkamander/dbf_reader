use crate::Result;

pub trait IterLend<'t> {
    type Item;

    fn next(&'t mut self) -> Result<Option<Self::Item>>;
}
