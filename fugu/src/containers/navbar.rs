use dioxus::{
    prelude::*,
    events::FormEvent
};
use crate::components::{
    button,
    dropdown
};
use crate::state::SELECTED_LANGUAGE;

#[derive(Props, PartialEq)]
pub struct NavbarProps {}

pub fn navbar(cx: Scope<NavbarProps>) -> Element {
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
            },
            button::button {
                class_name: "btn__navbar dark-btn-text",
                text: "Generate",
                on_click: |_| {
                    println!("click generate")
                }
            },
            button::button {
                class_name: "btn__navbar dark-btn",
                text: "Export",
                on_click: |_| {
                    println!("click export")
                }
            }
        }
    })
} 