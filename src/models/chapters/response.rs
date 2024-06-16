use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChaptersByStoryId {
    pub chapter_id: String,
    pub story_id: String,
    pub increment_id: i64, // New field for auto-increment ID
    pub title: String,
    pub url_key: String,
    pub ordered: i64,
    pub status: String,
    pub created_date: String,
    pub updated_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChaptersByStoryId {
    pub list: Vec<ChaptersByStoryId>,
    pub total: i64,
    pub total_page: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChaptersResponse {
    pub message: String,
    pub error: bool,
    pub data: ListChaptersByStoryId
}