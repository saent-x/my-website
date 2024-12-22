use pulldown_cmark::{html, Options, Parser};

#[derive(Debug, Clone)]
pub struct BlogPostDTO<'a> {
    pub id: u32,
    pub author: &'a str,
    pub date: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub category: &'a str,
}

#[derive(Debug)]
pub struct BlogPost<'a> {
    pub id: u32,
    pub author: &'a str,
    pub date: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub category: &'a str,
    pub content: String
}

impl<'a> BlogPost<'a> {
    pub fn new(id: u32, author: &'a str, date: &'a str, title: &'a str, description: &'a str, category: &'a str, content: String) -> Self{
        Self{ id, author, date, title, description, category, content }
    }

    pub fn convert_content_to_html(&self) -> String {
        let mut parser_options = Options::empty();
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);

        let parser = Parser::new_ext(&self.content, parser_options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn get_post_from_db() -> BlogPost<'a> {
        todo!("")
    }
}