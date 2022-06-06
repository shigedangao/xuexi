use dioxus::{prelude::*, events::FormEvent};
use crate::components::dropdown;
use super::layout::SELECTED_LANGUAGE;

#[derive(Props, PartialEq)]
pub struct NavbarProps {}

pub fn navbar(cx: Scope) -> Element {
    let set_language = use_set(&cx, SELECTED_LANGUAGE);
    let get_language = use_read(&cx, SELECTED_LANGUAGE);

    cx.render(rsx! {
        div {
            class: "navbar__container",
            dropdown::dropdown {
                list: vec!["chinese", "laotian"],
                on_change: |e: FormEvent| {
                    set_language(e.value.clone());
                },
            }
            p {
                "Selected language: {get_language}"
            }
            // add two button (generate & export)
        }
    })
} 