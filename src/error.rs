use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum LibError {
    Serialize(String),
    Utf8(String)
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(msg) => write!(f, "Fail to serialize due to: {msg}"),
            Self::Utf8(msg) => write!(f, "Fail to convert utf-8 to string: {msg}")
        }
    }
}

impl std::error::Error for LibError{}

impl From<csv::Error> for LibError {
    fn from(err: csv::Error) -> Self {
        LibError::Serialize(err.to_string())
    }
}

impl From<FromUtf8Error> for LibError {
    fn from(err: FromUtf8Error) -> Self {
        LibError::Utf8(err.to_string())   
    }
}