use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::pages::{home::Home, update_category::UpdateCategory, add_post::AddBlogPost, posts::Posts, update_posts::UpdatePosts, messages::Messages};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum SiteRouter {
    #[layout(Layout)]
    
    #[route("/")]
    Home {},
    
    #[route("/category")]
    UpdateCategory {},
    
    #[route("/add_post")]
    AddBlogPost {},
    
    #[route("/update_post/:uuid")]
    UpdatePosts {uuid: String},
    
    #[route("/posts")]
    Posts {},
    
    #[route("/messages")]
    Messages {},
}