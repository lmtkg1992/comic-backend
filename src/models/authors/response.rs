use serde::{Deserialize, Serialize};
use crate::models::authors::authors::{Authors, ListAuthors};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorResponse {
    pub message: String,
    pub error: bool,
    pub data: Option<Authors>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAuthorsResponse {
    pub message: String,
    pub error: bool,
    pub data: ListAuthors,
}