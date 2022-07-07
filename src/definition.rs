use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::ordering::Ops;
use crate::error::LibError;
use crate::export;

// Custom type
pub type DefinitionList = HashMap<String, Definition>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub writing_method: String,
    pub writing_method_two: Option<String>,
    pub pronunciation: Vec<String>,
    pub translation: Vec<String>,
    pub count: i64
}

/// Use to merge two definitions which has the same key.
/// 
/// For example the cedict dictionary has multiple definition of the character 得
/// To avoid having only the last item to be store in the HashMap. This trait allows
/// to merge the new definition with the older definition. 
/// 
/// This is so far not very fancy, it's just string concat for some field. This might need to change
/// and instead use a Vector instead of a String
pub trait InsertOrMerge {
    /// # Arguments
    /// 
    /// * `key` - String
    /// * `item` - Definition (the new definition)
    fn insert_or_merge(&mut self, key: String, item: Definition);
}

impl Definition {
    pub fn merge_definition(&mut self, item: Self) {
        // merge pronounciation vec
        if let Some(new_pronounciation) = item.pronunciation.get(0) {
            if !self.pronunciation.contains(new_pronounciation) {
                self.pronunciation.push(new_pronounciation.to_owned());
            }
        }

        // merge definitions vec
        if let Some(new_translation) = item.translation.get(0) {
            if !self.translation.contains(new_translation) {
                self.translation.push(new_translation.to_owned());
            }
        }
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
            founded.merge_definition(item)
        } else {
            self.insert(key, item);
        }
    }
}