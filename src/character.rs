use std::collections::BTreeMap;
use futures::future::join_all;
use super::Char;

// constant
const PUNC_FULL_STOP: &str = ".";
const PUNC_QS_MARK: &str = "?";
const PUNC_EXC_MARK: &str = "!";
const PUNC_EMPTY: &str = "\n";

pub struct Characters {
    content: String,
    unordered_list: BTreeMap<char, i128>
}

impl Characters {
    /// Create a new Characters struct with the content that needs to be parsed
    /// 
    /// # Arguments
    /// 
    /// * `content` - A slice of content (text, sentences)
    pub fn new(content: &str) -> Self {
        Characters { content: content.to_owned(), unordered_list: BTreeMap::new() }
    }

    /// Generate a list of character which contain it's number of recurrency
    pub async fn generate_characters_list(&mut self) {
        // split a content by a space to avoid working with a very long content
        let splitted: Vec<&str> = self.content.split(" ").collect();
        // create a list of async method which we'll join
        let workers: Vec<_> = splitted
            .into_iter()
            .map(|s| self.remove_punctuation_from_sentence(s))
            .map(|s| self.count_char_for_sentence(s))
            .collect();
    
        let outputs = join_all(workers).await;
        let mut list = BTreeMap::new();
        for map in outputs.into_iter() {
            for (k, v) in map.into_iter() {
                if let Some(lv) = list.get_mut(&k) {
                    *lv = *lv + v;
                } else {
                    list.insert(k, v);
                }
            }
        }

        self.unordered_list = list;    
    }

    /// Count character for a sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    async fn count_char_for_sentence(&self, sentence: String) -> BTreeMap<char, i128> {
        let mut m: BTreeMap<char, i128> = BTreeMap::new();
        let chars = sentence.chars();
    
        for char in chars.into_iter() {
            if let Some(count) = m.get_mut(&char) {
                *count = *count + 1;
            } else {
                m.insert(char, 1);
            }
        }
    
        m
    }

    /// Remove punctuation from a sentence to avoid being count
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A slice of string which represent a sentence
    fn remove_punctuation_from_sentence(&self, sentence: &str) -> String {
        sentence
            .replace(PUNC_FULL_STOP, "")
            .replace(PUNC_QS_MARK, "")
            .replace(PUNC_EXC_MARK, "")
            .replace(PUNC_EMPTY, "")
    }
}

impl Char<(char, i128)> for Characters {
    fn get_ordered_characters(self) -> Vec<(char, i128)> {
        let mut vec = Vec::from_iter(self.unordered_list.into_iter());
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        vec
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn expect_to_return_chinese_char_list() {
        let content = "我喜歡你的狗. 你喜不喜歡我的狗?";
        let mut handler = Characters::new(content);
        handler.generate_characters_list().await;

        assert_eq!(*handler.unordered_list.get(&'喜').unwrap(), 3);
        assert_eq!(*handler.unordered_list.get(&'的').unwrap(), 2);
        assert_eq!(*handler.unordered_list.get(&'不').unwrap(), 1);
    }

    #[tokio::test]
    async fn expect_to_return_chinese_char_for_list_sentences() {
        let content = r#"
        上個星期天我跟朋友一起去了學校的年位居活動. 我們一起介紹一首詞.
        詞的作者是呂本中. 呂本中是宋代人. 詞的題目是去年今夜.
        "#;

        let mut handler = Characters::new(content);
        handler.generate_characters_list().await;
        
        assert_eq!(*handler.unordered_list.get(&'是').unwrap(), 3);
        assert_eq!(*handler.unordered_list.get(&'我').unwrap(), 2);
        assert_eq!(*handler.unordered_list.get(&'中').unwrap(), 2);
    }
}