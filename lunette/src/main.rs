use iced::{Settings, Sandbox};

mod ui;

fn main() -> iced::Result {
    ui::App::run(Settings::default())
}
