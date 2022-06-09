use std::collections::HashMap;
use std::io::Error;
use std::io::{BufReader, BufRead};
use crate::definition::Definition;
use crate::common::{Clean, Ordered};

pub mod def;

// Constant
const NB_SIGN_CHARACTER_CEDICT: char = '#';
const PERCENT_CHARACTER_CEDICT: char = '%';
const EMPTY_SPACE_CHARACTER: char = ' ';
const LEFT_BRACKET_CHARACTER: char = '[';
const RIGHT_BRACKET_CHARACTER: char = ']';

#[derive(Debug, Clone, Default)]
pub struct Dictionnary {
    dic: HashMap<String, Definition>,
}

// Custom type to handle the sentences list
pub type SentencesDictionnary = HashMap<String, Definition>;

impl Dictionnary {
    /// Create a new empty dictionnary
    pub fn new() -> Dictionnary {
        Dictionnary::default()
    }

    /// Create a new Dictionnary from the cedict_ts.u8
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
                        item.prounciation = pinyin.to_owned().replace(LEFT_BRACKET_CHARACTER, "");
                        item.english = rest.trim().to_string();
                    }
                },
                None => continue
            }

            dic.insert(item.writing_method.to_owned(), item);
        }

        self.dic = dic;
    }

    /// Get a list of detected words based on the loaded cedict dictionnary from a given sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A string slice which represent a sentence
    pub fn get_list_detected_words(&self, sentence: &str) -> Option<SentencesDictionnary> {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionnary = HashMap::new();
        // split the sentence into a vector of characters
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence);
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
                        self.insert_map_word(&mut dictionnary, &step_def);
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
                        // Push the latest founded item in the dictionnary
                        self.insert_map_word(&mut dictionnary, &step_def);
                        // if nothing can be found on the dictionnary then we move the start_cursor to end_cursor - 1
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

        Some(dictionnary)
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

    /// Insert a definition in a map
    /// 
    /// # Arguments
    /// 
    /// * `map` - A mutable reference to a HashMap
    /// * `item` - A Definition item which we'll be insert
    fn insert_map_word(&self, map: &mut HashMap<String, Definition>, item: &Option<Definition>) {
        if item.is_none() {
            return;
        }

        let item = item.to_owned().unwrap();
        if let Some(def) = map.get_mut(&item.writing_method) {
            def.count += 1;
        } else {
            let mut item_clone = item.clone();
            item_clone.count = 1;
            map.insert(item.writing_method.clone(), item_clone);
        }
    }

}

impl Clean for Dictionnary {}

impl Ordered<(String, Definition)> for SentencesDictionnary {
    fn get_ordered_characters(&self) -> Vec<(String, Definition)> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter());
        vec.sort_by(|(_, a), (_, b)| b.count.cmp(&a.count));

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_load_dictionnary() {
        let mut dictionnary = super::Dictionnary::new();
        dictionnary.load();
        let words = dictionnary.get_list_detected_words("氣不錯你喜歡喝酒嗎你跟我一起喝");

        println!("{:?}", words);
        assert!(!words.unwrap().is_empty());
    }

    #[test]
    fn expect_to_not_load_dictionnary() {
        let mut dictionnary = super::Dictionnary::new();
        dictionnary.load();
        let words = dictionnary.get_list_detected_words("你好你好");

        println!("{:?}", words);
    }

    #[test]
    fn expect_to_load_dictionnary_en() {
        let mut dictionnary = super::Dictionnary::new();
        dictionnary.load();
        let words = dictionnary.get_list_detected_words("You");

        println!("{:?}", words);
    }
}