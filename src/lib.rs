mod character;
pub mod error;
pub mod definition;
pub mod common;
pub mod chinese;
pub mod laotian;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use chinese::Dictionnary as CNDictionnary;
#[cfg(feature = "laotian")]
use laotian::Dictionnary as LaoDictionnary;

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
pub fn load_chinese_dictionnary() -> CNDictionnary {
    let mut dictionnary = CNDictionnary::new();
    dictionnary.load();

    dictionnary
}

#[cfg(feature = "laotian")]
pub fn load_laotian_dictionnary() -> Result<LaoDictionnary, error::LibError> {
    let mut dictionnary = LaoDictionnary::new()?;
    dictionnary.load();

    Ok(dictionnary)
}