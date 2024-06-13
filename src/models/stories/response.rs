use serde::{Deserialize, Serialize};
use crate::models::stories::stories::{ListStories};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListStoriesResponse {
    pub message: String,
    pub error: bool,
    pub data: ListStories
}