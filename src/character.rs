use std::collections::HashMap;
use serde::Serialize;
use crate::error::LibError;
use crate::ordering::{Ops};
use crate::clean::Clean;
use crate::punctuation;
use crate::export;

pub struct Characters<'a> {
    content: &'a str,
    punctuation: Vec<String>
}

#[derive(Debug, Serialize)]
struct CharacterCount {
    char: char,
    count: i64
}

// Custom type to handle Map for character
pub type CharactersList = HashMap<char, i64>;

impl<'a> Characters<'a> {
    /// Create a new Characters struct with the content that needs to be parsed
    /// 
    /// # Arguments
    /// 
    /// * `content` - A slice of content (text, sentences)
    pub fn new(content: &'a str) -> Result<Self, LibError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Characters {
            content,
            punctuation: p.western
        })
    }

    /// Generate a list of character which contain it's number of recurrency
    pub fn generate_characters_list(&self) -> CharactersList {
        let cleaned_sentence = self.remove_punctuation_from_sentence(self.content, &self.punctuation);
        let output = self.count_char_for_sentence(&cleaned_sentence);
    
        output
    }

    /// Count character for a sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    fn count_char_for_sentence(&self, sentence: &str) -> HashMap<char, i64> {
        let mut m: HashMap<char, i64> = HashMap::new();
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

impl Clean for Characters<'_> {}

impl Ops<(char, i64)> for CharactersList {
    fn get_ordered_characters(&self) -> Vec<(char, i64)> {
        let mut vec: Vec<_> = Vec::from_iter(self.clone().into_iter());
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        vec
    }
}

impl export::Export for CharactersList {
    fn export_to_csv(&self) -> Result<String, crate::error::LibError> {
        let ordered = self.get_ordered_characters();
        let items: Vec<CharacterCount> = ordered.into_iter()
            .map(|(char, count)| CharacterCount {
                char,
                count
            })
            .collect();

        export::export_to_csv(items)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::Export;

    #[test]
    fn expect_to_return_chinese_char_list() {
        let content = "我喜歡你的狗. 你喜不喜歡我的狗?";
        let handler = Characters::new(content).unwrap();
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

        let handler = Characters::new(content).unwrap();
        let res = handler.generate_characters_list();
        
        assert_eq!(*res.get(&'是').unwrap(), 3);
        assert_eq!(*res.get(&'我').unwrap(), 2);
        assert_eq!(*res.get(&'中').unwrap(), 2);
    }

    #[test]
    fn expect_to_return_ordered_character_by_presence() {
        let content = "我跟你一起吃飯. 你要喝什麼";
        let handler = Characters::new(content).unwrap();
        let res = handler.generate_characters_list();

        let ordered_list = res.get_ordered_characters();
        let most_present = ordered_list.first();

        assert!(most_present.is_some());
        let (character, count) = most_present.unwrap();
        assert_eq!(*character, '你');
        assert_eq!(*count, 2);
    }

    #[test]
    fn expect_to_export_to_csv() {
        let content = "今天天氣非常熱";
        let handler = Characters::new(content).unwrap();
        let res = handler.generate_characters_list();

        let csv = res.export_to_csv();
        assert!(csv.is_ok());
    }
}