use crate::ui::{search_tool::SearchTool, top_bar::TopBar};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "rust-gradient h-screen w-screen flex flex-col",
            TopBar {}
            SearchTool {}
        }
    }
}
