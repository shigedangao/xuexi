use std::collections::HashMap;
use std::io::{BufReader, BufRead};

#[derive(Debug, Default, Clone)]
struct Cedict {
    traditional_character: String,
    simplify_character: String,
    pinyin: String,
    english: String
}

#[derive(Debug)]
pub struct Dictionnary {
    dic: HashMap<String, Cedict>
}

impl Dictionnary {
    pub fn new() -> Dictionnary {
        let mut dic = HashMap::new();
        let cedict: &[u8] = include_bytes!("../cedict_ts.u8");
        let buffer = BufReader::new(cedict);

        for line in buffer.lines() {
            if line.is_err() {
                continue;
            }

            let line = line.unwrap();
            if line.starts_with("#") || line.starts_with("%") {
                continue;
            }

            let mut item = Cedict::default();
            let mut reminder = "";

            if let Some((tw_character, rest)) = line.split_once(" ") {
                item.traditional_character = tw_character.to_owned();
                reminder = rest;
            }

            if let Some((sf_character, rest)) = reminder.split_once(" ") {
                item.simplify_character = sf_character.to_owned();
                reminder = rest;
            }

            if let Some((pinyin, rest)) = reminder.split_once("]") {
                item.pinyin = pinyin.to_owned();
                item.english = rest.to_owned();
            }

            dic.insert(item.traditional_character.to_owned(), item);
        }

        Dictionnary { dic }
    }

    fn get_dictionnary_for_sentence(&self, sentence: &str) -> Vec<Cedict> {
        let mut start_cursor = 0;
        let mut end_cursor = 1;
        let mut done = false;
        // flag used to count the number of character unmatched
        // this is to avoid a case where we can do an infinite loop on a single character
        let mut unmatched = 0;
        let mut dictionnary = Vec::new();

        let characters: Vec<char> = sentence.chars().collect();

        let mut def: Cedict = Cedict::default();
        while !done {
            // create a word based on the start cursor and the end cursor
            let word: String = characters[start_cursor..end_cursor].to_vec().iter().collect();
            match self.dic.get(&word) {
                Some(definition) => {
                    def = definition.clone();
                    if end_cursor == characters.len() {
                        dictionnary.push(def.clone());
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
                        dictionnary.push(def.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_dictionnary() {
        let dictionnary = Dictionnary::new();
        let res = dictionnary.dic.get("上");
        println!("{res:?}");
    }

    #[test]
    fn expect_to_get_dictionnary_for_sentence() {
        let dictionnary = Dictionnary::new();
        let def = dictionnary.get_dictionnary_for_sentence("去年今夜");
        println!("{def:?}");
    }

    #[test]
    fn expect_to_get_dictionnary_for_complicated_sentence() {
        let dictionnary = Dictionnary::new();
        let def = dictionnary.get_dictionnary_for_sentence("去年今夜中國人同醉月明花樹下lol台灣");
        println!("{def:?}");
    }
}