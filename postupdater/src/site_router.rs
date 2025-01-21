use dioxus::prelude::*;

use crate::components::Layout;
use crate::pages::{Home, UpdateCategory};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRouter {
    #[layout(Layout)]
    
    #[route("/")]
    Home {},
    
    #[route("/cat")]
    UpdateCategory {}
}