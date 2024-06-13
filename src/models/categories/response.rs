use serde::{Deserialize, Serialize};
use crate::models::categories::categories::{ListCategories};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListCategoriesResponse {
    pub message: String,
    pub error: bool,
    pub data: ListCategories
}