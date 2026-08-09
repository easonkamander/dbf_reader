use super::CellKind;
use crate::{Error, Result};
use chrono::NaiveDate;
use serde::{de, forward_to_deserialize_any};

fn is_blank(data: &[u8]) -> bool {
    data.iter().all(u8::is_ascii_whitespace)
}

fn as_text_raw(data: &[u8]) -> Result<&str> {
    Ok(core::str::from_utf8(data)?)
}

fn as_text_lhs(data: &[u8]) -> Result<&str> {
    as_text_raw(data).map(str::trim_end)
}

fn as_text_rhs(data: &[u8]) -> Result<&str> {
    as_text_raw(data).map(str::trim_start)
}

fn as_date(data: &[u8]) -> Result<Option<NaiveDate>> {
    if is_blank(data) {
        Ok(None)
    } else {
        let text = as_text_raw(data)?;
        Ok(Some(NaiveDate::parse_from_str(text, "%Y%m%d")?))
    }
}

fn as_bool(data: &[u8]) -> Result<Option<bool>> {
    match data {
        [b'T' | b't' | b'Y' | b'y'] => Ok(Some(true)),
        [b'F' | b'f' | b'N' | b'n'] => Ok(Some(false)),
        [b'?' | b' '] => Ok(None),
        data => Err(Error::InvalidBool(as_text_raw(data)?.to_owned())),
    }
}

fn as_char(data: &[u8]) -> Result<char> {
    let mut state = None;

    for char in as_text_raw(data)?.chars() {
        if !char.is_whitespace()
            && let Some(prev) = state.replace(char)
        {
            return Err(Error::InvalidChar(prev, char));
        }
    }

    Ok(state.unwrap_or(' '))
}

/// The value of a field in a [`Record`](super::Record).
pub struct Cell<'a> {
    pub kind: CellKind,
    pub data: &'a [u8],
}

impl<'de, 'a> de::Deserializer<'de> for Cell<'a> {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.kind {
            CellKind::Integer => self.deserialize_i64(visitor),
            CellKind::Decimal => self.deserialize_f64(visitor),
            CellKind::Text => self.deserialize_str(visitor),
            CellKind::Bool => self.deserialize_bool(visitor),
            CellKind::Date => self.deserialize_str(visitor),
            CellKind::Memo => self.deserialize_u32(visitor),
        }
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(as_bool(self.data)?.ok_or(Error::UnwrapBool)?)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(as_text_rhs(self.data)?.parse()?)
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_char(as_char(self.data)?)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.kind == CellKind::Date {
            match as_date(self.data)? {
                Some(date) => visitor.visit_str(date.to_string().as_str()),
                None => visitor.visit_str(""),
            }
        } else {
            visitor.visit_str(as_text_lhs(self.data)?)
        }
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.kind == CellKind::Date {
            match as_date(self.data)? {
                Some(date) => visitor.visit_string(date.to_string()),
                None => visitor.visit_string(String::new()),
            }
        } else {
            visitor.visit_string(as_text_lhs(self.data)?.to_owned())
        }
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bytes(self.data)
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_byte_buf(self.data.to_owned())
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if is_blank(self.data) || self.kind == CellKind::Bool && as_bool(self.data)?.is_none() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
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

    forward_to_deserialize_any! { seq tuple tuple_struct map struct enum }
}
