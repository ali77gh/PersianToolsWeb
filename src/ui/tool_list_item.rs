use dioxus::prelude::*;

use crate::core::tool::Tool;

#[derive(PartialEq, Props, Clone)]
pub struct ToolWrapper {
    pub tool: Tool,
}

#[component]
pub fn ToolListItem(props: ToolWrapper) -> Element {
    let tool = props.tool;

    let tags = tool.tags.iter().map(|tag| {
        rsx! {
            div { class: "inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mt-1",
                "#{tag}"
            }
        }
    });

    rsx! {
        div {
            dir: "rtl",
            class: "max-w-2xl rounded  shadow-lg bg-white m-5 p-5 grid justify-items-stretch",
            div { class: "font-bold text-xl mb-2", "{tool.name}" }
            div { class: "text-gray-700 text-base mb-4 mt-2", "{tool.description}" }
            div { {tags} }
            a {
                href: "/PersianToolsWeb/tool/{tool.route}",
                target: "_blank",
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-3 rounded justify-self-end",
                "شروع"
            }
        }
    }
}
