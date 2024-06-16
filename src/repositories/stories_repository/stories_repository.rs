use crate::config::{Config, IConfig};
use crate::models::response::{ Response };
use crate::models::stories::stories::{ Stories, ListStories };
use crate::models::stories::payload::{ PostStoryCategoryMapping };
use crate::models::stories::response::{ ListStoriesResponse };
use crate::utils::helpers::{get_next_increment_id, generate_url_key};

extern crate serde_json;

use std::collections::HashMap;

use futures::stream::StreamExt;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::FindOptions;
use mongodb::Client;

pub struct StoriesRepository {
    pub connection: Client,
}

impl StoriesRepository {

    /**
     * Get list documents
     */
    pub async fn get_list(&self, query_string: HashMap<String, String>) -> Result<ListStoriesResponse, Error> {
            let _config: Config = Config {};
            let database_name = _config.get_config_with_key("DATABASE_NAME");
            let collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
            let db = self.connection.database(database_name.as_str());

            let condition_query = self.build_condition_query(&query_string);

            // paging
            let condition_query_count = condition_query.clone();
            let total_document = db
                .collection(collection_name.as_str())
                .count_documents(condition_query_count, None)
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

            // query data
            let find_options = FindOptions::builder().skip(page * size).limit(size).build();
            let mut cursor = db
                .collection(collection_name.as_str())
                .find(condition_query, find_options)
                .await
                .unwrap();

            let mut list_document: Vec<Stories> = Vec::new();
            while let Some(doc) = cursor.next().await {
                        match doc {
                            Ok(doc) => {
                                let document = Stories {
                                    story_id: doc.get_str("story_id").unwrap().to_owned(),
                                    increment_id: doc.get_i64("increment_id").unwrap().to_owned(),
                                    title: doc.get_str("title").unwrap().to_owned(),
                                    url_key: doc.get_str("url_key").unwrap().to_owned(),
                                    author: doc.get_str("author").unwrap().to_owned(),
                                    description: doc.get_str("description").unwrap().to_owned(),
                                    publish_date: doc.get_str("publish_date").unwrap().to_owned(),
                                    status: doc.get_str("status").unwrap().to_owned()
                                };
                                list_document.push(document)
                            }
                            Err(_err) => (),
                        }
                    }
            Ok(ListStoriesResponse {
                message: String::from("Successfully"),
                error: false,
                data: ListStories {
                    list: list_document,
                    total: total_document,
                    total_page: total_page,
                }
            })
    }

    /**
     * Create story
     */
    pub async fn create(&self, document: Stories) -> Response {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

//      println!("{:#?}", story);

        let increment_id = get_next_increment_id(&self.connection, collection_name.as_str()).await;
        let url_key = generate_url_key(&document.title);

        let document_id = uuid::Uuid::new_v4().to_string();

                let _ex = db
                    .collection(collection_name.as_str())
                    .insert_one(
                        doc! {
                            "story_id": document_id,
                            "increment_id": increment_id,
                            "title": document.title,
                            "url_key": url_key,
                            "author": document.author,
                            "description": document.description,
                            "publish_date": document.publish_date,
                            "status": document.status
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
                        error: false,
                        message: "Something wrong.".to_string(),
                    },
                }
    }

    /**
     * Get story detail by id
     */
    pub async fn get_detail_by_id(&self, story_id: &str) -> Option<Stories> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "story_id": story_id };
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        result.and_then(|doc| bson::from_document(doc).ok())
    }

    pub async fn assign_categories(&self, mapping: PostStoryCategoryMapping) -> Response {

        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("STORIES_CATEGORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

       // First, delete existing connections for the story_id
        let delete_result = db
            .collection(collection_name.as_str())
            .delete_many(doc! { "story_id": &mapping.story_id }, None)
            .await;

        match delete_result {
            Ok(_) => (),
            Err(_) => {
                return Response {
                    error: true,
                    message: "Failed to delete existing mappings.".to_string(),
                }
            }
        }

        // Prepare new mappings
        let mut new_mappings = Vec::new();
        for category_id in mapping.category_ids.iter() {
            new_mappings.push(doc! {
                "story_id": &mapping.story_id,
                "category_id": category_id,
            });
        }

        // Insert new mappings
        let insert_result = db
            .collection(collection_name.as_str())
            .insert_many(new_mappings, None)
            .await;

        match insert_result {
            Ok(_) => Response {
                error: false,
                message: "Mapping created successfully.".to_string(),
            },
            Err(_) => Response {
                error: true,
                message: "Something went wrong.".to_string(),
            },
        }
    }

    /**
     * Build condition query
     */
    pub fn build_condition_query(&self, query_string: &HashMap<String, String>) -> Document {

        let mut condition_query = Document::new();

        let status: String = query_string
            .get("status")
            .unwrap_or(&String::from(""))
            .to_string();
        if status != "" {
            condition_query.insert("status".to_owned(), status);
        }

        let title: String = query_string
                .get("title")
                .unwrap_or(&String::from(""))
                .to_string();
        if title != "" {
            condition_query.insert("title".to_owned(), title);
        }

        let author: String = query_string
                .get("author")
                .unwrap_or(&String::from(""))
                .to_string();
        if author != "" {
            condition_query.insert("author".to_owned(), author);
        }

        return condition_query;
    }
}