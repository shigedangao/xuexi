use crate::error::DictionaryError;
use crate::export::{self, Export};
use serde::Serialize;
use std::collections::BTreeMap;

/// Type alias to BTreeMap<String, Word>
pub type WordParserResult = BTreeMap<String, Word>;

#[derive(Default, Clone, Debug, Serialize)]
pub struct Word {
    #[serde(serialize_with = "export::serialize_vec_to_string")]
    pub written: Vec<String>,

    #[serde(serialize_with = "export::serialize_vec_to_string")]
    pub pronunciations: Vec<String>,

    #[serde(serialize_with = "export::serialize_vec_to_string")]
    pub translations: Vec<String>,
    pub count: i128,
}

pub trait WordParser {
    /// Parse a sentence into words by breaking the sentence using a list of punctuations.
    /// The parsing will be done using a dictionary
    fn parse_sentence_into_words<S: AsRef<str>>(&self, sentence: S) -> WordParserResult;
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

impl Export for WordParserResult {
    fn to_csv(&self) -> Result<String, DictionaryError> {
        let items = self.values().collect::<Vec<_>>();

        export::export_to_csv(items)
    }
}
