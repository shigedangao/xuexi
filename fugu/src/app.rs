use dioxus::prelude::*;
use crate::theme;
use crate::containers::layout;
use crate::state::CHINESE_DICTIONNARY;
use phasa;

pub fn app(cx: Scope) -> Element {
    let set_dictionnary = use_set(&cx, CHINESE_DICTIONNARY);
    // load the dictionnary before rendering anything
    let dictionnary = phasa::load_chinese_dictionnary();
    // Create the theme which will be used accross the app
    let theme_colors = theme::get_theme_color("dark");

    // set the theme in the global state
    cx.use_hook(|_| { cx.provide_context(theme_colors); });
    // set the dictionnary
    set_dictionnary(dictionnary);

    cx.render(
        rsx!(
            layout::layout{}
        )
    )
}