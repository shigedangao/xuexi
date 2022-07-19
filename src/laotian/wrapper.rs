use std::{io::{BufReader, BufRead}, ops::Deref};
use chamkho::wordcut_engine::{Dict, create_prefix_tree};
use crate::error::LibError;

/// Decorator over the chamkho::load_dict method in order to add
/// support to load a set of laotian words from a list of &[u8].
/// This use case is due to the fact that the chamkho library compute dictionary path
/// at compile time. As such, this could not really be shipped to third parties project such as binaries
/// /!\ Note we also uses a different set of words
pub fn load_laotian_words() -> Result<Dict, LibError> {
    let lao_words: &[u8] = include_bytes!("../../laodict.txt");
    let reader = BufReader::new(lao_words);
    let mut definitions = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.contains("#") {
                continue;
            }

            definitions.push(l);
        }
    }

    let d: Vec<_> = definitions.iter().map(|d| d.deref()).collect();
    let tree = create_prefix_tree(&d);
    Ok(tree)
}
