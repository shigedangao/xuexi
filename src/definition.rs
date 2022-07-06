use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::ordering::Ops;
use crate::error::LibError;
use crate::export;
use crate::word::InsertOrMerge;

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

impl Definition {
    /// Get a vector of english translation from the string representation
    pub fn get_english_translations(&self) -> Vec<String> {
        self.english.split('/')
            .into_iter()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                Some(s.trim().to_string())
            })
            .collect::<Vec<String>>()
    }
}

impl Ops<(String, Definition)> for DefinitionList {
    fn get_ordered_characters(&self) -> Vec<(String, Definition)> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter());
        vec.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

        vec
    }
}

impl export::Export for DefinitionList {
    fn to_csv(&self) -> Result<String, LibError> {
        let ordered = self.get_ordered_characters();
        // get the list of definition
        let definitions: Vec<Definition> = ordered.into_iter()
            .map(|(_, d)| d)
            .collect();

        export::export_to_csv(definitions)
    }
}

impl InsertOrMerge for DefinitionList {
    fn insert_or_merge(&mut self, key: String, item: Definition) {
        if let Some(founded) = self.get_mut(&key) {
            // merge the two english translation
            founded.english = format!("{}/{}", founded.english, item.english);
            // Merge the two pronounciation if it containing a different pronounciation
            if !founded.pronunciation.contains(&item.pronunciation) {
                founded.pronunciation = format!("{}/{}", founded.pronunciation, item.pronunciation);
            }
        } else {
            self.insert(key, item);
        }
    }
}