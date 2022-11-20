use std::collections::BTreeMap;
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};
use crate::ordering::Ops;
use crate::error::LibError;
use crate::export;

// Custom type
pub type DefinitionList = BTreeMap<String, Definition>;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Definition {
    pub writing_method: String,
    pub second_writing_method: Option<String>,
    pub pronunciations: Vec<String>,
    pub translations: Vec<String>,
    pub count: i64,
    pub level: Option<String>
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
        if let Some(new_pronounciation) = item.pronunciations.get(0) {
            if !self.pronunciations.contains(new_pronounciation) {
                self.pronunciations.push(new_pronounciation.to_owned());
            }
        }

        // merge definitions vec
        if let Some(new_translation) = item.translations.get(0) {
            if !self.translations.contains(new_translation) {
                self.translations.push(new_translation.to_owned());
            }
        }
    }
}

impl Ops<Definition> for DefinitionList {
    fn get_ordered_characters(&self) -> Vec<Definition> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter())
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Definition>>();

        vec.sort_by(|a, b| b.count.cmp(&a.count));

        vec
    }
}

impl export::Export for DefinitionList {
    fn to_csv(&self) -> Result<String, LibError> {
        let definitions = self.get_ordered_characters();
        // get the list of definition

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

// Implement Serialize trait as we can't export vector to csv
impl Serialize for Definition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {
        let mut s = serializer.serialize_struct("Definition", 5)?;
        s.serialize_field("writing_method", &self.writing_method)?;
        s.serialize_field("second_writing_method", &self.second_writing_method)?;
        s.serialize_field("pronounciation", &self.pronunciations.join(","))?;
        s.serialize_field("translation", &self.translations.join(","))?;
        s.serialize_field("count", &self.count)?;

        s.end()
    }
}
