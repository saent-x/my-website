
pub struct BlogPost<'a> {
    author: &'a str,
    date: &'a str,
    content: String
}

impl<'a> BlogPost<'a> {
    pub fn new(author: &'a str, date: &'a str, content: String) -> Self{
        Self{ author, date, content }
    }

    pub fn convert_content_to_html(&self) -> String {
        // use the crate:: pulldown-cmark to perform the conversion
        todo!("return the html representation of the markdown")
    }

    pub fn get_post_from_db() -> BlogPost<'a>{
        todo!("")
    }
}