// src/repositories/top_story_repository/mod.rs

mod routes;
mod top_story_repository;

pub use routes::init_routes;
pub use top_story_repository::{TopStoryRepository};