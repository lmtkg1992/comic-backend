use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapters {
    pub chapter_id: String,
    pub story_id: String,
    pub increment_id: i64, // New field for auto-increment ID
    pub title: String,
    pub short_title: String,
    pub url_key: String,
    pub content: String,
    pub ordered: i64,
    pub status: String,
    pub created_date: String,
    pub updated_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChapters {
    pub list: Vec<Chapters>,
    pub total: i64,
    pub total_page: i64,
}