use crate::{error::DictionaryError, word::Word};
use chamkho::Wordcut;
use dodo_zh::variant::KeyVariant;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

pub enum Lang {
    Chinese(KeyVariant),
    Laotian(Option<Box<Wordcut>>),
}

// Blanket state implementation used to initialize Dictionary for different language.
/// A Chinese dictionary
pub struct Chinese;
/// A Laotian dictionary
pub struct Laotian;

/// Dictionary contains the definitions of the dictionary.
/// A dictionary can support multiple language for multiple Initializer trait implementation
/// done through the usage of the PhantomData
///
/// The Dictionary struct usually implements the Initializer trait.
pub struct Dictionary<T> {
    pub _lang: PhantomData<T>,
    pub dict: HashMap<String, Word>,
    pub punctuation: Vec<String>,
    pub params: Lang,
}

pub(crate) trait Initializer<T> {
    /// Initialize the dictionary with default parameters
    ///
    /// # Arguments
    ///
    /// * `params` - Lang
    fn initialize(params: Lang) -> Result<Self, DictionaryError>
    where
        Self: Sized;

    /// Load the dictionary with the given file path (e.g: cedict, chamko...)
    ///
    /// # Arguemnts
    ///
    /// * `path` - PathBuf
    fn load(&mut self, path: PathBuf) -> Result<(), DictionaryError>;
}
