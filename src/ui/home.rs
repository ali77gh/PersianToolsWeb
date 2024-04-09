use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(name: String) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Tool: {name}"
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                name: count().to_string(),
            },
            "Open tool"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
