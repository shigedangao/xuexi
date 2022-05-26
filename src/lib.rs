mod character;
mod cedict;

use character::Characters;

pub async fn get_character_by_usage(content: &str) -> Vec<(char, i128)> {
    let mut handler = Characters::new(content);
    handler.generate_characters_list().await;
    
    handler.get_ordered_characters()
}

pub async fn get_words_by_usage(content: &str) -> Vec<(String, i128)> {
    Vec::new()
}