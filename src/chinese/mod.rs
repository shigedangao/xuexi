use std::collections::BTreeMap;
use serde::Deserialize;
use crate::definition::{Definition, DefinitionList, InsertOrMerge};
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
    pinyin_accent: String,
    translations: String
}

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    dic: DefinitionList,
    punctuation: Vec<String>,
    version: Version
}


impl Dictionary {
    /// Create a new empty Dictionary
    /// 
    /// # Arguments
    /// 
    /// * `version` - Option<Version> (Simplified or Traditional chinese)
    pub fn new(version: Option<Version>) -> Result<Dictionary, LibError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Dictionary {
            dic: BTreeMap::default(),
            punctuation: p.chinese,
            version: version.unwrap_or_default()
        })
    }

    /// Create a new Dictionary from the cedict_ts.u8
    pub fn load(&mut self) -> Result<(), LibError> {
        let mut dic = BTreeMap::new();
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
                pronunciations: vec![record.pinyin_accent],
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

    fn decorate_insert_map_word(&self, map: &mut DefinitionList, item: &Option<Definition>) {
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
    fn get_list_detected_words(&self, sentence: impl AsRef<str>) -> Option<DefinitionList> {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionary = BTreeMap::new();
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
                        // 去年今夜 -> at some point the method will check this characters 去年今
                        // the start_cursor will move to 2
                        // the end_cursor will be equal to 3
                        // from these cursors, this will match the character "今 " in the sentence
                        // then it'll continue to move the end_cursor to 4 -> 今夜 
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

        let res = dictionary.get_list_detected_words("得").unwrap();
        let dei = res.get("得").unwrap();

        assert_eq!(dei.pronunciations.get(0).unwrap(), "dé");
        assert_eq!(dei.pronunciations.last().unwrap(), "děi");
    }

    #[test]
    fn expect_to_get_characters() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("你好你好").unwrap();
        let nihao = words.get("你好");

        assert!(nihao.is_some());

        let nihao = nihao.unwrap();
        assert_eq!(nihao.count, 2);
        assert_eq!(nihao.writing_method, "你好");
    }

    #[test]
    fn expect_to_get_ordered_characters() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("去年我去過日本看我好朋友日本很好看").unwrap();
        let riben = words.get("日本").unwrap();
        
        assert_eq!(riben.count, 2);
        assert_eq!(riben.writing_method, "日本");

        let vec = words.get_ordered_characters();
        let (f, item) = vec.first().unwrap();

        assert_eq!(f, "日本");
        assert_eq!(item.count, 2);
        assert_eq!(item.writing_method, "日本");
    }

    #[test]
    fn expect_to_generate_list_with_for_multiple_sentences() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("今天我日本朋友吃拉麵. 日本拉麵看起來好吃! 吃拉麵讓我高興").unwrap();
        
        let riben = words.get("日本").unwrap();
        assert_eq!(riben.count, 2);
        assert_eq!(riben.writing_method, "日本");

        let chi = words.get("吃").unwrap();
        assert_eq!(chi.count, 2);
        assert_eq!(chi.writing_method, "吃");

        let gaoxing = words.get("高興").unwrap();
        assert_eq!(gaoxing.count, 1);
        assert_eq!(gaoxing.writing_method, "高興");
    }

    #[test]
    fn expect_to_generate_list_with_punctuation() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("今天天氣好嗎 ? 天氣非常好. ").unwrap();
        
        let weather = words.get("天氣").unwrap();
        assert_eq!(weather.count, 2);

        let ma = words.get("嗎").unwrap();
        assert_eq!(ma.count, 1);
    }

    #[test]
    fn expect_to_generate_csv() {
        let mut dictionary = super::Dictionary::new(None).unwrap();
        dictionary.load().unwrap();

        let words = dictionary.get_list_detected_words("我昨天感冒了").unwrap();
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

        let words = dictionary.get_list_detected_words("你喜欢开车吗?");
        assert!(words.is_some());

        let words = words.unwrap();
        let like = words.get("喜欢").unwrap();
        assert_eq!(like.count, 1);
        assert_eq!(like.writing_method, "喜歡");

        let question_mark = words.get("吗").unwrap();
        assert_eq!(question_mark.count, 1);
        assert_eq!(question_mark.writing_method, "嗎");
    }
}
