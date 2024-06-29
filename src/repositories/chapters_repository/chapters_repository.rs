use crate::config::{Config, IConfig};
use crate::models::response::Response;
use crate::models::chapters::chapters::{Chapters};
use crate::models::chapters::response::{ ChaptersByStoryId, ListChaptersByStoryId, ListChaptersResponse };

use futures::stream::StreamExt;
use mongodb::bson::doc;
use mongodb::error::Error;
use mongodb::Client;

use std::collections::HashMap;

use crate::utils::helpers::{get_next_increment_id, generate_url_key};

pub struct ChaptersRepository {
    pub connection: Client,
}

impl ChaptersRepository {

    /**
     * Create a new chapter
     */
    pub async fn create(&self, chapter: Chapters) -> Response {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CHAPTERS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let chapter_id = uuid::Uuid::new_v4().to_string();

        // Increment the URL increment ID
        let increment_id = get_next_increment_id(&self.connection, collection_name.as_str()).await;
        let url_key = generate_url_key(&chapter.title);

        let _ex = db
            .collection(collection_name.as_str())
            .insert_one(
                doc! {
                    "chapter_id": chapter_id,
                    "story_id": chapter.story_id,
                    "increment_id": increment_id,
                    "title": chapter.title,
                    "url_key": url_key,
                    "content": chapter.content,
                    "ordered": chapter.ordered,
                    "status": chapter.status,
                    "created_date": chapter.created_date,
                    "updated_date": chapter.updated_date,
                },
                None,
            )
            .await;

        match _ex {
            Ok(_) => Response {
                error: false,
                message: "Create document successful.".to_string(),
            },
            Err(_) => Response {
                error: true,
                message: "Something went wrong.".to_string(),
            },
        }
    }

    /**
     * Get chapter detail by ID
     */
    pub async fn get_detail_by_id(&self, chapter_id: &str) -> Option<Chapters> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CHAPTERS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "chapter_id": chapter_id };
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        result.and_then(|doc| bson::from_document(doc).ok())
    }

    /**
     * Get chapter detail by story ID and ordered
     */
    pub async fn get_detail_by_story_and_ordered(&self, story_id: &str, ordered: i64) -> Option<Chapters> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CHAPTERS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "story_id": story_id, "ordered": ordered };
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        result.and_then(|doc| bson::from_document(doc).ok())
    }

    /**
     * Get list of chapters by story ID
     */
    pub async fn get_list_by_story_id(&self, story_id: &str,query_string: HashMap<String, String>) -> Result<ListChaptersResponse, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CHAPTERS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "story_id": story_id };

        // Get total documents count for pagination
        let total_document = db
            .collection(collection_name.as_str())
            .count_documents(filter.clone(), None)
            .await
            .unwrap();

        let mut page = match query_string.get("page") {
            Some(value) => value.parse::<i64>().unwrap(),
            None => 1,
        };
        page = page - 1;
        let size = match query_string.get("size") {
            Some(value) => value.parse::<i64>().unwrap(),
            None => total_document,
        };
        let mut total_page = total_document / size;
        if total_document % size > 0 {
            total_page = total_page + 1;
        }

        let find_options = mongodb::options::FindOptions::builder()
            .skip(page * size)
            .limit(size)
            .build();

        let mut cursor = db
            .collection(collection_name.as_str())
            .find(filter, find_options)
            .await
            .unwrap();

        let mut list_document: Vec<ChaptersByStoryId> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let document = ChaptersByStoryId {
                        chapter_id: doc.get_str("chapter_id").unwrap().to_owned(),
                        story_id: doc.get_str("story_id").unwrap().to_owned(),
                        increment_id: doc.get_i64("increment_id").unwrap(),
                        title: doc.get_str("title").unwrap().to_owned(),
                        url_key: doc.get_str("url_key").unwrap().to_owned(),
                        ordered: doc.get_i64("ordered").unwrap(),
                        status: doc.get_str("status").unwrap().to_owned(),
                        created_date: doc.get_str("created_date").unwrap().to_owned(),
                        updated_date: doc.get_str("updated_date").unwrap().to_owned(),
                    };
                    list_document.push(document)
                }
                Err(_err) => (),
            }
        }

        Ok(ListChaptersResponse {
            message: String::from("Successfully"),
            error: false,
            data: ListChaptersByStoryId {
                list: list_document,
                total: total_document,
                total_page: total_page,
            }
        })
    }
}