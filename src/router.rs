use crate::ui::home::Blog;
use crate::ui::home::Home;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/tool/:name")]
    Blog { name: String },
}
