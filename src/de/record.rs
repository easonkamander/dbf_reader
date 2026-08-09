use super::{Cell, Column};
use crate::{Error, Result};
use serde::{de, forward_to_deserialize_any};

/// The current row in a [`Document`](super::Document).
pub struct Record {
    pub fields: Vec<Column>,
    pub buffer: Vec<u8>,
}

impl Record {
    pub fn parse(header: Vec<u8>) -> Result<Self> {
        let chunks = header.chunks_exact(32);
        let remain = chunks.remainder();
        if remain != b"\x0D" {
            return Err(Error::HeaderRemain(remain.to_vec()));
        }

        let mut index = 1;
        let fields = chunks
            .map(|chunk| Column::parse(chunk, &mut index))
            .collect::<Result<Vec<_>>>()?;

        let buffer = vec![0; index];
        Ok(Self { fields, buffer })
    }

    pub fn alive(&self) -> bool {
        self.buffer.starts_with(b" ")
    }

    fn cells<'a>(&'a self) -> CellPeek<'a> {
        CellPeek {
            fields: self.fields.iter(),
            buffer: &self.buffer,
            active: None,
        }
    }
}

struct CellPeek<'a> {
    fields: core::slice::Iter<'a, Column>,
    buffer: &'a [u8],
    active: Option<&'a Column>,
}

impl<'a> CellPeek<'a> {
    fn cell(&self) -> Option<Cell<'a>> {
        self.active.map(|field| Cell {
            kind: field.kind,
            data: &self.buffer[field.zone.clone()],
        })
    }
}

impl<'de, 'a> de::MapAccess<'de> for CellPeek<'a> {
    type Error = Error;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        self.active = self.fields.next();
        if let Some(field) = self.active {
            use de::value::StrDeserializer as Str;
            seed.deserialize(Str::new(&field.name)).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(
            self.cell()
                .expect("next_value called on map without next_key"),
        )
    }
}

impl<'de, 'a> de::SeqAccess<'de> for CellPeek<'a> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        use de::MapAccess;
        if let Some(de::IgnoredAny) = self.next_key()? {
            Ok(Some(self.next_value_seed(seed)?))
        } else {
            Ok(None)
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a Record {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(core::str::from_utf8(&self.buffer)?)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(core::str::from_utf8(&self.buffer)?.to_owned())
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bytes(&self.buffer)
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_byte_buf(self.buffer.to_owned())
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(self.cells())
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(self.cells())
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool
        i8 i16 i32 i64
        u8 u16 u32 u64
        f32 f64
        char identifier
        enum
    }
}
