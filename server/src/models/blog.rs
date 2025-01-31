use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySchema {
    pub uuid: String,
    pub name: String
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlogPostSchema {
    pub uuid: Option<String>,
    pub author: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub category: Vec<CategorySchema>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlogPost {
    pub author: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub category: Vec<String>,
    pub content: String,
}


impl BlogPostSchema {
    pub fn convert_content_to_html(&mut self) {
        let mut parser_options = Options::empty();
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
        
        let content = match &self.content{
            Some(value) => value.clone(),
            None => "".to_string()
        };

        let parser = Parser::new_ext(&content, parser_options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        self.content = Some(html_output);
    }
}
