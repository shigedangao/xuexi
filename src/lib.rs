mod character;
mod clean;
mod punctuation;
pub mod export;
pub mod word;
pub mod error;
pub mod definition;
pub mod ordering;
pub mod chinese;
pub mod laotian;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use chinese::Dictionary as CNDictionary;
#[cfg(feature = "laotian")]
use laotian::Dictionary as LaoDictionary;

/// Get a list of characters by it's usage
/// 
/// # Arguments
/// 
/// * `&str` - A string content
pub fn get_character_by_usage(content: impl AsRef<str>) -> Result<CharactersList, error::LibError> {
    let handler = Characters::new(content.as_ref())?;
    
    Ok(handler.generate_characters_list())
}

/// Load a chinese dictionnary which allows you to get a list of chinese definitions
#[cfg(feature = "chinese")]
pub fn load_chinese_dictionary(version: Option<chinese::Version>) -> Result<CNDictionary, error::LibError> {
    let mut dictionary = CNDictionary::new(version)?;
    dictionary.load()?;

    Ok(dictionary)
}

/// Load a laotian dictionnary which allows you to get a list of laotian word definitions
#[cfg(feature = "laotian")]
pub fn load_laotian_dictionary() -> Result<LaoDictionary, error::LibError> {
    let mut dictionary = LaoDictionary::new()?;
    dictionary.load();

    Ok(dictionary)
}
