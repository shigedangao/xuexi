#[cfg(feature = "chinese")]
pub mod chinese;

pub mod dictionary;
pub mod error;
pub mod export;

#[cfg(feature = "laotian")]
pub mod laotian;

mod punctuation;
pub(crate) mod util;
pub mod word;

#[allow(unused_imports)]
use dictionary::{Chinese, Initializer, Lang, Laotian};

#[allow(unused_imports)]
use std::{collections::BTreeMap, path::PathBuf};
#[allow(unused_imports)]
use word::{Word, WordParser};

// Re export KeyVariant
pub use dodo_zh::variant::KeyVariant;

/// Load a chinese dictionnary which allows you to get a list of chinese definitions
#[cfg(feature = "chinese")]
pub fn load_chinese_dictionary(
    version: Lang,
    path: PathBuf,
) -> Result<dictionary::Dictionary<Chinese>, error::DictionaryError> {
    let mut dictionary = dictionary::Dictionary::<Chinese>::initialize(version)?;
    dictionary.load(path)?;

    Ok(dictionary)
}

/// Load a laotian dictionnary which allows you to get a list of laotian word definitions
#[cfg(feature = "laotian")]
pub fn load_laotian_dictionary(
    path: PathBuf,
) -> Result<dictionary::Dictionary<Laotian>, error::DictionaryError> {
    let mut dictionary = dictionary::Dictionary::<Laotian>::initialize(Lang::Laotian(None))?;
    dictionary.load(path)?;

    Ok(dictionary)
}
