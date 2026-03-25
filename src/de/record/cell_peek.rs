use super::{Cell, Record};
use crate::{Error, Result};
use serde::de;

pub struct CellPeek<'h, 'r> {
    iter: Record<'h, 'r>,
    save: Option<Cell<'r>>,
}

impl<'h, 'r> From<Record<'h, 'r>> for CellPeek<'h, 'r> {
    fn from(value: Record<'h, 'r>) -> Self {
        Self {
            iter: value,
            save: None,
        }
    }
}

impl<'h, 'r> de::MapAccess<'r> for CellPeek<'h, 'r> {
    type Error = Error;

    fn next_key_seed<K: de::DeserializeSeed<'r>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if let Some((name, cell)) = self.iter.next() {
            self.save = Some(cell);
            use de::value::StrDeserializer as Str;
            seed.deserialize(Str::new(name)).map(Some)
        } else {
            self.save = None;
            Ok(None)
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'r>>(&mut self, seed: V) -> Result<V::Value> {
        let cell = self.save.take().unwrap();
        seed.deserialize(cell)
    }
}

impl<'h, 'r> de::SeqAccess<'r> for CellPeek<'h, 'r> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'r>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        use de::MapAccess;
        if let Some(de::IgnoredAny) = self.next_key()? {
            self.next_value_seed(seed).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}
