use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    rsx! {
        div { class: "flex flex-row p-4 items-center top-bar-grad",
            img { src: "./logo.png", width: "50px", height: "50px" }
            div { "Persian-Tools-Web" }
            div { class: "grow", "" }
            a { href: "https://github.com/persian-tools/rust-persian-tools", "Github" }
        }
    }
}
