use std::collections::BTreeMap;
use serde::Deserialize;
use crate::definition::{
    Definition,
    DefinitionList,
    InsertOrMerge
};
use crate::clean::Clean;
use crate::word::DetectWord;
use crate::error::LibError;
use crate::punctuation;
use crate::dictionary::{
    Dictionary,
    DictionaryLoader,
    Laotian,
    Options
};

pub mod wrapper;

/// Used for parsing the dictionnary
#[derive(Debug, Clone, Deserialize)]
pub struct JPEnLaoItem {
    #[serde(rename(deserialize = "LaoWord"))]
    lao: String,
    #[serde(rename(deserialize = "Pronunciation"))]
    phonetic: String,
    #[serde(rename(deserialize = "English"))]
    english: String
}

impl DictionaryLoader<Laotian> for Dictionary<Laotian> {
    /// Create a new dictionnary and load the chamkho parser which
    /// is used to found the word in a laotian sentence
    fn new_lang() -> Result<Dictionary<Laotian>, LibError> {
        let p = punctuation::Puncutation::new()?;
        // preload the wordcut dictionnary 
        let dic = wrapper::load_laotian_words()?;
        let wordcut = chamkho::Wordcut::new(dic);

        Ok(Dictionary {
            lang: std::marker::PhantomData::<Laotian>,
            dic: BTreeMap::new(),
            punctuation: p.laotian,
            options: Options::Laotian(Box::new(wordcut))
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
    fn load(&mut self) -> Result<(), LibError> {
        let mut dic = BTreeMap::new();
        let resource: &[u8] = include_bytes!("../../lao-eng-dictionary.csv");

        // reading the csv
        let mut reader = csv::Reader::from_reader(resource);
        for str_record in reader.deserialize() {
            let record: JPEnLaoItem = match str_record {
                Ok(res) => res,
                Err(_) => continue
            };

            let key = record.lao.trim().to_string();
            // create a definition from the record
            let def = Definition {
                writing_method: key.clone(),
                second_writing_method: None,
                pronunciations: vec![record.phonetic.trim().to_string()],
                translations: vec![record.english.trim().to_string()],
                count: 0,
                level: None
            };

            dic.insert_or_merge(key, def);
        }

        self.dic = dic;

        Ok(())
    } 
}

impl DetectWord for Dictionary<Laotian> {
    fn get_list_detected_words(&self, sentence: impl AsRef<str>) -> Option<DefinitionList> {
        let mut matched = BTreeMap::new();
        // clean the string first 
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence.as_ref(), &self.punctuation);
        
        // get a list of laotian word from the sentence
        let parser = match &self.options {
            Options::Laotian(parser) => parser,
            _ => return None
        };

        let words = parser.segment_into_strings(&cleaned_sentence);
        if words.is_empty() {
            return None;
        }

        for word in words {
            if let Some(item) = self.dic.get(&word) {
                self.insert_map_word(&mut matched, &Some(item.to_owned()), &item.writing_method);
            }
        }

        if matched.is_empty() {
            return None;
        }
        
        Some(matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ordering::Ops;

    #[test]
    fn expect_to_load_lao_dictionnary() {
        let mut dictionnary = Dictionary::<Laotian>::new_lang().unwrap();
        dictionnary.load().unwrap();

        assert!(!dictionnary.dic.is_empty());
    }

    #[test]
    fn expect_to_get_item() {
        let mut dictionnary = Dictionary::<Laotian>::new_lang().unwrap();
        dictionnary.load().unwrap();

        let item = dictionnary.dic.get("ຮັກ");
        assert!(item.is_some());

        let item = item.unwrap();
        assert_eq!(item.writing_method, "ຮັກ");
        assert_eq!(item.pronunciations.get(0).unwrap(), "hak");
        assert_eq!(item.translations.get(0).unwrap(), "love");
    }

    #[test]
    fn expect_to_get_list_word_for_sentence() {
        let mut dictionnary = Dictionary::<Laotian>::new_lang().unwrap();
        dictionnary.load().unwrap();

        let words = dictionnary.get_list_detected_words("ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ");
        assert!(words.is_some());
        
        let words = words.unwrap();
        let baby = words.get("ລູກ");
        assert!(baby.is_some());

        let baby = baby.unwrap();
        assert_eq!(baby.writing_method, "ລູກ");
        assert_eq!(baby.pronunciations.get(0).unwrap(), "lù:k");
        assert_eq!(baby.translations.get(0).unwrap(), "below");
        assert_eq!(baby.translations.last().unwrap(), "downstairs");
    }

    #[test]
    fn expect_to_get_list_of_word_by_order() {
        let mut dictionnary = Dictionary::<Laotian>::new_lang().unwrap();
        dictionnary.load().unwrap();

        let words = dictionnary.get_list_detected_words("ລູກຫລ້າຢາກໄດ້ກິນຫຍັງລູກຢາກກິນເຂົ້າຫນຽວ");
        assert!(words.is_some());

        let ordered_words = words.unwrap().get_ordered_characters();
        let item = ordered_words.get(0).unwrap();
        
        assert_eq!(item.count, 2);
        assert_eq!(item.writing_method, "ລູກ");
    }

    #[test]
    fn expect_to_not_match_anything() {
        let mut dictionnary = Dictionary::<Laotian>::new_lang().unwrap();
        dictionnary.load().unwrap();

        let words = dictionnary.get_list_detected_words("hello");
        assert!(words.is_none());        
    }
}
