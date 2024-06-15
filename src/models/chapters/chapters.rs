use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapters {
    pub chapter_id: String,
    pub story_id: String,
    pub title: String,
    pub content: String,
    pub ordered: i64,
    pub created_date: String,
    pub updated_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChapters {
    pub list: Vec<Chapters>,
    pub total: i64,
    pub total_page: i64,
}