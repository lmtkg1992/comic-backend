use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stories {
    pub story_id: String,
    pub increment_id: i64, // New field for auto-increment ID
    pub title: String,
    pub url_key: String,
    pub author: String,
    pub description: String,
    pub publish_date: String,
    pub status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListStories {
    pub list: Vec<Stories>,
    pub total: i64,
    pub total_page: i64,
}