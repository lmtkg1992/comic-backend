use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Authors {
    pub author_id: String,
    pub title: String,
    pub url_key: String,
    pub created_date: String,
    pub updated_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorStory {
    pub author_id: String,
    pub author_title: String,
    pub url_key: String
}