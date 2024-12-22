use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DictionaryError {
    Serialize(String),
    Utf8(String),
    ChineseDictionary(String),
    LaoDictionary(String),
    Puncutation(String),
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::Serialize(msg) => write!(f, "Fail to serialize due to: {msg}"),
            DictionaryError::Utf8(msg) => write!(f, "Fail to convert utf-8 to string: {msg}"),
            DictionaryError::ChineseDictionary(msg) => {
                write!(f, "Fail to load chinese dinctionary: {msg}")
            }
            DictionaryError::LaoDictionary(msg) => write!(f, "Fail to load lao dictionary: {msg}"),
            DictionaryError::Puncutation(msg) => {
                write!(f, "Unable to retrieve the punctuation: {msg}")
            }
        }
    }
}

impl std::error::Error for DictionaryError {}

impl From<csv::Error> for DictionaryError {
    fn from(err: csv::Error) -> Self {
        DictionaryError::Serialize(err.to_string())
    }
}

impl From<FromUtf8Error> for DictionaryError {
    fn from(err: FromUtf8Error) -> Self {
        DictionaryError::Utf8(err.to_string())
    }
}

impl From<serde_json::Error> for DictionaryError {
    fn from(err: serde_json::Error) -> Self {
        DictionaryError::Puncutation(err.to_string())
    }
}
