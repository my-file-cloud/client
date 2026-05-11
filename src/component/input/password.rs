use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use dioxus_style::with_css;

#[component]
#[with_css(style, "src/component/input/password.scss")]
pub fn PasswordInput(
    value: String,
    placeholder: Option<String>,
    on_input: EventHandler<Event<FormData>>,
) -> Element {
    let mut is_password_visible = use_signal(|| false);
    
    let input_type = if is_password_visible() { "text" } else { "password" };
    
    rsx! {
        div {
            class: style::container,
            
            input {
                class: style::input,
                
                r#type: input_type,
                
                value,
                placeholder,
                oninput: on_input,
            }
            div {
                style: "margin-right: 5px; display: flex",
                
                VisibilityToggle {
                    on_toggle: move |val: bool| {
                        is_password_visible.set(!val);
                    }
                }
            }
        }
    }
}

#[component]
#[with_css(style, "src/component/input/password.scss")]
fn VisibilityToggle(on_toggle: Callback<bool>) -> Element {
    // keep the checked state internally
    let checked = use_signal(|| false);

    let toggle = {
        let mut checked = checked.clone();
        let on_toggle = on_toggle.clone();
        move |_| {
            checked.set(!checked());
            on_toggle(!checked());
        }
    };

    rsx! {
        label {
            class: style::visibility_toggle,
            
            input {
                display: "none",
                
                r#type: "checkbox",
                checked: "{checked()}",
                onclick: toggle,
            }

            MaterialIcon {
                name: if checked() { "visibility_off" } else { "visibility" },
                color: MaterialIconColor::Light,
            }
        }
    }
}