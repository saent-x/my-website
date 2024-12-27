
#[derive(Debug, Clone)]
pub struct BlogPostDTO<'a> {
    pub id: u32,
    pub author: &'a str,
    pub date: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub category: &'a str,
}