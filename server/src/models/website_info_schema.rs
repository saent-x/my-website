use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct WebsiteInfo {
    pub title: String,
    pub profile: Profile,
    pub projects: Vec<Project>,
    pub work_experience: Vec<WorkExperience>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub job_title: String,
    pub about_me: String,
    pub resume_link: String,
    pub home_page_info: String,
    pub skills: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub link: String,
    pub img_url: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct WorkExperience {
    pub job_title: String,
    pub company: String,
    pub from: String,
    pub to: String,
}