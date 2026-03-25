pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("unable to read file: {0}")]
    ReadFileIO(#[from] std::io::Error),
    #[error("invalid ASCII for text: {0}")]
    ConvertText(#[from] core::str::Utf8Error),
    #[error("invalid ASCII for integer: {0}")]
    ConvertInt(#[from] core::num::ParseIntError),
    #[error("invalid ASCII for decimal: {0}")]
    ConvertDec(#[from] core::num::ParseFloatError),
    #[error("invalid ASCII for date: {0}")]
    ConvertDate(#[from] chrono::ParseError),
    #[error("failed truncating integer: {0}")]
    ResizeInt(#[from] core::num::TryFromIntError),

    #[error("invalid header: shorter than 32 bytes")]
    HeaderLength,
    #[error("invalid header: expected terminator [0x0D], found {0:?}")]
    HeaderRemain(Vec<u8>),
    #[error("invalid header: column sizes {0:?} + 1 must total {1}")]
    RecordLength(Vec<usize>, usize),

    #[error("unable to deserialize enum")]
    HintedEnum,
    #[error("unable to deserialize record as basic type")]
    HintedSimpleRecord,
    #[error("unable to deserialize cell as complex type")]
    HintedComplexCell,

    #[error("{}", CellTypeError::new(.0, .1))]
    UnknownCellType(u8, u8),
    #[error("invalid ASCII for bool: {0:?}")]
    UnknownBool(Vec<u8>),

    #[error("failed unwrapping null bool")]
    UnwrapBool,
    #[error("failed unwrapping null date")]
    UnwrapDate,

    #[error("failed extracting character from empty string")]
    EmptyString,
}

#[derive(Debug, thiserror::Error)]
struct CellTypeError {
    code: u8,
    dots: u8,
}

impl core::fmt::Display for CellTypeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.code {
            b'N' | b'F' => write!(f, "invalid field type: {} decimal points", self.dots),
            b'C' => write!(f, "invalid field type: decimal point in text"),
            b'L' => write!(f, "invalid field type: decimal point in bool"),
            b'D' => write!(f, "invalid field type: decimal point in date"),
            b'M' => write!(f, "invalid field type: decimal point in memo"),
            _ => write!(f, "invalid field type: unrecognized code {}", self.code),
        }
    }
}

impl CellTypeError {
    fn new(code: &u8, dots: &u8) -> Self {
        CellTypeError {
            code: *code,
            dots: *dots,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
