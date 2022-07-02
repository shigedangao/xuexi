pub trait Clean {
    /// Remove punctuation from a sentence to avoid being count
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    fn remove_punctuation_from_sentence(&self, sentence: &str, punctuation: &Vec<String>) -> String {
        let mut filtered_sentence = sentence.to_string();
        for pattern in punctuation {
            filtered_sentence = filtered_sentence.replace(pattern, "");
        }

        filtered_sentence
    }
}