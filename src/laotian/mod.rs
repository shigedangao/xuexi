use std::collections::HashMap;
use serde::Deserialize;
use chamkho::Wordcut;
use crate::definition::{Definition, DefinitionList};
use crate::common::{Clean, DetectWord, Ops};
use crate::error::LibError;

pub struct Dictionnary {
    dic: HashMap<String, Definition>,
    parser: Wordcut,
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
    pub fn new() -> Result<Self, LibError> {
        // preload the wordcut dictionnary 
        let lao_dic_path = chamkho::lao_path();
        let dic = chamkho::load_dict(lao_dic_path)
            .map_err(|err| LibError::LaoDictionnary(err.to_string()))?;

        let wordcut = chamkho::Wordcut::new(dic);

        Ok(Dictionnary {
            dic: HashMap::new(),
            parser: wordcut,  
        })
    }

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
        let words = self.parser.segment_into_strings(&cleaned_sentence);
        for word in words {
            if let Some(item) = self.dic.get(&word) {
                self.insert_map_word(&mut matched, &Some(item.to_owned()));
            }
        }
        
        Some(matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        println!("{words:?}");
    }
}