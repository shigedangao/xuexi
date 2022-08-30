use chamkho::Wordcut;
use crate::clean::Clean;
use crate::definition::DefinitionList;
use crate::chinese::Version;

// Default implementation
pub struct NoLang;

// State for Chinese language
pub struct Chinese;

// State for Lao language
pub struct Laotian;

pub enum Options {
    TraditionalChinese,
    Chinese(Version),
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

impl<L> Clean for Dictionary<L> {}
