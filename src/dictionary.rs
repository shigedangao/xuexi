use chamkho::Wordcut;
use crate::clean::Clean;
use crate::definition::DefinitionList;
use crate::error::LibError;

// Default implementation
pub struct NoLang;

// State for Chinese language
pub struct Chinese;

// State for Lao language
pub struct Laotian;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChineseVersion {
    Traditional,
    Simplified
}

impl Default for ChineseVersion {
    fn default() -> Self { ChineseVersion::Traditional }
}

pub enum Options {
    Chinese(ChineseVersion),
    Laotian(Box<Wordcut>)
}

/// Dictionary is a generic dictionary which can handle
/// multiple languages based on the given language by leveraging the type system.
/// 
/// # Examples
/// 
/// Dictionary::<Chinese>::new(Version::TraditionalChinese) // Load the chinese dictionary and it's associated methods.
/// Dictionar::<Laotian>::new() // Load the laotian dictionary and it's associated methods.
pub struct Dictionary<Lang = NoLang> {
    pub lang: std::marker::PhantomData<Lang>,
    pub dic: DefinitionList,
    pub punctuation: Vec<String>,
    pub options: Options
}

/// Dictionary Loader are a set of common method that each dictionary of 
/// lang T need to implement
pub trait DictionaryLoader<T> {
    fn new_lang() -> Result<Dictionary<T>, LibError>;
    fn load(&mut self) -> Result<(), LibError>;
}

impl<L> Clean for Dictionary<L> {}
