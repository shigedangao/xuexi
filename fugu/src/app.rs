use std::sync::Arc;
use dioxus::prelude::*;
use crate::theme;
use crate::containers::layout;
use phasa;

pub fn app(cx: Scope) -> Element {
    // load the dictionnary before rendering anything
    let dictionnary = phasa::load_chinese_dictionnary();
    // Create the theme which will be used accross the app
    let theme_colors = theme::get_theme_color("dark");

    // set the theme in the global state
    cx.use_hook(|_| { cx.provide_context(theme_colors); });
    cx.use_hook(|_| cx.provide_context(Arc::new(dictionnary)));

    cx.render(
        rsx!(
            layout::layout{}
        )
    )
}