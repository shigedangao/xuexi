use std::collections::BTreeMap;
use csv::Writer;
use serde::Serialize;
use crate::error::LibError;
use crate::common::{Ops, Clean};

// constant
const EMPTY_SPACE_CHARACTER: char = ' ';

pub struct Characters {
    content: String,
}

#[derive(Debug, Serialize)]
struct CharacterCount {
    char: char,
    count: i64
}

// Custom type to handle Map for character
pub type CharactersList = BTreeMap<char, i64>;

impl Characters {
    /// Create a new Characters struct with the content that needs to be parsed
    /// 
    /// # Arguments
    /// 
    /// * `content` - A slice of content (text, sentences)
    pub fn new(content: &str) -> Self {
        Characters { content: content.to_owned() }
    }

    /// Generate a list of character which contain it's number of recurrency
    pub fn generate_characters_list(&self) -> CharactersList {
        // split a content by a space to avoid working with a very long content
        let splitted = self.content.split(EMPTY_SPACE_CHARACTER);
        // create a list of async method which we'll join
        let outputs: Vec<_> = splitted
            .into_iter()
            .map(|s| self.remove_punctuation_from_sentence(s))
            .map(|s| self.count_char_for_sentence(s))
            .collect();
    
        let mut list = BTreeMap::new();
        for map in outputs {
            for (k, v) in map.into_iter() {
                if let Some(lv) = list.get_mut(&k) {
                    *lv += v;
                } else {
                    list.insert(k, v);
                }
            }
        }

        list
    }

    /// Count character for a sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    fn count_char_for_sentence(&self, sentence: String) -> BTreeMap<char, i64> {
        let mut m: BTreeMap<char, i64> = BTreeMap::new();
        let chars = sentence.chars();
    
        for char in chars {
            if let Some(count) = m.get_mut(&char) {
                *count += 1;
            } else {
                m.insert(char, 1);
            }
        }
    
        m
    }
}

impl Clean for Characters {}

impl Ops<(char, i64)> for CharactersList {
    fn get_ordered_characters(&self) -> Vec<(char, i64)> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter());
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        vec
    }

    fn export_to_csv(&self) -> Result<String, crate::error::LibError> {
        let ordered = self.get_ordered_characters();
        let items: Vec<CharacterCount> = ordered.into_iter()
            .map(|(char, count)| CharacterCount {
                char,
                count
            })
            .collect();

        let mut wrt = Writer::from_writer(vec![]);
        wrt.serialize(items)?;

        let inner = wrt.into_inner()
            .map_err(|err| LibError::Serialize(err.to_string()))?;

        let res = String::from_utf8(inner)?;

        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_return_chinese_char_list() {
        let content = "我喜歡你的狗. 你喜不喜歡我的狗?";
        let handler = Characters::new(content);
        let res = handler.generate_characters_list();

        assert_eq!(*res.get(&'喜').unwrap(), 3);
        assert_eq!(*res.get(&'的').unwrap(), 2);
        assert_eq!(*res.get(&'不').unwrap(), 1);
    }

    #[test]
    fn expect_to_return_chinese_char_for_list_sentences() {
        let content = r#"
        上個星期天我跟朋友一起去了學校的年位居活動. 我們一起介紹一首詞.
        詞的作者是呂本中. 呂本中是宋代人. 詞的題目是去年今夜.
        "#;

        let handler = Characters::new(content);
        let res = handler.generate_characters_list();
        
        assert_eq!(*res.get(&'是').unwrap(), 3);
        assert_eq!(*res.get(&'我').unwrap(), 2);
        assert_eq!(*res.get(&'中').unwrap(), 2);
    }

    #[test]
    fn expect_to_return_ordered_character_by_presence() {
        let content = "我跟你一起吃飯你要吃什麼";
        let handler = Characters::new(content);
        let res = handler.generate_characters_list();

        let ordered_list = res.get_ordered_characters();
        let most_present = ordered_list.first();

        assert!(most_present.is_some());
        let (character, count) = most_present.unwrap();
        assert_eq!(*character, '你');
        assert_eq!(*count, 2);
    }
}