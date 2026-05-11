use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css(style, "src/component/modal.scss")]
#[component]
pub fn Modal(
    children: Element,
) -> Element {
    rsx! {
        div {
            popover: "auto",
            
            class: style::body,
            
            div {
                class: style::inner,
                
                {children}
            }
        }
    }
}