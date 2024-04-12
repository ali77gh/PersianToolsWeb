use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Tool(name: String) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Tool: {name}"
    }
}
