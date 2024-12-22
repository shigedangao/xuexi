use std::collections::BTreeMap;

#[derive(Default, Clone, Debug)]
pub struct Word {
    pub written: Vec<String>,
    pub pronunciations: Vec<String>,
    pub translations: Vec<String>,
    pub count: i128,
}

pub trait WordParser {
    /// Parse a sentence into words by breaking the sentence using a list of punctuations.
    /// The parsing will be done using a dictionary
    fn parse_sentence_into_words<S: AsRef<str>>(&self, sentence: S) -> BTreeMap<String, Word>;
    /// Update word count update the counter provided hashmap
    ///
    /// # Arguments
    /// * `map` - &mut HashMap<String, i128>
    /// * `word` - S
    fn insert_word<S: AsRef<str>>(
        &self,
        map: &mut BTreeMap<String, Word>,
        word_str: S,
        word: Word,
    ) {
        let mut word = word.clone();
        match map.get_mut(word_str.as_ref()) {
            Some(w) => {
                w.count += 1;
            }
            None => {
                word.count = 1;
                map.insert(word_str.as_ref().to_string(), word.clone());
            }
        };
    }
}
