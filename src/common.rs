use std::collections::HashMap;
use crate::{error::LibError, definition::Definition};

// Constant
const PUNCTUATION: [&str; 10] = [".", "?", "!", ",", "...", "《", "》", "。", "\n", ";"];

pub trait Ops<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(&self) -> Vec<T>;
    /// Export a a type to CSV
    ///     - Definitions: In this case this will return a csv of definitions
    ///     - Character count: This is gonna return the character and the count amount 
    fn export_to_csv(&self) -> Result<String, LibError>; 
}

pub trait Clean {
    /// Remove punctuation from a sentence to avoid being count
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    fn remove_punctuation_from_sentence(&self, sentence: &str) -> String {
        let mut filtered_sentence = sentence.to_string();
        for pattern in PUNCTUATION {
            filtered_sentence = filtered_sentence.replace(pattern, "");
        }

        filtered_sentence
    }
}

pub trait DetectWord: Clean {
    fn get_dictionnary(&self) -> &HashMap<String, Definition>;
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
    /// Get a list of detected words based on the loaded cedict dictionnary from a given sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A string slice which represent a sentence
    fn get_list_detected_words(&self, sentence: &str) -> Option<HashMap<String, Definition>>;
}