use dioxus::prelude::*;

// use crate::core::get_all_tools;
use crate::{
    core::{tool::Tool, TOOLS},
    ui::tool_list_item::ToolListItem,
};

#[component]
pub fn SearchTool() -> Element {
    let mut input_search = use_signal(|| "".to_string());

    let tools_list = TOOLS.iter().map(|tool| {
        rsx! {
            if check_match(&input_search.read(), &tool) {
                ToolListItem { tool: tool.clone() }
            }
        }
    });

    rsx! {
        div { dir: "rtl", class: "grow flex flex-col items-center",
            input {
                dir: "rtl",
                class: "rounded shadow-lg bg-white m-5 p-3",
                placeholder: "جست و جو",
                value: "{input_search}",
                oninput: move |event| input_search.set(event.value())
            }
            div { class: "items-center", {tools_list} }
        }
    }
}

fn check_match(query: &str, tool: &Tool) -> bool {
    if tool.name.contains(query) {
        return true;
    }
    false
}
