use dioxus::prelude::*;
use phasa::chinese::Dictionnary;

#[derive(Debug)]
pub enum SupportedLanguage {
    Chinese
}

impl From<String> for SupportedLanguage {
    fn from(lang: String) -> Self {
        match lang.as_str() {
            "chinese" => SupportedLanguage::Chinese,
            _ => SupportedLanguage::Chinese
        }
    }
}

impl std::fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLanguage::Chinese => write!(f, "chinese")
        }
    }
}

// State for the layout
pub static SELECTED_LANGUAGE: Atom<SupportedLanguage> = |_| SupportedLanguage::Chinese;
pub static USER_TEXT_INPUT: Atom<String> = |_| "".to_string();
pub static CHINESE_DICTIONNARY: Atom<Dictionnary> = |_| Dictionnary::new();