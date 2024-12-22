use crate::dictionary::{Chinese, Dictionary, Initializer, Lang};
use crate::error::DictionaryError;
use crate::punctuation;
use crate::util;
use crate::word::{Word, WordParser};
use dodo_zh::cedict::Item;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::path::PathBuf;

impl Initializer<Chinese> for Dictionary<Chinese> {
    fn initialize(params: Lang) -> Result<Dictionary<Chinese>, DictionaryError> {
        let p = punctuation::Puncutation::new()?;

        Ok(Dictionary::<Chinese> {
            _lang: PhantomData::<Chinese>,
            dict: HashMap::default(),
            punctuation: p.chinese,
            params,
        })
    }

    /// Create a new Dictionary from the cedict_ts.u8
    fn load(&mut self, path: PathBuf) -> Result<(), DictionaryError> {
        let Lang::Chinese(variant) = &self.params else {
            return Err(DictionaryError::ChineseDictionary(
                "Unable to found the definition of chinese version to use".to_string(),
            ));
        };

        let dictionary = dodo_zh::load_cedict_dictionary(path, variant.clone())
            .map_err(|err| DictionaryError::ChineseDictionary(err.to_string()))?;

        let dict: HashMap<String, Word> = dictionary
            .items
            .into_iter()
            .map(|(k, item)| (k, Word::from(item)))
            .collect();

        self.dict = dict;

        Ok(())
    }
}

impl WordParser for Dictionary<Chinese> {
    fn parse_sentence_into_words<S: AsRef<str>>(&self, sentence: S) -> BTreeMap<String, Word> {
        // Collections
        let mut words = BTreeMap::new();
        // Cursors
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        // split the sentence into a vector of characters
        let sentence = util::clean_sentence(sentence.as_ref(), &self.punctuation);

        let sentence_chars = sentence.chars().collect::<Vec<_>>();
        // temp definition
        let (mut step_found_word, mut step_found_word_str) = (Word::default(), String::default());
        while let Some(chars) = sentence_chars.get(start_cursor..end_cursor) {
            let word = chars.into_iter().collect::<String>();
            // create a word based on the start cursor and the end cursor
            match self.dict.get(&word) {
                Some(res) => {
                    step_found_word = res.clone();
                    step_found_word_str = word.clone();
                    // If the end_cursor is equal to the length of the sentence, then push the latest founded value.
                    if end_cursor == sentence_chars.len() {
                        self.insert_word(
                            &mut words,
                            step_found_word_str.clone(),
                            step_found_word.clone(),
                        );
                    }

                    end_cursor += 1;
                    // reset the unmatched flag
                    unmatched = 0;
                }
                None => {
                    // this unmatched is used in case if we're encountering a character which can't be matched
                    // multiple time. If we're unable to find the same character / word for multiple time
                    // then we're increasing the start_cursor & end_cursor in a hope that we'll match something later on...
                    if unmatched > 1 {
                        start_cursor += 1;
                        end_cursor += 1;
                    } else {
                        if !step_found_word_str.is_empty() {
                            // Push the latest founded item in the Dictionary
                            self.insert_word(
                                &mut words,
                                step_found_word_str.clone(),
                                step_found_word.clone(),
                            );
                            // if nothing can be found on the Dictionary then we move the start_cursor to end_cursor - 1
                            // this allow us to check the last -1 character again
                            // for example
                            // 去年今夜 -> at some point the method will check this characters 去年今
                            // the start_cursor will move to 2
                            // the end_cursor will be equal to 3
                            // from these cursors, this will match the character "今 " in the sentence
                            // then it'll continue to move the end_cursor to 4 -> 今夜
                            // which was matched at the latest (end_cursor)
                        }

                        start_cursor = end_cursor - 1;
                    }

                    unmatched += 1;
                }
            }
        }

        words
    }
}

