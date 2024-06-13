use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostStoryCategoryMapping {
    pub story_id: String,
    pub category_ids: Vec<String>
}