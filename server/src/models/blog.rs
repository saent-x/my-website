use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlogPost {
    pub author: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub category: String,
    pub content: String
}

#[derive(Debug, Deserialize)]
pub struct BlogPost {
    pub uuid: String,
    pub author: String,
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub content: String
}

impl BlogPost {
    pub fn new<'a>(uuid: &'a str, author: &'a str, date: &'a str, title: &'a str, description: &'a str, category: &'a str, content: String) -> Self{
        Self{ 
            uuid: uuid.to_string(),
            author: author.to_string(),
            date: date.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            content 
        }
    }

    pub fn convert_content_to_html(&self) -> String {
        let mut parser_options = Options::empty();
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    
        let parser = Parser::new_ext(&self.content, parser_options);
    
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        html_output
    }
}