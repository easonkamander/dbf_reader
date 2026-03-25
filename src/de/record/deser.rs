use super::Record;
use super::cell_peek::CellPeek;
use crate::{Error, Result};
use serde::de;

impl<'h, 'r> de::Deserializer<'r> for Record<'h, 'r> {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_i8<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_i16<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_i32<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_i64<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_u8<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_u16<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_u32<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_u64<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_f32<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_f64<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_char<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedSimpleRecord)
    }

    fn deserialize_str<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(core::str::from_utf8(self.record)?)
    }

    fn deserialize_string<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(core::str::from_utf8(self.record)?.to_owned())
    }

    fn deserialize_bytes<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bytes(self.record)
    }

    fn deserialize_byte_buf<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_byte_buf(self.record.to_owned())
    }

    fn deserialize_identifier<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_option<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: de::Visitor<'r>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'r>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(CellPeek::from(self))
    }

    fn deserialize_tuple<V: de::Visitor<'r>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'r>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(CellPeek::from(self))
    }

    fn deserialize_struct<V: de::Visitor<'r>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: de::Visitor<'r>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        Err(Error::HintedEnum)
    }
}
