use dioxus::prelude::*;

use crate::components::Layout;
use crate::pages::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRouter {
    #[layout(Layout)]
    #[route("/")]
    Home {}
}