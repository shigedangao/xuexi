use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum LibError {
    Serialize(String),
    Utf8(String),
    LaoDictionnary(String),
    Puncutation(String)
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibError::Serialize(msg) => write!(f, "Fail to serialize due to: {msg}"),
            LibError::Utf8(msg) => write!(f, "Fail to convert utf-8 to string: {msg}"),
            LibError::LaoDictionnary(msg) => write!(f, "Fail to load lao dictionnary: {msg}"),
            LibError::Puncutation(msg) => write!(f, "Unable to retrieve the punctuation: {msg}")
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

impl From<serde_json::Error> for LibError {
    fn from(err: serde_json::Error) -> Self {
        LibError::Puncutation(err.to_string())
    }
}
