use dioxus::{prelude::*, events::FormEvent};
use crate::{state::USER_TEXT_INPUT, components::textarea};

#[derive(Props, PartialEq)]
pub struct EditorProps {}

pub fn editor(cx: Scope<EditorProps>) -> Element {
    let set_text = use_set(&cx, USER_TEXT_INPUT);

    cx.render(rsx! {
        textarea::textarea {
            on_input: |e: FormEvent| {
                set_text(e.value.clone());
            }
        }
    })
}