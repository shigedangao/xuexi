use std::collections::HashMap;
use serde::Deserialize;
use chamkho::Wordcut;
use crate::definition::Definition;
use crate::common::{Clean, DetectWord};
use crate::error::LibError;

#[derive(Default)]
pub struct Dictionnary {
    dic: HashMap<String, Definition>,
    parser: Option<Wordcut>,
}

/// Used for parsing the dictionnary
#[derive(Debug, Clone, Deserialize)]
pub struct JPEnLaoItem {
    #[serde(rename(deserialize = "Lao"))]
    lao: String,
    #[serde(rename(deserialize = "phonetic alphabet"))]
    phonetic: String,
    #[serde(rename(deserialize = "English"))]
    english: String
}

impl Dictionnary {
    /// Create a new dictionnary and load the chamkho parser which
    /// is used to found the word in a laotian sentence
    pub fn new() -> Result<Self, LibError> {
        // preload the wordcut dictionnary 
        let lao_dic_path = chamkho::lao_path();
        let dic = chamkho::load_dict(lao_dic_path)
            .map_err(|err| LibError::LaoDictionnary(err.to_string()))?;

        let wordcut = chamkho::Wordcut::new(dic);

        Ok(Dictionnary {
            dic: HashMap::new(),
            parser: Some(wordcut),  
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
    pub fn load(&mut self) {
        let mut dic = HashMap::new();
        let resource: &[u8] = include_bytes!("../../jp-lao-en.csv");

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
                writing_method_two: None,
                prounciation: record.phonetic.trim().to_string(),
                english: record.english.trim().to_string(),
                count: 0
            };

            dic.insert(key, def);
        }

        self.dic = dic;
    } 
}

impl Clean for Dictionnary {}

impl DetectWord for Dictionnary {
    fn get_dictionnary(&self) -> &HashMap<String, Definition> {
        &self.dic
    }

    fn get_list_detected_words(&self, sentence: &str) -> Option<HashMap<String, Definition>> {
        let mut matched = HashMap::new();
        // clean the string first 
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence);
        // get a list of laotian word from the sentence
        if self.parser.is_none() {
            return None;
        }
        
        let parser = self.parser.as_ref().unwrap();
        let words = parser.segment_into_strings(&cleaned_sentence);
        if words.is_empty() {
            return None;
        }

        for word in words {
            if let Some(item) = self.dic.get(&word) {
                self.insert_map_word(&mut matched, &Some(item.to_owned()));
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
    use crate::common::Ops;

    #[test]
    fn expect_to_load_lao_dictionnary() {
        let mut dictionnary = Dictionnary::new().unwrap();
        dictionnary.load();

        assert!(!dictionnary.dic.is_empty());
    }

    #[test]
    fn expect_to_get_item() {
        let mut dictionnary = Dictionnary::new().unwrap();
        dictionnary.load();

        let item = dictionnary.dic.get("ຮັກ");
        assert!(item.is_some());

        let item = item.unwrap();
        assert_eq!(item.writing_method, "ຮັກ");
        assert_eq!(item.prounciation, "hak");
        assert_eq!(item.english, "love");
    }

    #[test]
    fn expect_to_get_list_word_for_sentence() {
        let mut dictionnary = Dictionnary::new().unwrap();
        dictionnary.load();

        let words = dictionnary.get_list_detected_words("ລູກຫລ້າຢາກໄດ້ກິນຫຍັງ");
        assert!(words.is_some());
        
        let words = words.unwrap();
        let baby = words.get("ລູກ");
        assert!(baby.is_some());

        let baby = baby.unwrap();
        assert_eq!(baby.writing_method, "ລູກ");
        assert_eq!(baby.prounciation, "luuk");
        assert_eq!(baby.english, "baby");
    }

    #[test]
    fn expect_to_get_list_of_word_by_order() {
        let mut dictionnary = Dictionnary::new().unwrap();
        dictionnary.load();

        let words = dictionnary.get_list_detected_words("ລູກຫລ້າຢາກໄດ້ກິນຫຍັງລູກຢາກກິນເຂົ້າຫນຽວ");
        assert!(words.is_some());

        let ordered_words = words.unwrap().get_ordered_characters();
        let (word, item) = ordered_words.get(0).unwrap();
        
        assert_eq!(item.count, 2);
        assert_eq!(word, "ລູກ");
    }

    #[test]
    fn expect_to_not_match_anything() {
        let mut dictionnary = Dictionnary::new().unwrap();
        dictionnary.load();

        let words = dictionnary.get_list_detected_words("hello");
        assert!(words.is_none());        
    }
}