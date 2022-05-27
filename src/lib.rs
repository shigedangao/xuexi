mod character;
mod cedict;

use character::Characters;
use cedict::Dictionnary;

trait Char<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(self) -> Vec<T>;
}

pub async fn get_character_by_usage(content: &str) -> Vec<(char, i128)> {
    let mut handler = Characters::new(content);
    handler.generate_characters_list().await;
    
    handler.get_ordered_characters()
}

pub fn load_dictionnary() -> Dictionnary {
    Dictionnary::new()
}
