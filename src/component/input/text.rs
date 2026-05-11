use dioxus::prelude::*;

#[component]
pub fn TextInput(
    value: String,
    placeholder: Option<String>,
    on_input: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        input {
            /* styling */
            background_color: "#333",
            border: "1px solid gray",
            border_radius: "10px",
            outline: "none",
            
            padding: "5px",
            margin: "5px",
            text_indent: "5px",
            
            
            /* properties */
            r#type: "text",
            
            /* propagated properties */
            value,
            placeholder,
            oninput: on_input,
        }
    }
}