use crate::core::find_by_route;
use crate::ui::home::Home;
use crate::ui::tool::{ToolPage, ToolPageProps};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/:..segments")]
    MyRouter { segments: Vec<String> },
}

#[component]
fn MyRouter(segments: Vec<String>) -> Element {
    match segments.last() {
        Some(last) => match find_by_route(&last) {
            Some(tool) => ToolPage(ToolPageProps { tool }),
            None => Home(),
        },
        None => Home(),
    }
}
