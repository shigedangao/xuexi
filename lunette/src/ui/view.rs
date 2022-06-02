use iced::{Row, PickList, Sandbox, Element, Container, Button, Text, TextInput, Column};
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
            Message::GenerateBtnPressed => {
                self.words.push("foo".to_owned());
            },
            Message::ExportBtnPressed => println!("export pressed"),
            Message::OnInput(content) => {
                self.input_text = content;
            }
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

        let mut test_col = Column::new();
        for item in self.words.iter() {
            test_col = test_col.push(Text::new(item));
        }

        let content_row = Row::new()
            .push(
                TextInput::new(
                    &mut self.text_input_widget,
                    "This is a placeholder",
                    &self.input_text,
                    Message::OnInput
                )
            )
            .push(
                test_col
            );

        let col = Column::new()
            .push(row)
            .push(content_row);

        Container::new(col)
            .into()
    }
}