impl From<Item> for Word {
    fn from(value: Item) -> Self {
        Word {
            written: vec![value.traditional_character, value.simplified_character],
            pronunciations: value.pinyin_tone_number,
            translations: value.translations,
            count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dodo_zh::variant::KeyVariant::{Simplified, Traditional};
    use std::sync::LazyLock;

    static DICTIONARY: LazyLock<Dictionary<Chinese>> = LazyLock::new(|| {
        let mut dictionary =
            super::Dictionary::<Chinese>::initialize(Lang::Chinese(Traditional)).unwrap();
        dictionary.load(PathBuf::from("./cedict_ts.u8")).unwrap();

        dictionary
    });

    #[test]
    fn expect_to_load_dictionary() {
        let mut dictionary =
            super::Dictionary::<Chinese>::initialize(Lang::Chinese(Traditional)).unwrap();
        dictionary.load(PathBuf::from("./cedict_ts.u8")).unwrap();

        assert!(!dictionary.dict.is_empty());
    }

    #[test]
    fn expect_to_get_same_char_and_different_pronounciation() {
        let res = DICTIONARY.parse_sentence_into_words("得");
        let dei = res.get("得").unwrap();

        assert_eq!(dei.pronunciations.get(0).unwrap(), "dei3");
        assert_eq!(dei.pronunciations.last().unwrap(), "dei3");
    }

    #[test]
    fn expect_to_get_characters() {
        let words = DICTIONARY.parse_sentence_into_words("你好你好");
        let nihao = words.get("你好");

        assert!(nihao.is_some());

        let nihao = nihao.unwrap();
        assert_eq!(nihao.count, 2);
        assert_eq!(nihao.written.first().unwrap(), "你好");
    }

    #[test]
    fn expect_to_get_ordered_characters() {
        let words = DICTIONARY.parse_sentence_into_words("去年我去過日本看我好朋友日本很好看");
        let riben = words.get("日本").unwrap();

        assert_eq!(riben.count, 2);
        assert_eq!(riben.written.first().unwrap(), "日本");
    }

    #[test]
    fn expect_to_generate_list_with_for_multiple_sentences() {
        let words = DICTIONARY
            .parse_sentence_into_words("今天我日本朋友吃拉麵. 日本拉麵看起來好吃! 吃拉麵讓我高興");

        let riben = words.get("日本").unwrap();
        assert_eq!(riben.count, 2);
        assert_eq!(riben.written.first().unwrap(), "日本");

        let chi = words.get("吃").unwrap();
        assert_eq!(chi.count, 2);
        assert_eq!(chi.written.first().unwrap(), "吃");

        let gaoxing = words.get("高興").unwrap();
        assert_eq!(gaoxing.count, 1);
        assert_eq!(gaoxing.written.first().unwrap(), "高興");
    }

    #[test]
    fn expect_to_generate_list_with_punctuation() {
        let words = DICTIONARY.parse_sentence_into_words("今天天氣好嗎 ? 天氣非常好. ");

        let weather = words.get("天氣").unwrap();
        assert_eq!(weather.count, 2);

        let ma = words.get("嗎").unwrap();
        assert_eq!(ma.count, 1);
    }

    #[test]
    fn expect_to_return_none_when_no_chinese_word() {
        let words = DICTIONARY.parse_sentence_into_words("hello");
        assert!(words.is_empty());
    }

    #[test]
    fn expect_to_load_simplified_chinese() {
        let mut dictionary =
            super::Dictionary::<Chinese>::initialize(Lang::Chinese(Simplified)).unwrap();
        dictionary.load(PathBuf::from("./cedict_ts.u8")).unwrap();

        let words = dictionary.parse_sentence_into_words("你喜欢开车吗?");

        let like = words.get("喜欢").unwrap();
        assert_eq!(like.count, 1);
        assert_eq!(like.written.first().unwrap(), "喜歡");

        let question_mark = words.get("吗").unwrap();
        assert_eq!(question_mark.count, 1);
        assert_eq!(question_mark.written.first().unwrap(), "嗎");
    }
}
