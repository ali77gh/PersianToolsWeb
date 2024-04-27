use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    rsx! {
        div { dir: "rtl", class: "flex flex-row p-2 items-center top-bar-grad",
            img { src: "/PersianToolsWeb/logo.png", width: "60px" }
            div { class: "text-white text-xl p-2", "مجموع ابزار فارسی" }
            div { class: "grow", "" }
            a {
                class: "text-white text-xl p-2",
                href: "https://github.com/persian-tools/rust-persian-tools",
                "گیت هاب"
            }
        }
    }
}
