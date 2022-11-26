use std::fmt;

#[derive(Clone, Debug)]
pub enum BitMaskError {
    IndexOutOfBounds,
    DeserializationFailed,
}

impl fmt::Display for BitMaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitMaskError::IndexOutOfBounds => write!(f, "IndexOutOfBounds"),
            BitMaskError::DeserializationFailed => write!(f, "DeserializationFailed"),
        }
    }
}
