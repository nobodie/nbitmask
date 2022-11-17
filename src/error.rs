use std::fmt;

#[derive(Clone, Debug)]
pub enum BitMaskError {
    IndexOutOfBounds,
}

impl fmt::Display for BitMaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BitMaskError::IndexOutOfBounds => write!(f, "IndexOutOfBounds"),
        }
    }
}
