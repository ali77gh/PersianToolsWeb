use dioxus::prelude::*;

use crate::core::tool::Tool;

#[derive(PartialEq, Props, Clone)]
pub struct ToolWrapper {
    pub tool: Tool,
}

#[component]
pub fn ToolListItem(props: ToolWrapper) -> Element {
    let tool = props.tool;

    let tags = tool
        .tags
        .iter()
        .map(|x| format!("#{}", x))
        .collect::<Vec<String>>()
        .join(", ");

    rsx! {
        div { dir: "rtl", class: "max-w-sm rounded  shadow-lg bg-white m-5 p-5",
            div { class: "font-bold text-xl mb-2", "{tool.name}" }
            div { class: "text-gray-700 text-base mb-3", "{tool.description}" }
            div { "{tags}" }
        }
    }
}
