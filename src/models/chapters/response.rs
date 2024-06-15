use serde::{Deserialize, Serialize};
use crate::models::chapters::chapters::{ListChapters};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChaptersResponse {
    pub message: String,
    pub error: bool,
    pub data: ListChapters
}