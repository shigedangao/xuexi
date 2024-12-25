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

/// Load a chinese dictionnary allowing you to get a list of chinese definitions for a given sentence
///
/// # Arguments
///
/// * `version` - Lang
/// * `path` - PathBuf
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use xuexi::{
///     self,
///     KeyVariant,
///     dictionary::Lang,
///     word::WordParser
/// };
///
///
/// let chinese_dict = xuexi::load_chinese_dictionary(Lang::Chinese(KeyVariant::Traditional), PathBuf::from("./cedict_ts.u8")).unwrap();
/// let res = chinese_dict.parse_sentence_into_words("你好我是馬克的摯友");
///
/// assert_eq!(res.get("你好").unwrap().written, vec!["你好", "你好"]);
/// ```
#[cfg(feature = "chinese")]
pub fn load_chinese_dictionary(
    version: Lang,
    path: PathBuf,
) -> Result<dictionary::Dictionary<Chinese>, error::DictionaryError> {
    let mut dictionary = dictionary::Dictionary::<Chinese>::initialize(version)?;
    dictionary.load(path)?;

    Ok(dictionary)
}

/// Load a laotian dictionnary allowing you to get a list of laotian word definitions for a given sentence
///
/// # Arguments
///
/// * `path` - PathBuf
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use xuexi::{
///     self,
///     KeyVariant,
///     dictionary::Lang,
///     word::WordParser
/// };
///
/// let lao_dict = xuexi::load_laotian_dictionary(PathBuf::from("./lao-eng-dictionary.csv")).unwrap();
/// let res = lao_dict.parse_sentence_into_words("ມື້ນີ້ແມ່ນວັນຄຣິດສະມາດ");
///
/// assert_eq!(res.get("ມື້ນີ້").unwrap().written, vec!["ມື້ນີ້"]);
/// ```
#[cfg(feature = "laotian")]
pub fn load_laotian_dictionary(
    path: PathBuf,
) -> Result<dictionary::Dictionary<Laotian>, error::DictionaryError> {
    let mut dictionary = dictionary::Dictionary::<Laotian>::initialize(Lang::Laotian(None))?;
    dictionary.load(path)?;

    Ok(dictionary)
}
