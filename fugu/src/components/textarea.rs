use dioxus::{prelude::*, events::FormEvent};

#[derive(Props)]
pub struct TextareaProps<'a> {
    on_input: EventHandler<'a, FormEvent>
}

pub fn textarea<'a>(cx: Scope<'a, TextareaProps<'a>>) -> Element {
    cx.render(rsx! {
        textarea {
            oninput: move |evt| cx.props.on_input.call(evt)
        }
    })
}