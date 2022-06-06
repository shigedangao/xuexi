use dioxus::{prelude::*, events::FormEvent};

#[derive(Props)]
pub struct DropdownProps<'a> {
    list: Vec<&'a str>,
    on_change: EventHandler<'a, FormEvent>
}

pub fn dropdown<'a>(cx: Scope<'a, DropdownProps<'a>>) -> Element {
    let options = cx.props.list.clone()
        .into_iter()
        .map(|l| rsx! {
            option {
                label: format_args!("{}", l),
                value: format_args!("{}", l),
            }
        });
    
    cx.render(rsx! {
        select {
            onchange: move |evt| cx.props.on_change.call(evt),
            options
        }
    })
} 