use crate::dictionary::{Dictionary, Initializer, Lang, Laotian};
use crate::error::DictionaryError;
use crate::word::{Word, WordParser, WordParserResult};
use crate::{punctuation, util};
use chamkho::wordcut_engine::create_prefix_tree;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::path::PathBuf;

/// Used for parsing the dictionnary
#[derive(Debug, Clone, Deserialize)]
pub struct JPEnLaoItem {
    #[serde(rename(deserialize = "LaoWord"))]
    lao: String,
    #[serde(rename(deserialize = "Pronunciation"))]
    phonetic: String,
    #[serde(rename(deserialize = "English"))]
    english: String,
}

impl Initializer<Laotian> for Dictionary<Laotian> {
    /// Create a new dictionnary and load the chamkho parser which is used to found the word in a laotian sentence
    ///
    /// # Arguments
    /// * `params` - Lang
    fn initialize(_: Lang) -> Result<Dictionary<Laotian>, DictionaryError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Dictionary {
            _lang: std::marker::PhantomData::<Laotian>,
            dict: HashMap::new(),
            punctuation: p.laotian,
            params: Lang::Laotian(None),
        })
    }

    /// Load the laotian dictionnary. The chamkho library does not provide a set of definitions for words
    /// Hence we're using a different asset for getting the definition of each word. So far we're using
    /// this definition available here
    ///
    /// @link http://srachai.web.fc2.com
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    fn load(&mut self, path: PathBuf) -> Result<(), DictionaryError> {
        let mut dict = HashMap::new();
        let mut chamkho_tree = Vec::new();

        // reading the csv
        let mut reader = csv::Reader::from_path(path)?;
        for str_record in reader.deserialize::<JPEnLaoItem>() {
            let Ok(record) = str_record else {
                continue;
            };

            let key = record.lao.trim().to_string();
            // create a definition from the record
            let def = Word {
                written: vec![key.clone()],
                pronunciations: vec![record.phonetic.trim().to_string()],
                translations: vec![record.english.trim().to_string()],
                count: 0,
            };

            chamkho_tree.push(key.clone());

            dict.insert(key, def);
        }

        // Create the chamkho parser instance from the laotian word that has been founded
        let prefix_tree: Vec<&str> = chamkho_tree.iter().map(|d| d.deref()).collect();
        let tree = create_prefix_tree(&prefix_tree);
        let wordcut = chamkho::Wordcut::new(tree);

        self.dict = dict;
        self.params = Lang::Laotian(Some(Box::new(wordcut)));

        Ok(())
    }
}

impl WordParser for Dictionary<Laotian> {
    fn parse_sentence_into_words<S: AsRef<str>>(&self, sentence: S) -> WordParserResult {
        let mut founded = BTreeMap::new();
        // clean the string first
        let cleaned_sentence = util::clean_sentence(sentence.as_ref(), &self.punctuation);

        // get a list of laotian word from the sentence
        let Lang::Laotian(parser) = &self.params else {
            return founded;
        };

        let parser = parser.as_ref().expect("Expect to have the parser set");
        let words = parser.segment_into_strings(&cleaned_sentence);
        for word in words {
            if let Some(item) = self.dict.get(&word) {
                founded.insert(word.clone(), item.to_owned());
                // add to the counter
                self.insert_word(&mut founded, word, item.clone());
            }
        }

        founded
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static DICTIONARY: LazyLock<Dictionary<Laotian>> = LazyLock::new(|| {
        let mut dictionnary = Dictionary::<Laotian>::initialize(Lang::Laotian(None)).unwrap();
        dictionnary
            .load(PathBuf::from("./lao-eng-dictionary.csv"))
            .unwrap();

        dictionnary
    });

    #[test]
    fn expect_to_load_lao_dictionnary() {
        let mut dictionnary = Dictionary::<Laotian>::initialize(Lang::Laotian(None)).unwrap();
        let res = dictionnary.load(PathBuf::from("./lao-eng-dictionary.csv"));

        assert!(res.is_ok());
    }

    #[test]
    fn expect_to_get_item() {
        let item = DICTIONARY.dict.get("ຮັກ");
        assert!(item.is_some());

        let item = item.unwrap();
        assert_eq!(item.written.first().unwrap(), "ຮັກ");
        assert_eq!(item.pronunciations.get(0).unwrap(), "hak");
        assert_eq!(item.translations.get(0).unwrap(), "love");
    }

    #[test]
    fn expect_to_get_list_word_for_sentence() {
        let words = DICTIONARY.parse_sentence_into_words("ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ");
        let baby = words.get("ລູກຫລ້າ");
        assert!(baby.is_some());

        let baby = baby.unwrap();
        assert_eq!(baby.written.first().unwrap(), "ລູກຫລ້າ");
        assert_eq!(baby.pronunciations.first().unwrap(), "lù:k lar");
        assert_eq!(baby.translations, vec!["youngest child"]);
    }

    #[test]
    fn expect_to_not_match_anything() {
        let words = DICTIONARY.parse_sentence_into_words("hello");
        assert!(words.is_empty());
    }
}
