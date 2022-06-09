use dioxus::prelude::*;
use crate::state::{
    SELECTED_LANGUAGE,
    USER_TEXT_INPUT,
    CHINESE_DICTIONNARY,
    SupportedLanguage
};
use phasa::common::Ordered;

#[derive(Props, PartialEq)]
pub struct GeneratorProps {}

pub fn generator(cx: Scope<GeneratorProps>) -> Element {
    let targeted_language = use_read(&cx, SELECTED_LANGUAGE);
    let sentence = use_read(&cx, USER_TEXT_INPUT);
    let cn_dictionnary = use_read(&cx, CHINESE_DICTIONNARY);

    let res = match targeted_language {
        SupportedLanguage::Chinese => cn_dictionnary.get_list_detected_words(sentence),
    };

    println!("here");
    println!("{:?}", sentence);

    if res.is_none() {
        return cx.render(rsx! {
            div {
                "nothing"
            }
        });
    }

    let list = res.unwrap()
        .get_ordered_characters()
        .into_iter()
        .map(|(s, d)| {
            rsx!{
                p { "{s}" }
                p { "{d.prounciation}"}
            }
        });

    cx.render(rsx! {
        div {
            list
        }
    })
}