use crate::ui::home::Home;
use crate::ui::tool::ToolPage;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/:..segments")]
    ToolPage { segments: Vec<String> },
}
