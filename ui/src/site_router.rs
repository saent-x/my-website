use crate::pages::home::HomePage;
use crate::pages::blog::BlogPage;
use crate::pages::blogpost::BlogPostPage;
use crate::pages::about::AboutPage;
use crate::pages::contact::ContactPage;

use crate::components::layout::Layout;
use dioxus::prelude::*;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum SiteRoute {
    #[layout(Layout)]
    #[route("/")]
    HomePage {},

    #[route("/blog")]
    BlogPage { },

    #[route("/about")]
    AboutPage { },

    #[route("/contact")]
    ContactPage { },

    #[route("/blog/:blog_post_id")]
    BlogPostPage {blog_post_id: String}
}
