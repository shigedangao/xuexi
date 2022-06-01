use iced::{button, pick_list};

mod view;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    TraditionalChinese
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::TraditionalChinese => write!(f, "Traditional Chinese")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LanguageSelected(Language),
    GenerateBtnPressed,
    ExportBtnPressed
}

#[derive(Default)]
pub struct App {
    input_text: String,
    // select element
    select: pick_list::State<Language>,
    selected_language: Option<Language>,
    // button
    generate_btn_pressed: button::State,
    export_btn_pressed: button::State
}