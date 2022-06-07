use dioxus::prelude::*;

// State for the layout
pub static SELECTED_LANGUAGE: Atom<String> = |_| "chinese".to_string();
pub static USER_TEXT_INPUT: Atom<String> = |_| "".to_string();