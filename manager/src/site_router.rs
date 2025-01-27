use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::pages::{home::Home, update_category::UpdateCategory, add_post::AddBlogPost, posts::Posts};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRouter {
    #[layout(Layout)]
    
    #[route("/")]
    Home {},
    
    #[route("/update_category")]
    UpdateCategory {},
    
    #[route("/add_post")]
    AddBlogPost {},
    
    #[route("/posts")]
    Posts {},
}