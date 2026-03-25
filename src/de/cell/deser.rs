use super::{Cell, CellType};
use crate::{Error, Result};
use serde::de;

impl<'r> de::Deserializer<'r> for Cell<'r> {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        match self.meta {
            CellType::Integer => self.deserialize_u64(visitor),
            CellType::Decimal => self.deserialize_f64(visitor),
            CellType::Text => self.deserialize_str(visitor),
            CellType::Bool => self.deserialize_bool(visitor),
            CellType::Date => self.deserialize_str(visitor),
            CellType::Memo => self.deserialize_u32(visitor),
        }
    }

    fn deserialize_bool<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(match self.meta {
            CellType::Bool => self.bool()?.ok_or(Error::UnwrapBool)?,
            CellType::Integer => self.integer()? != 0,
            CellType::Decimal => self.decimal()? != 0.0,
            _ => self.nonempty(),
        })
    }

    fn deserialize_i8<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.integer()?.try_into()?)
    }

    fn deserialize_i16<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.integer()?.try_into()?)
    }

    fn deserialize_i32<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.integer()?.try_into()?)
    }

    fn deserialize_i64<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.integer()?)
    }

    fn deserialize_u8<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.integer()?.try_into()?)
    }

    fn deserialize_u16<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.integer()?.try_into()?)
    }

    fn deserialize_u32<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.integer()?.try_into()?)
    }

    fn deserialize_u64<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.integer()?.try_into()?)
    }

    fn deserialize_f32<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.decimal()? as f32)
    }

    fn deserialize_f64<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.decimal()?)
    }

    fn deserialize_char<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        let first = *self.data.first().ok_or(Error::EmptyString)?;
        visitor.visit_char(first as char)
    }

    fn deserialize_str<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        match self.meta {
            CellType::Date => {
                let date = self.date()?.ok_or(Error::UnwrapDate)?;
                visitor.visit_str(date.to_string().as_str())
            }
            _ => visitor.visit_str(self.text()?),
        }
    }

    fn deserialize_string<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        match self.meta {
            CellType::Date => {
                let date = self.date()?.ok_or(Error::UnwrapDate)?;
                visitor.visit_string(date.to_string())
            }
            _ => visitor.visit_string(self.text()?.to_owned()),
        }
    }

    fn deserialize_bytes<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bytes(self.data)
    }

    fn deserialize_byte_buf<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_byte_buf(self.data.to_owned())
    }

    fn deserialize_identifier<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_option<V: de::Visitor<'r>>(self, visitor: V) -> Result<V::Value> {
        match self.meta {
            CellType::Bool if self.bool()?.is_some() => visitor.visit_some(self),
            _ if self.nonempty() => visitor.visit_some(self),
            _ => visitor.visit_none(),
        }
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

    fn deserialize_seq<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedComplexCell)
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

    fn deserialize_map<V: de::Visitor<'r>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::HintedComplexCell)
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
