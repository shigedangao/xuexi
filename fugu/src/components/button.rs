use dioxus::{prelude::*, events::MouseEvent};

#[derive(Props)]
pub struct ButtonProps<'a> {
    class_name: &'a str,
    text: &'a str,
    on_click: EventHandler<'a, MouseEvent>
}

pub fn button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx!{
        button {
            class: format_args!("{}", cx.props.class_name),
            onclick: move |evt| cx.props.on_click.call(evt),
            "{cx.props.text}",
        }
    })
}