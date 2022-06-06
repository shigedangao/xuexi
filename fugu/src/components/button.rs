use dioxus::{prelude::*, events::MouseEvent};

#[derive(Props, PartialEq)]
pub struct ButtonProps {
    text: String,
    color: String,
    background_color: String,
    on_click: fn(MouseEvent)
}

pub fn button(cx: Scope<ButtonProps>) -> Element {
    cx.render(rsx!{
        button {
            background_color: format_args!("{}", cx.props.background_color),
            color: format_args!("{}", cx.props.color),
            onclick: cx.props.on_click,
            "{cx.props.text}",
        }
    })
}