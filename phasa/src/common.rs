// Constant
const PUNCTUATION: [&str; 10] = [".", "?", "!", ",", "...", "《", "》", "。", "\n", ";"];

pub trait Ordered<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(&self) -> Vec<T>;
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