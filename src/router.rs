use crate::ui::home::Home;
use crate::ui::tool::Tool;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/tool/:name")]
    Tool { name: String },
}
