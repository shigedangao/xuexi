use std::collections::HashMap;
use std::io::Error;
use std::io::{BufReader, BufRead};
use crate::definition::Definition;
use crate::word::DetectWord;
use crate::clean::Clean;
use crate::error::LibError;
use crate::punctuation;

pub mod def;

// Constant
const NB_SIGN_CHARACTER_CEDICT: char = '#';
const PERCENT_CHARACTER_CEDICT: char = '%';
const EMPTY_SPACE_CHARACTER: char = ' ';
const LEFT_BRACKET_CHARACTER: char = '[';
const RIGHT_BRACKET_CHARACTER: char = ']';

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    dic: HashMap<String, Definition>,
    punctuation: Vec<String>
}

impl Dictionary {
    /// Create a new empty Dictionary
    pub fn new() -> Result<Dictionary, LibError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Dictionary {
            dic: HashMap::default(),
            punctuation: p.chinese
        })
    }

    /// Create a new Dictionary from the cedict_ts.u8
    pub fn load(&mut self) {
        let mut dic = HashMap::new();
        let definition: &[u8] = include_bytes!("../../cedict_ts.u8");
        let buffer = BufReader::new(definition);
        
        for line in buffer.lines() {
            let mut item = Definition::default();
            match self.verify_line(line) {
                Some(content) => {
                    let mut reminder = "";
                    if let Some((tw_character, rest)) = content.split_once(EMPTY_SPACE_CHARACTER) {
                        item.writing_method = tw_character.to_owned();
                        reminder = rest;
                    }
        
                    if let Some((sf_character, rest)) = reminder.split_once(EMPTY_SPACE_CHARACTER) {
                        item.writing_method_two = Some(sf_character.to_owned());
                        reminder = rest;
                    }
        
                    if let Some((pinyin, rest)) = reminder.split_once(RIGHT_BRACKET_CHARACTER) {
                        item.pronunciation = pinyin.to_owned().replace(LEFT_BRACKET_CHARACTER, "");
                        item.english = rest.trim().to_string();
                    }
                },
                None => continue
            }

            dic.insert(item.writing_method.to_owned(), item);
        }

        self.dic = dic;
    }

    /// Check that the line does not contains any character that we want to avoid
    /// 
    /// # Arguments
    /// * `line` - Result<String, Error>
    fn verify_line(&self, line: Result<String, Error>) -> Option<String> {
        match line {
            Ok(content) => {
                if content.starts_with(NB_SIGN_CHARACTER_CEDICT) || content.starts_with(PERCENT_CHARACTER_CEDICT) {
                    return None;
                }

                Some(content)
            }
            Err(_) => None,
        }
    }
}

impl Clean for Dictionary {}

impl DetectWord for Dictionary {
    fn get_dictionary(&self) -> &HashMap<String, Definition> {
        &self.dic
    }

    fn get_list_detected_words(&self, sentence: &str) -> Option<HashMap<String, Definition>> {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionary = HashMap::new();
        // split the sentence into a vector of characters
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence, &self.punctuation);
        let characters: Vec<char> = cleaned_sentence.chars().collect();

        // temp definition
        let mut step_def: Option<Definition> = None;
        while let Some(char) = characters.get(start_cursor..end_cursor) {
            // create a word based on the start cursor and the end cursor
            let word: String = char.to_vec().iter().collect();
            match self.get_dictionary().get(&word) {
                Some(res) => {
                    step_def = Some(res.clone());
                    if end_cursor == characters.len() {
                        self.insert_map_word(&mut dictionary, &step_def);
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
                        self.insert_map_word(&mut dictionary, &step_def);
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
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

        assert!(!dictionary.dic.is_empty());
    }

    #[test]
    fn expect_to_get_characters() {
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

        let words = dictionary.get_list_detected_words("你好你好").unwrap();
        let nihao = words.get("你好");

        assert!(nihao.is_some());

        let nihao = nihao.unwrap();
        assert_eq!(nihao.count, 2);
        assert_eq!(nihao.writing_method, "你好");
    }

    #[test]
    fn expect_to_get_ordered_characters() {
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

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
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

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
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

        let words = dictionary.get_list_detected_words("今天天氣好嗎 ? 天氣非常好. ").unwrap();
        
        let weather = words.get("天氣").unwrap();
        assert_eq!(weather.count, 2);

        let ma = words.get("嗎").unwrap();
        assert_eq!(ma.count, 1);

        println!("{:?}", words);
    }

    #[test]
    fn expect_to_generate_csv() {
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

        let words = dictionary.get_list_detected_words("我昨天感冒了").unwrap();
        let res = words.export_to_csv();

        assert!(res.is_ok());
    }

    #[test]
    fn expect_to_return_none_when_no_chinese_word() {
        let mut dictionary = super::Dictionary::new().unwrap();
        dictionary.load();

        let words = dictionary.get_list_detected_words("hello");
        assert!(words.is_none());
    }
}