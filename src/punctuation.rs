use crate::error::DictionaryError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Puncutation {
    pub chinese: Vec<String>,
    pub laotian: Vec<String>,
    pub western: Vec<String>,
}

impl Puncutation {
    /// Create a new puncutation enum from the json list
    pub fn new() -> Result<Self, DictionaryError> {
        let value = include_bytes!("./puncutation.json");
        let ds: Puncutation = serde_json::from_slice(value)?;

        Ok(ds)
    }
}
