use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use csv::Writer;
use crate::common::Ops;
use crate::error::LibError;

// Custom type
pub type DefinitionList = HashMap<String, Definition>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub writing_method: String,
    pub writing_method_two: Option<String>,
    pub pronunciation: String,
    pub english: String,
    pub count: i64
}

pub trait CommonDefinitionLanguage {
    fn get_english_translations(&self) -> Vec<String>;
}

impl Ops<(String, Definition)> for DefinitionList {
    fn get_ordered_characters(&self) -> Vec<(String, Definition)> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter());
        vec.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

        vec
    }

    fn export_to_csv(&self) -> Result<String, LibError> {
        let ordered = self.get_ordered_characters();
        // get the list of definition
        let definitions: Vec<Definition> = ordered.into_iter()
            .map(|(_, d)| d)
            .collect();

        let mut wrt = Writer::from_writer(vec![]);
        wrt.serialize(definitions)?;

        let inner = wrt.into_inner()
            .map_err(|err| LibError::Serialize(err.to_string()))?;

        let res = String::from_utf8(inner)?;

        Ok(res)
    }
}