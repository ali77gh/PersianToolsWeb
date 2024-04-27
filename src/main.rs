#![allow(non_snake_case)]

mod core;
mod router;
mod ui;

use dioxus::prelude::*;
use log::LevelFilter;
use router::Route;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
