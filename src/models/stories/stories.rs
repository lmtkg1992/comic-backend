use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stories {
    pub story_id: String,
    pub increment_id: i64,
    pub title: String,
    pub url_key: String,
    pub is_active: bool,
    pub path_image: String,
    pub author_id: String,
    pub description: String,
    pub publish_date: String,
    pub updated_date: String,
    pub status: String,
    pub is_full: bool,
    pub is_hot: bool,
    pub total_chapters: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListStories {
    pub list: Vec<Stories>,
    pub total: i64,
    pub total_page: i64,
}