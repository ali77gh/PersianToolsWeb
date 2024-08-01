use dioxus::prelude::*;

use crate::core::tool::Tool;
use crate::ui::top_bar::TopBar;

#[component]
pub fn ToolPage(tool: Tool) -> Element {
    rsx! {
        div { class: "rust-gradient h-screen w-screen flex flex-col",
            TopBar {}
            ToolUse { tool }
        }
    }
}

#[component]
pub fn ToolUse(tool: Tool) -> Element {
    let mut input = use_signal(|| tool.sample_input.to_string());

    let tags = tool.tags.iter().map(|tag| {
        rsx! {
            div { class: "inline-block bg-gray-200 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mt-1",
                "#{tag}"
            }
        }
    });

    let mut out = "".to_string();
    let mut err = "".to_string();
    match (tool.exe)(&input.read()) {
        Ok(s) => {
            out = s;
        }
        Err(e) => {
            err = e;
        }
    }

    let out_lines = out.split('\n').map(|e| {
        rsx! {
            div { "{e}" }
        }
    });

    rsx! {
        div { class: "grow flex flex-col items-center",
            div {
                dir: "rtl",
                class: "max-w-2xl rounded  shadow-lg bg-white m-5 p-5 grid justify-items-stretch items-center justify-self-center",
                div { class: "font-bold text-xl mb-2", "{tool.name}" }
                div { class: "text-gray-700 text-base mb-4 mt-2", "{tool.description}" }
                div { {tags} }
                input {
                    dir: "rtl",
                    class: "rounded m-2 p-3 border-2 border-black mt-4 mb-4",
                    placeholder: "ورودی",
                    value: "{input}",
                    oninput: move |event| input.set(event.value())
                }
                div { class: "flex flex-row text-red-600", "{err}" }
                div { {out_lines} }
                a {
                    href: "{tool.doc_link}",
                    target: "_blank",
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-3 rounded justify-self-end",
                    "مستندات"
                }
            }
        }
    }
}
