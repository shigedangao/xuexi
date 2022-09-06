mod character;
mod clean;
mod punctuation;
pub mod dictionary;
pub mod export;
pub mod word;
pub mod error;
pub mod definition;
pub mod ordering;
#[cfg(feature = "chinese")]
pub mod chinese;
#[cfg(feature = "laotian")]
pub mod laotian;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use dictionary::{ChineseVersion, Chinese};
#[cfg(feature = "laotian")]
use dictionary::Laotian;
#[allow(unused_imports)]
use dictionary::DictionaryLoader;

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
pub fn load_chinese_dictionary(version: ChineseVersion) -> Result<dictionary::Dictionary<Chinese>, error::LibError> {
    let mut dictionary = dictionary::Dictionary::<Chinese>::new_lang()?;
    dictionary.set_chinese_version(version);
    dictionary.load()?;

    Ok(dictionary)
}

/// Load a laotian dictionnary which allows you to get a list of laotian word definitions
#[cfg(feature = "laotian")]
pub fn load_laotian_dictionary() -> Result<dictionary::Dictionary<Laotian>, error::LibError> {
    let mut dictionary = dictionary::Dictionary::<Laotian>::new_lang()?;
    dictionary.load()?;

    Ok(dictionary)
}
