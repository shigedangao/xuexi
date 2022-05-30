use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use super::{Char, Clean};

// Constant
const NB_SIGN_CHARACTER_CEDICT: char = '#';
const PERCENT_CHARACTER_CEDICT: char = '%';

#[derive(Debug, Default, Clone)]
pub struct Cedict {
    traditional_character: String,
    simplify_character: String,
    pinyin: String,
    english: String
}

#[derive(Debug, Clone, Default)]
pub struct Dictionnary {
    dic: HashMap<String, Cedict>,
}

// Custom type to handle the sentences list
type SentencesDictionnary = HashMap<String, (Cedict, i64)>;

impl Cedict {
    /// Get a vector of english translation from the string representation
    fn get_english_translations(&self) -> Vec<String> {
        self.english
            .split("/")
            .into_iter()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                Some(s.trim().to_string())
            })
            .collect::<Vec<String>>()
    }
}

impl Dictionnary {
    /// Create a new Dictionnary from the cedict_ts.u8
    pub fn new() -> Dictionnary {
        let mut dic = HashMap::new();
        let cedict: &[u8] = include_bytes!("../cedict_ts.u8");
        let buffer = BufReader::new(cedict);

        for line in buffer.lines() {
            if line.is_err() {
                continue;
            }

            let line = line.unwrap();
            if line.starts_with(NB_SIGN_CHARACTER_CEDICT) || line.starts_with(PERCENT_CHARACTER_CEDICT) {
                continue;
            }

            let mut item = Cedict::default();
            let mut reminder = "";

            if let Some((tw_character, rest)) = line.split_once(' ') {
                item.traditional_character = tw_character.to_owned();
                reminder = rest;
            }

            if let Some((sf_character, rest)) = reminder.split_once(' ') {
                item.simplify_character = sf_character.to_owned();
                reminder = rest;
            }

            if let Some((pinyin, rest)) = reminder.split_once(']') {
                item.pinyin = pinyin.to_owned().replace('[', "");
                item.english = rest.trim().to_string();
            }

            dic.insert(item.traditional_character.to_owned(), item);
        }

        Dictionnary { dic }
    }

    /// Get a list of definitions based on the loaded cedict dictionnary from a given sentence
    /// 
    /// # Arguments
    /// 
    /// * `sentence` - A string slice which represent a sentence
    pub fn get_definitions_for_sentence(&self, sentence: &str) -> SentencesDictionnary {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        let mut done = false;
        // flag used to count the number of character unmatched
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionnary = HashMap::new();

        // split the sentence into a vector of characters
        let cleaned_sentence = self.remove_punctuation_from_sentence(sentence);
        let characters: Vec<char> = cleaned_sentence.chars().collect();

        let mut def: Cedict = Cedict::default();
        while !done {
            // create a word based on the start cursor and the end cursor
            let word: String = characters[start_cursor..end_cursor].to_vec().iter().collect();
            match self.dic.get(&word) {
                Some(definition) => {
                    def = definition.clone();
                    if end_cursor == characters.len() {
                        insert_map_word(&mut dictionnary, def.clone());
                        done = true;
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
                        // Only used in the case if the end_cursor is at the end of the characters vector
                        // in that case it means that we weren't able to match anything
                        // hence, we're canceling the loop to avoid creating an index overflow error
                        if end_cursor == characters.len() {
                            done = true;
                        }
                    } else {
                        // Push the latest founded item in the dictionnary
                        insert_map_word(&mut dictionnary, def.clone());
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

        dictionnary
    }
}

// Note that we're not implementing the trait for the Dictionnary
// this is to avoid having conflict when working with the struct when
// multiple worker can use it
impl Char<(Cedict, i64)> for SentencesDictionnary {
    fn get_ordered_characters(&self) -> Vec<(Cedict, i64)> {
        let mut vec = self.values()
            .cloned()
            .collect::<Vec<(Cedict, i64)>>();

        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        vec
    }
}

impl Clean for Dictionnary {}

/// Insert a definition in a map
/// 
/// # Arguments
/// 
/// * `map` - A mutable reference to a HashMap
/// * `item` - A Cedict item which we'll be insert
fn insert_map_word(map: &mut HashMap<String, (Cedict, i64)>, item: Cedict) {
    if let Some((_, v)) = map.get_mut(&item.traditional_character) {
        *v += 1;
    } else {
        map.insert(item.traditional_character.to_string(), (item, 1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_dictionnary() {
        let dictionnary = Dictionnary::new();
        let res = dictionnary.dic.get("上");
        
        assert!(res.is_some());
        let shang = res.unwrap();
        assert_eq!(shang.traditional_character, "上");
    }

    #[test]
    fn expect_to_get_dictionnary_for_sentence() {
        let dictionnary = Dictionnary::new();
        let res = dictionnary.get_definitions_for_sentence("去年今夜");

        let qu = res.get("去年");
        assert!(qu.is_some());

        let (qu_def, qu_count) = qu.unwrap();
        assert_eq!(qu_def.traditional_character, "去年");
        assert_eq!(*qu_count, 1);

        let english_translation = qu_def.get_english_translations();
        let english_translation = english_translation.get(0).unwrap();
        assert_eq!(english_translation, "last year");
    }

    #[test]
    fn expect_to_get_dictionnary_for_complicated_sentence() {
        let dictionnary = Dictionnary::new();
        let res = dictionnary.get_definitions_for_sentence("去年今夜中國人同醉月明花樹下lol台灣去年");
        
        let qu = res.get("去年");
        assert!(qu.is_some());

        let (qu_def, qu_count) = qu.unwrap();
        assert_eq!(qu_def.traditional_character, "去年");
        assert_eq!(*qu_count, 2);

        let taiwan = res.get("台灣");
        assert!(taiwan.is_some());

        let (taiwan_def, taiwan_count) = taiwan.unwrap();
        assert_eq!(taiwan_def.traditional_character, "台灣");
        assert_eq!(*taiwan_count, 1);
    }

    #[test]
    fn expect_to_get_ordered_list_of_definition() {
        let dictionnary = Dictionnary::new();
        let content = "今天天氣好熱.我今天要吃冰糕";
        let definitions = dictionnary.get_definitions_for_sentence(content);
        let ordered_definition = definitions.get_ordered_characters();

        let (today_def, count) = ordered_definition.first().unwrap();
        assert_eq!(today_def.traditional_character, "今天");
        assert_eq!(*count, 2);
    }
}