use dioxus::prelude::*;

use crate::core::TOOLS;
use rust_persian_tools::digits::en_to_fa;

#[component]
pub fn TopBar() -> Element {
    let top_bar_text = format!(
        "مجموعه {} ابزار آفلاین فارسی ایرانی",
        en_to_fa(TOOLS.len().to_string())
    );

    rsx! {
        div { dir: "rtl", class: "flex flex-row p-2 items-center top-bar-grad",
            img { src: "/PersianToolsWeb/logo.png", width: "60px" }
            div { class: "text-white text-xl p-2", {top_bar_text} }
            div { class: "grow", "" }
            a {
                class: "text-white text-xl p-2",
                href: "https://github.com/ali77gh/PersianToolsWeb",
                "گیت هاب"
            }
        }
    }
}
