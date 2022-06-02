use iced::{button, pick_list, text_input};

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
    ExportBtnPressed,
    OnInput(String)
}

#[derive(Default)]
pub struct App {
    input_text: String,
    words: Vec<String>,
    // select element
    select: pick_list::State<Language>,
    selected_language: Option<Language>,
    // button
    generate_btn_pressed: button::State,
    export_btn_pressed: button::State,
    // text input state
    text_input_widget: text_input::State,
}