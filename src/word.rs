use std::collections::HashMap;
use crate::definition::Definition;
use crate::clean::Clean;

pub trait DetectWord: Clean {
    /// Insert a definition in a map
    /// 
    /// # Arguments
    /// 
    /// * `map` - A mutable reference to a HashMap
    /// * `item` - A Definition item which we'll be insert
    fn insert_map_word(&self, map: &mut HashMap<String, Definition>, item: &Option<Definition>, key: &str) {
        if item.is_none() {
            return;
        }

        if let Some(def) = item {
            if let Some(exist) = map.get_mut(key) {
                exist.count += 1;
            } else {
                let mut item_clone = def.clone();
                item_clone.count = 1;
                map.insert(key.to_owned(), item_clone);
            }
        }
    }
    /// Get a list of detected words based on the loaded dictionnary from a given sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - impl AsRef<str> anything that can be converted to a &str
    fn get_list_detected_words(&self, sentence: impl AsRef<str>) -> Option<HashMap<String, Definition>>;
}
