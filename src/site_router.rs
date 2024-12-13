use crate::pages::{home::Home, blog::Blog};
use crate::components::layout::Layout;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRoute {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}