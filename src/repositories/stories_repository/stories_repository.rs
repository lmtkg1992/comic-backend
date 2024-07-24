use crate::config::{Config, IConfig};
use crate::models::response::{ Response };
use crate::models::stories::stories::{ Stories, ListStories };
use crate::models::stories::payload::{ PostStoryCategoryMapping };
use crate::models::stories::response::{ ListStoriesResponse };
use crate::models::authors::authors::{ Authors, AuthorStory};
use crate::models::categories::categories::{ CategoryStory};

use crate::utils::helpers::{get_next_increment_id, generate_url_key};
use crate::repositories::authors_repository::AuthorsRepository;

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
            let cdn_path = _config.get_config_with_key("CDN_PATH");

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
                None => 15,//default size
            };

            let mut total_page = total_document / size;
            if total_document % size > 0 {
                total_page = total_page + 1;
            }


            // Determine sort order
            let sort_by_latest: String = query_string
                .get("sort_by_latest")
                .unwrap_or(&String::from(""))
                .to_string();
            let sort_order = if sort_by_latest == "true" {
                doc! { "updated_date": -1 } // descending order
            } else {
                doc! {} // no sorting or apply default sorting logic
            };

            // query data
            let find_options = FindOptions::builder().skip(page * size).limit(size).sort(sort_order).build();
            let mut cursor = db
                .collection(collection_name.as_str())
                .find(condition_query, find_options)
                .await
                .unwrap();

            let mut list_document: Vec<Stories> = Vec::new();
            while let Some(doc) = cursor.next().await {
                        match doc {
                            Ok(doc) => {
                                let document = self.doc_to_story(&doc, &cdn_path).await;
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

        // Check if url_key already exists
        let existing_story_key = db
            .collection(collection_name.as_str())
            .find_one(doc! { "url_key": &url_key }, None)
            .await
            .unwrap();

        if existing_story_key.is_some() {
            return Response {
                error: true,
                message: "URL key already exists.".to_string(),
            };
        }

        // Check if author title exists, if not, create new author, otherwise, get the author_id
        let authors_collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let existing_author = db
            .collection(authors_collection_name.as_str())
            .find_one(doc! { "title": &document.author.author_title }, None)
            .await
            .unwrap();

        let author_id = match existing_author {
            Some(author_doc) => author_doc.get_str("author_id").unwrap().to_owned(),
            None => {
                // Create new author and get the author_id
                let author_repository = AuthorsRepository {
                    connection: self.connection.clone(),
                };
                let new_author = Authors {
                    author_id: "auto".to_string(), // This will be generated by the repository
                    title: document.author.author_title.clone(),
                    url_key: "".to_string(), // This will be generated by the repository
                    created_date: document.publish_date.clone(),
                    updated_date: document.updated_date.clone(),
                };
                match author_repository.create(new_author).await {
                    Response { error: false, message: _ } => {
                        // Retrieve the newly created author_id
                        let created_author = db
                            .collection(authors_collection_name.as_str())
                            .find_one(doc! { "title": &document.author.author_title }, None)
                            .await
                            .unwrap()
                            .unwrap();
                        created_author.get_str("author_id").unwrap().to_owned()
                    },
                    Response { error: true, message } => {
                        return Response {
                            error: true,
                            message: format!("Failed to create new author: {}", message),
                        }
                    }
                }
            }
        };

        let document_id = uuid::Uuid::new_v4().to_string();

                let _ex = db
                    .collection(collection_name.as_str())
                    .insert_one(
                        doc! {
                            "story_id": document_id,
                            "increment_id": increment_id,
                            "title": document.title,
                            "url_key": url_key,
                            "is_active" : document.is_active,
                            "path_image": document.path_image,
                            "author_id": author_id,
                            "description": document.description,
                            "publish_date": document.publish_date,
                            "updated_date" : document.updated_date,
                            "status": document.status,
                            "is_full": document.is_full,
                            "is_hot": document.is_hot,
                            "total_chapters": document.total_chapters,
                            "source": document.source,
                            "translator": document.translator
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
        let cdn_path = _config.get_config_with_key("CDN_PATH");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "story_id": story_id , "is_active" : bson::Bson::Boolean(true)};
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        match result {
            Some(story) => Some(self.doc_to_story(&story, &cdn_path).await),
            None => None,
        }
    }

    /**
     * Assign categories
     */
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
     * Get list stories by category_id
     */
    pub async fn get_list_by_category_id(&self, category_id: &str, query_string: HashMap<String, String>) -> Result<ListStoriesResponse, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let stories_collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
        let mapping_collection_name = _config.get_config_with_key("STORIES_CATEGORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        // Find story_ids by category_id
        let filter = doc! { "category_id": category_id };
        let mut cursor = db.collection(mapping_collection_name.as_str()).find(filter, None).await?;
        let mut story_ids = Vec::new();
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

        // If no stories found, return empty list
        if story_ids.is_empty() {
            return Ok(ListStoriesResponse {
                message: String::from("Successfully"),
                error: false,
                data: ListStories {
                    list: vec![],
                    total: 0,
                    total_page: 0,
                }
            });
        }

        let mut condition_query = self.build_condition_query(&query_string);
        condition_query.insert("story_id".to_owned(), doc! { "$in": story_ids });

        // paging
        let condition_query_count = condition_query.clone();
        let total_document = db
            .collection(stories_collection_name.as_str())
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
            None => 15,//default size
        };
        let mut total_page = total_document / size;
        if total_document % size > 0 {
            total_page = total_page + 1;
        }

        // query data
        let find_options = FindOptions::builder().skip(page * size).limit(size).build();
        let mut cursor = db
            .collection(stories_collection_name.as_str())
            .find(condition_query, find_options)
            .await
            .unwrap();

        let cdn_path = _config.get_config_with_key("CDN_PATH");
        let mut list_document: Vec<Stories> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let document = self.doc_to_story(&doc, &cdn_path).await;
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
     * Update path image
     */
    pub async fn update_path_image(&self, story_id: &str, new_path_image: &str) -> Result<(), Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("STORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        db.collection(collection_name.as_str())
            .update_one(
                doc! { "story_id": story_id },
                doc! { "$set": { "path_image": new_path_image } },
                None,
            )
            .await?;

        Ok(())
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
        if !title.is_empty() {
            let regex = format!(".*{}.*", regex::escape(&title));
            condition_query.insert("title".to_owned(), doc! { "$regex": regex, "$options": "i" });
         }

        let is_active: String = query_string
                .get("is_active")
                .unwrap_or(&String::from(""))
                .to_string();
        if is_active == "true" {
            condition_query.insert("is_active".to_owned(), bson::Bson::Boolean(true));
        }

        let author_id: String = query_string
                .get("author_id")
                .unwrap_or(&String::from(""))
                .to_string();
        if author_id != "" {
            condition_query.insert("author_id".to_owned(), author_id);
        }

        let differ_story_id: String = query_string
                .get("differ_story_id")
                .unwrap_or(&String::from(""))
                .to_string();
        if !differ_story_id.is_empty() {
            condition_query.insert("story_id".to_owned(), doc! { "$ne": differ_story_id });
        }

        let is_full: String = query_string
            .get("is_full")
            .unwrap_or(&String::from(""))
            .to_string();
        if is_full == "true" {
            condition_query.insert("is_full".to_owned(), bson::Bson::Boolean(true));
        } else if is_full == "false" {
            condition_query.insert("is_full".to_owned(), bson::Bson::Boolean(false));
        }

        let is_hot: String = query_string
            .get("is_hot")
            .unwrap_or(&String::from(""))
            .to_string();
        if is_hot == "true" {
            condition_query.insert("is_hot".to_owned(), bson::Bson::Boolean(true));
        } else if is_hot == "false" {
            condition_query.insert("is_hot".to_owned(), bson::Bson::Boolean(false));
        }

        return condition_query;
    }

    /**
     * Convert document to story
     */
    async fn doc_to_story(&self, doc: &Document, cdn_path: &str) -> Stories {
        let _config: Config = Config {};
        let authors_collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let categories_collection_name = _config.get_config_with_key("CATEGORIES_COLLECTION_NAME");
        let mapping_collection_name = _config.get_config_with_key("STORIES_CATEGORIES_COLLECTION_NAME");

        let db = self.connection.database(_config.get_config_with_key("DATABASE_NAME").as_str());

        let author_id = doc.get_str("author_id").unwrap().to_owned();
        let author_doc = db.collection(authors_collection_name.as_str())
            .find_one(doc! { "author_id": &author_id }, None)
            .await
            .unwrap();

        let author = match author_doc {
            Some(author) => AuthorStory {
                author_id: author.get_str("author_id").unwrap().to_owned(),
                author_title: author.get_str("title").unwrap().to_owned(),
                url_key: author.get_str("url_key").unwrap().to_owned()
            },
            None => AuthorStory {
                author_id: "".to_string(),
                author_title: "".to_string(),
                url_key: "".to_string()
            },
        };

        // Fetch category mappings
        let mut categories: Vec<CategoryStory> = Vec::new();
        let mut cursor = db.collection(mapping_collection_name.as_str())
            .find(doc! { "story_id": doc.get_str("story_id").unwrap().to_owned() }, None)
            .await
            .unwrap();

        while let Some(mapping_doc) = cursor.next().await {
            if let Ok(mapping) = mapping_doc {
                if let Ok(category_id) = mapping.get_str("category_id") {
                    let category_doc = db.collection(categories_collection_name.as_str())
                        .find_one(doc! { "category_id": category_id, "type_category": "category" }, None)
                        .await
                        .unwrap();

                    if let Some(category) = category_doc {
                        categories.push(CategoryStory {
                            category_id: category.get_str("category_id").unwrap().to_owned(),
                            category_name: category.get_str("title").unwrap().to_owned(),
                            url_key: category.get_str("url_key").unwrap().to_owned()
                        });
                    }
                }
            }
        }

        Stories {
            story_id: doc.get_str("story_id").unwrap().to_owned(),
            increment_id: doc.get_i64("increment_id").unwrap().to_owned(),
            title: doc.get_str("title").unwrap().to_owned(),
            url_key: doc.get_str("url_key").unwrap().to_owned(),
            is_active: doc.get_bool("is_active").unwrap().to_owned(),
            path_image: format!("{}{}", cdn_path, doc.get_str("path_image").unwrap().to_owned()),
            author,
            description: doc.get_str("description").unwrap().to_owned(),
            publish_date: doc.get_str("publish_date").unwrap().to_owned(),
            updated_date: doc.get_str("updated_date").unwrap().to_owned(),
            status: doc.get_str("status").unwrap().to_owned(),
            is_full: doc.get_bool("is_full").unwrap().to_owned(),
            is_hot: doc.get_bool("is_hot").unwrap().to_owned(),
            total_chapters: doc.get_i64("total_chapters").unwrap().to_owned(),
            source: doc.get_str("source").unwrap().to_owned(),
            translator: doc.get_str("translator").unwrap().to_owned(),
            categories
        }
    }
}