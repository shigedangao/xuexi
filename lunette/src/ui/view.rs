use iced::{Row, PickList, Sandbox, Element, Container, Button, Text};
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
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = Some(language);
            },
            Message::GenerateBtnPressed => println!("generate pressed"),
            Message::ExportBtnPressed => println!("export pressed")
        }
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
            )
            // push a generate button
            .push(
                Button::new(&mut self.generate_btn_pressed, Text::new("Generate"))
                    .on_press(Message::GenerateBtnPressed)
            )
            // push an export button
            .push(
                Button::new(&mut self.export_btn_pressed, Text::new("Export"))
                    .on_press(Message::ExportBtnPressed)
            );

        Container::new(row).into()
    }
}