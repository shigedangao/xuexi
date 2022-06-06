use dioxus::prelude::*;
use crate::theme;
use crate::containers::layout;

pub fn app(cx: Scope) -> Element {
    // Create the theme which will be used accross the app
    let theme_colors = theme::get_theme_color("dark");

    // set the theme in the global state
    cx.use_hook(|_| { cx.provide_context(theme_colors); });

    cx.render(
        rsx!(
            layout::layout{}
        )
    )
}