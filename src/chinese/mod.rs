use std::collections::HashMap;
use serde::Deserialize;
use crate::definition::{Definition, InsertOrMerge};
use crate::word::DetectWord;
use crate::clean::Clean;
use crate::error::LibError;
use crate::punctuation;

// Constant
const SLASH_CHARACTER: char = '/';

#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    Traditional,
    Simplified
}

impl Default for Version {
    fn default() -> Self {
        Version::Traditional
    }
}

#[derive(Debug, Deserialize)]
struct Chinese {
    traditional_chinese: String,
    simplified_chinese: String,
    pinyin: String,
    translations: String
}

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    dic: HashMap<String, Definition>,
    punctuation: Vec<String>,
    version: Version
}


impl Dictionary {
    /// Create a new empty Dictionary
    pub fn new(version: Option<Version>) -> Result<Dictionary, LibError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Dictionary {
            dic: HashMap::default(),
            punctuation: p.chinese,
            version: version.unwrap_or_default()
        })
    }

    /// Create a new Dictionary from the cedict_ts.u8
    pub fn load(&mut self) -> Result<(), LibError> {
        let mut dic = HashMap::new();
        let definition: &[u8] = include_bytes!("../../cedict.csv");
        
        let mut reader = csv::Reader::from_reader(definition);
        for res in reader.deserialize() {
            let record: Chinese = res?;

            let translations: Vec<String> = record.translations
                .split(SLASH_CHARACTER)
                .filter_map(|v| {
                    if !v.is_empty() {
                        return Some(v.to_string())
                    }

                    None
                })
                .collect();

            let item = Definition {
                count: 0,
                writing_method: record.traditional_chinese.to_owned(),
                second_writing_method: Some(record.simplified_chinese.to_owned()),
                pronunciations: vec![record.pinyin],
                translations
            };

            match self.version {
                Version::Simplified => dic.insert_or_merge(record.simplified_chinese, item),
                Version::Traditional =>  dic.insert_or_merge(record.traditional_chinese, item)
            };
        }

        self.dic = dic;

        Ok(())
    }

    fn decorate_insert_map_word(&self, map: &mut HashMap<String, Definition>, item: &Option<Definition>) {
        if let Some(def) = item {
            match self.version {
                Version::Simplified => self.insert_map_word(map, item, def.second_writing_method.as_ref().unwrap()),
                Version::Traditional => self.insert_map_word(map, item, &def.writing_method)
            }
        }
    }
}

impl Clean for Dictionary {}

impl DetectWord for Dictionary {
    fn get_list_detected_words(&self, sentence: impl AsRef<str>) -> Option<HashMap<String, Definition>> {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionary = HashMap::new();
        // split the sentence into a vector of characters
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence.as_ref(), &self.punctuation);
        let characters: Vec<char> = cleaned_sentence.chars().collect();

        // temp definition
        let mut step_def: Option<Definition> = None;
        while let Some(char) = characters.get(start_cursor..end_cursor) {
            // create a word based on the start cursor and the end cursor
            let word: String = char.to_vec().iter().collect();
            match self.dic.get(&word) {
                Some(res) => {
                    step_def = Some(res.clone());
                    if end_cursor == characters.len() {
                        self.decorate_insert_map_word(&mut dictionary, &step_def);
                    }

                    end_cursor += 1;
                    // reset the unmatched flag
                    unmatched = 0;
                },
                None => {
                    // this unmatched is used in case if we're encountering a character which can't be matched
                    // multiple time. If we're unable to find the same character / word for multiple time
                    // then we're increasing the start_cursor & end_cursor in a hope that we'll match something later on...
                    if unmatched > 1 {
                        start_cursor += 1;
                        end_cursor += 1;
                    } else {
                        // Push the latest founded item in the Dictionary
                        self.decorate_insert_map_word(&mut dictionary, &step_def);
                        // if nothing can be found on the Dictionary then we move the start_cursor to end_cursor - 1
                        // this allow us to check the last -1 character again
                        // for example
                        // ???????????? -> at some point the method will check this characters ?????????
                        // the start_cursor will move to 2
                        // the end_cursor will be equal to 3
                        // from these cursors, this will match the character "??? " in the sentence
                        // then it'll continue to move the end_cursor to 4 -> ?????? 
                        // which was matched at the latest (end_cursor)
                        start_cursor = end_cursor - 1;
                    }

                    unmatched += 1;
                }
            }
        }

        if dictionary.is_empty() {
            return None;
        }

        Some(dictionary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ordering::Ops;
    use crate::export::Export;

    #[test]
    fn expect_to_load_dictionary() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        assert!(!dictionary.dic.is_empty());
    }

    #[test]
    fn expect_to_get_same_char_and_different_pronounciation() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let res = dictionary.get_list_detected_words("???").unwrap();
        let dei = res.get("???").unwrap();

        assert_eq!(dei.pronunciations.get(0).unwrap(), "de2");
        assert_eq!(dei.pronunciations.last().unwrap(), "dei3");
    }

    #[test]
    fn expect_to_get_characters() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("????????????").unwrap();
        let nihao = words.get("??????");

        assert!(nihao.is_some());

        let nihao = nihao.unwrap();
        assert_eq!(nihao.count, 2);
        assert_eq!(nihao.writing_method, "??????");
    }

    #[test]
    fn expect_to_get_ordered_characters() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("???????????????????????????????????????????????????").unwrap();
        let riben = words.get("??????").unwrap();
        
        assert_eq!(riben.count, 2);
        assert_eq!(riben.writing_method, "??????");

        let vec = words.get_ordered_characters();
        let (f, item) = vec.first().unwrap();

        assert_eq!(f, "??????");
        assert_eq!(item.count, 2);
        assert_eq!(item.writing_method, "??????");
    }

    #[test]
    fn expect_to_generate_list_with_for_multiple_sentences() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("??????????????????????????????. ???????????????????????????! ?????????????????????").unwrap();
        
        let riben = words.get("??????").unwrap();
        assert_eq!(riben.count, 2);
        assert_eq!(riben.writing_method, "??????");

        let chi = words.get("???").unwrap();
        assert_eq!(chi.count, 2);
        assert_eq!(chi.writing_method, "???");

        let gaoxing = words.get("??????").unwrap();
        assert_eq!(gaoxing.count, 1);
        assert_eq!(gaoxing.writing_method, "??????");
    }

    #[test]
    fn expect_to_generate_list_with_punctuation() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("?????????????????? ? ???????????????. ").unwrap();
        
        let weather = words.get("??????").unwrap();
        assert_eq!(weather.count, 2);

        let ma = words.get("???").unwrap();
        assert_eq!(ma.count, 1);
    }

    #[test]
    fn expect_to_generate_csv() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("??????????????????").unwrap();
        let res = words.to_csv();

        assert!(res.is_ok());
    }

    #[test]
    fn expect_to_return_none_when_no_chinese_word() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("hello");
        assert!(words.is_none());
    }
    
    #[test]
    fn expect_to_load_simplified_chinese() {
        let mut dictionary = super::Dictionary::new(Some(Version::Simplified)).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("???????????????????");
        assert!(words.is_some());

        let words = words.unwrap();
        let like = words.get("??????").unwrap();
        assert_eq!(like.count, 1);
        assert_eq!(like.writing_method, "??????");

        let question_mark = words.get("???").unwrap();
        assert_eq!(question_mark.count, 1);
        assert_eq!(question_mark.writing_method, "???");
    }
}
