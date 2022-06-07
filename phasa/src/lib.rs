mod character;
mod common;
#[cfg(feature = "chinese")]
pub mod cedict;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use cedict::Dictionnary;

// Constant
const PUNCTUATION: [&str; 10] = [".", "?", "!", ",", "...", "《", "》", "。", "\n", ";"];

trait Char<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(&self) -> Vec<T>;
}

trait Clean {
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

/// Get a list of characters by it's usage
/// 
/// # Arguments
/// 
/// * `&str` - A string content
pub fn get_character_by_usage(content: &str) -> CharactersList {
    let handler = Characters::new(content);
    
    handler.generate_characters_list()
}

/// Load a chinese dictionnary which allows you to get a list of chinese definitions
#[cfg(feature = "chinese")]
pub fn load_chinese_dictionnary() -> Dictionnary {
    Dictionnary::new()
}
