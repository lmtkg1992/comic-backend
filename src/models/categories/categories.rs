use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub category_id: String,
    pub increment_id: i64,
    pub title: String,
    pub url_key: String,
    pub priority: i64,
    pub type_category: String, // category, tag, or number_chapters
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListCategories {
    pub list: Vec<Category>,
    pub total: i64,
    pub total_page: i64,
}