use dioxus::prelude::*;

use crate::core::find_by_route;
use crate::ui::home::Home;
use crate::ui::tool::ToolPage;

#[component]
pub fn MyRouter() -> Element {
    let href = get_href();
    let sp: Vec<&str> = href.split("/").collect();
    let last = sp.last();
    if let Some(last) = last {
        if let Some(tool) = find_by_route(&last) {
            rsx!(ToolPage { tool })
        } else {
            rsx!(Home {})
        }
    } else {
        rsx!(Home {})
    }
}

fn get_href() -> String {
    let w = web_sys::window().unwrap();
    let href = w.location().href().unwrap();
    href
}
