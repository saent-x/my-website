use serde::{Deserialize, Serialize};


#[derive(Default, Clone, Debug, Deserialize)]
pub struct WebsiteInfoDTO {
    pub title: String,
    pub profile: Profile,
    pub projects: Vec<Project>,
    pub work_experience: Vec<WorkExperience>,
    pub socials: Vec<Social>
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub job_title: String,
    pub about_me: String,
    pub resume_link: String,
    pub home_page_info: String,
    pub skills: String,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub link: String,
    pub img_url: String,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct WorkExperience {
    pub job_title: String,
    pub company: String,
    pub from: String,
    pub to: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Social {
    pub name: String,
    pub img_url: String,
    pub link: String
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize)]
pub struct CategoryDTO {
    pub uuid: Option<String>,
    pub name: String
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct BlogPostDTO {
    pub uuid: String,
    pub author: String,
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: Vec<CategoryDTO>,
    pub content: Option<String>
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ContactFormDTO {
    pub name: String,
    pub email: String,
    pub content: String
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: String,
    pub data: T
}

impl<T: Default> ApiResponse<T> {
    pub fn error() -> Self{
        Self {
            status: "error".to_string(),
            code: "500".to_string(),
            data: T::default() // applies the default of the generic value
        }
    }
}
