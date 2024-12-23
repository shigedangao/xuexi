use crate::error::DictionaryError;
use chamkho::wordcut_engine::{create_prefix_tree, Dict};
use std::{
    io::{BufRead, BufReader},
    ops::Deref,
};

/// Decorator over the chamkho::load_dict method in order to add
/// support to load a set of laotian words from a list of &[u8].
/// This use case is due to the fact that the chamkho library compute dictionary path
/// at compile time. As such, this could not really be shipped to third parties project such as binaries
/// /!\ Note we also uses a different set of words
pub fn load_laotian_words() -> Result<Dict, DictionaryError> {
    let lao_words: &[u8] = include_bytes!("../../laodict.txt");
    let reader = BufReader::new(lao_words);
    let mut definitions = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.contains('#') {
            continue;
        }

        definitions.push(line);
    }

    let d: Vec<_> = definitions.iter().map(|d| d.deref()).collect();
    let tree = create_prefix_tree(&d);
    Ok(tree)
}
