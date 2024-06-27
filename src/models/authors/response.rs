use serde::{Deserialize, Serialize};
use crate::models::authors::authors::Authors;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorResponse {
    pub message: String,
    pub error: bool,
    pub data: Option<Authors>,
}