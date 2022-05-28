mod character;
#[cfg(feature = "chinese")]
mod cedict;

use character::{Characters, CharactersList};
#[cfg(feature = "chinese")]
use cedict::Dictionnary;

trait Char<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(&self) -> Vec<T>;
}

/// Get a list of characters by it's usage
/// 
/// # Arguments
/// 
/// * `&str` - A string content
pub async fn get_character_by_usage(content: &str) -> CharactersList {
    let handler = Characters::new(content);
    
    handler.generate_characters_list().await
}

/// Load a chinese dictionnary which allows you to get a list of chinese definitions
#[cfg(feature = "chinese")]
pub async fn load_chinese_dictionnary() -> Result<Dictionnary, tokio::task::JoinError> {
    let res = tokio::spawn(async move {
        Dictionnary::new()
    }).await;

    res
}
