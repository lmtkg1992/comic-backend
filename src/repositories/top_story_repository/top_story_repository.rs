use crate::config::{Config, IConfig};
use crate::models::stories::stories::{Stories, ListStories};
use crate::models::stories::response::{ListStoriesResponse};
use crate::repositories::stories_repository::StoriesRepository;
use futures::stream::StreamExt;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::Client;
use std::collections::HashMap;

pub struct TopStoryRepository {
    pub connection: Client,
}

impl TopStoryRepository {

    pub async fn get_list_by_period(
        &self,
        period: &str,
        category_id: Option<&str>,
        query_string: HashMap<String, String>,
    ) -> Result<ListStoriesResponse, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let top_stories_collection_name = _config.get_config_with_key("TOP_STORIES_COLLECTION_NAME");
        let stories_collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
        let mapping_collection_name = _config.get_config_with_key("STORIES_CATEGORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        // Step 1: Filter by period to get the top stories
        let filter = doc! { "period": period };

        let mut cursor = db
            .collection(top_stories_collection_name.as_str())
            .find(filter, None)
            .await
            .unwrap();

        let mut story_ids: Vec<String> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    if let Some(story_id) = doc.get_str("story_id").ok() {
                        story_ids.push(story_id.to_string());
                    }
                }
                Err(_err) => (),
            }
        }

        // Step 2: If category_id is provided, filter stories by category_id
        if let Some(category_id) = category_id {
            let category_filter = doc! {
                "story_id": { "$in": &story_ids },
                "category_id": category_id,
            };
            let mut category_cursor = db.collection(mapping_collection_name.as_str()).find(category_filter, None).await?;
            let mut filtered_story_ids: Vec<String> = Vec::new();
            while let Some(doc) = category_cursor.next().await {
                match doc {
                    Ok(doc) => {
                        if let Some(story_id) = doc.get_str("story_id").ok() {
                            filtered_story_ids.push(story_id.to_string());
                        }
                    }
                    Err(_err) => (),
                }
            }
            story_ids = filtered_story_ids;
        }

        // Step 3: Apply the size limit and query stories in bulk
        let size = query_string
            .get("size")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(story_ids.len() as i64);

        let limited_story_ids = story_ids.iter().take(size as usize).cloned().collect::<Vec<String>>();

        let story_filter = doc! { "story_id": { "$in": limited_story_ids } };
        let mut story_cursor = db
            .collection(stories_collection_name.as_str())
            .find(story_filter, None)
            .await
            .unwrap();

        let mut list_document: Vec<Stories> = Vec::new();
        let cdn_path = _config.get_config_with_key("CDN_PATH");

        // Create an instance of StoriesRepository to use its methods
        let stories_repository = StoriesRepository {
            connection: self.connection.clone(),
        };

        while let Some(story_doc) = story_cursor.next().await {
            match story_doc {
                Ok(story) => {
                    let mut story = stories_repository.doc_to_story(&story, &cdn_path).await;
                    story.last_chapter = stories_repository
                        .get_last_chapter(&story.story_id, story.total_chapters)
                        .await;
                    list_document.push(story);
                }
                Err(_err) => (),
            }
        }

        // Update total_document to reflect the number of stories returned
        let total_document = list_document.len() as i64;

        Ok(ListStoriesResponse {
            message: String::from("Successfully"),
            error: false,
            data: ListStories {
                list: list_document,
                total: total_document,
                total_page: 1, // We don't have paging, so we default to 1 page.
            },
        })
    }
}