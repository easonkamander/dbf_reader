mod deser;
use crate::{Error, Result};
use chrono::NaiveDate;

#[derive(Clone, Copy)]
pub enum CellType {
    Integer,
    Decimal,
    Text,
    Bool,
    Date,
    Memo,
}

impl CellType {
    pub fn new(code: u8, dots: u8) -> Result<Self> {
        match (code, dots) {
            (b'N' | b'F', 1) => Ok(CellType::Decimal),
            (b'N' | b'F', 0) => Ok(CellType::Integer),
            (b'C', 0) => Ok(CellType::Text),
            (b'L', 0) => Ok(CellType::Bool),
            (b'D', 0) => Ok(CellType::Date),
            (b'M', 0) => Ok(CellType::Memo),
            _ => Err(Error::UnknownCellType(code, dots)),
        }
    }
}

pub struct Cell<'r> {
    meta: CellType,
    data: &'r [u8],
}

impl<'r> Cell<'r> {
    pub const fn new(meta: CellType, data: &'r [u8]) -> Self {
        Self { meta, data }
    }

    fn nonempty(&self) -> bool {
        !self.data.iter().all(u8::is_ascii_whitespace)
    }

    fn text(&self) -> Result<&str> {
        Ok(core::str::from_utf8(self.data)?.trim())
    }

    fn integer(&self) -> Result<i64> {
        Ok(self.text()?.trim_start().parse()?)
    }

    fn decimal(&self) -> Result<f64> {
        Ok(self.text()?.trim_start().parse()?)
    }

    fn bool(&self) -> Result<Option<bool>> {
        match self.data {
            b"T" | b"t" | b"Y" | b"y" => Ok(Some(true)),
            b"F" | b"f" | b"N" | b"n" => Ok(Some(false)),
            b"?" => Ok(None),
            _ => Err(Error::UnknownBool(self.data.to_vec())),
        }
    }

    fn date(&self) -> Result<Option<NaiveDate>> {
        Ok(if self.nonempty() {
            Some(NaiveDate::parse_from_str(self.text()?, "%Y%m%d")?)
        } else {
            None
        })
    }
}
