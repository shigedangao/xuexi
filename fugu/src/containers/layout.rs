use dioxus::core::Scope;
use dioxus::prelude::*;
use crate::theme::Colors;
use super::navbar;

// State for the layout
pub static SELECTED_LANGUAGE: Atom<String> = |_| "chinese".to_string();

#[derive(Props, PartialEq)]
pub struct LayoutProps {}

// Create the layout of the app
pub fn layout(cx: Scope) -> Element {
    let colors = match cx.use_hook(|_| cx.consume_context::<Colors>()) {
        Some(colors) => colors.to_owned(),
        None => Colors::from_dark_theme()
    };

    cx.render(rsx! {
        div {
            class: format_args!("{}", colors.class_name),
            style { [include_str!("../assets/style.css")] }
            "Hello",
            navbar::navbar {}
        }
    })
}