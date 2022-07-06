use std::collections::HashMap;
use crate::definition::Definition;
use crate::clean::Clean;

pub trait DetectWord: Clean {
    /// Get a reference of a dictionary
    fn get_dictionary(&self) -> &HashMap<String, Definition>;
    /// Insert a definition in a map
    /// 
    /// # Arguments
    /// 
    /// * `map` - A mutable reference to a HashMap
    /// * `item` - A Definition item which we'll be insert
    fn insert_map_word(&self, map: &mut HashMap<String, Definition>, item: &Option<Definition>) {
        if item.is_none() {
            return;
        }

        let item = item.to_owned().unwrap();
        if let Some(def) = map.get_mut(&item.writing_method) {
            def.count += 1;
        } else {
            let mut item_clone = item.clone();
            item_clone.count = 1;
            map.insert(item.writing_method.clone(), item_clone);
        }
    }
    /// Get a list of detected words based on the loaded dictionnary from a given sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - impl AsRef<str> anything that can be converted to a &str
    fn get_list_detected_words(&self, sentence: impl AsRef<str>) -> Option<HashMap<String, Definition>>;
}

pub trait InsertOrMerge {
    fn insert_or_merge(&mut self, key: String, item: Definition);
}