// src/models/top_story/top_story.rs

use serde::{Deserialize, Serialize};
use crate::models::stories::stories::{Stories};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopStory {
    pub period: String,
    pub story_id: String,
    pub priority: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListTopStories {
    pub list: Vec<Stories>,
    pub total: i64,
    pub total_page: i64,
}