use iced::{Column, Row, PickList, Sandbox, Element, Container};
use super::{Language, App, Message};

// Constant
const LANGUAGE: [Language; 1] = [
    Language::TraditionalChinese
];
const SELECT_PLACEHOLDER: &str = "Select a language";

impl Sandbox for App {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Lunette")
    }

    fn new() -> Self {
        App::default()
    }

    fn update(&mut self, message: Self::Message) {
        
    }

    fn view(&mut self) -> Element<Self::Message> {
        let row = Row::new()
            .push(
                PickList::new(
                    &mut self.select,
                    &LANGUAGE[..],
                    self.selected_language,
                    Message::LanguageSelected
                )
                .placeholder(SELECT_PLACEHOLDER)
            );

        Container::new(row).into()
    }
}