pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("invalid ASCII for text: {0}")]
    ConvertText(#[from] core::str::Utf8Error),
    #[error("invalid ASCII for integer: {0}")]
    ConvertInt(#[from] core::num::ParseIntError),
    #[error("invalid ASCII for decimal: {0}")]
    ConvertDec(#[from] core::num::ParseFloatError),
    #[error("invalid ASCII for date: {0}")]
    ConvertDate(#[from] chrono::ParseError),

    #[error("unable to read file: {0}")]
    ReadFile(#[from] std::io::Error),

    #[error("invalid header: shorter than 32 bytes")]
    HeaderLength,
    #[error("invalid header: expected terminator [0x0D], found {0:?}")]
    HeaderRemain(Vec<u8>),
    #[error("invalid header: field sizes totalling {0} must match {1}")]
    RecordLength(usize, usize),

    #[error("invalid field type: unrecognized code {0}")]
    UnknownCellType(u8),
    #[error("invalid field type: field with type {0:?} contains a decimal point")]
    ContainsDots(super::CellKind),
    #[error("invalid field type: numeric field contains {0} decimal points")]
    MultipleDots(u8),

    #[error("invalid ASCII for char: both '{0}' and '{1}'")]
    InvalidChar(char, char),
    #[error("invalid ASCII for bool: {0}")]
    InvalidBool(String),

    #[error("failed unwrapping null bool")]
    UnwrapBool,
    #[error("failed unwrapping null date")]
    UnwrapDate,
}

impl serde::de::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
