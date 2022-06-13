mod character;
pub mod error;
pub mod definition;
pub mod common;
pub mod chinese;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use chinese::Dictionnary;

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
    let mut dictionnary = Dictionnary::new();
    dictionnary.load();

    dictionnary
}
