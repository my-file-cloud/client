use dioxus::prelude::*;
use dioxus_style::{with_css};

#[with_css(style, "src/component/input/button.scss")]
#[component]
pub fn Button(
    on_click: EventHandler<Event<MouseData>>,
    style: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: style::button,
            onclick: on_click,
            
            {children}
        }
    }
}

#[with_css(style, "src/component/input/button.scss")]
#[component]
pub fn ButtonLabel(
    r#for: String,
    children: Element,
) -> Element {
    rsx! {
        label {
            class: style::button,
            
            r#for,
            {children}
        }
    }
}