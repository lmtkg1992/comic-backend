use crate::config::{Config, IConfig};
use crate::models::categories::categories::{Category, ListCategories};
use crate::models::categories::response::{ListCategoriesResponse};
use crate::models::response::Response;
use crate::utils::helpers::{get_next_increment_id, generate_url_key};

use futures::stream::StreamExt;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::FindOptions;
use mongodb::Client;
use std::collections::HashMap;

pub struct CategoriesRepository {
    pub connection: Client,
}

impl CategoriesRepository {
    pub async fn get_list(&self, query_string: HashMap<String, String>) -> Result<ListCategoriesResponse, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CATEGORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let condition_query = self.build_condition_query(&query_string);

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

        let find_options = FindOptions::builder().skip(page * size).limit(size).build();
        let mut cursor = db
            .collection(collection_name.as_str())
            .find(condition_query, find_options)
            .await
            .unwrap();

        let mut list_document: Vec<Category> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let document = Category {
                        category_id: doc.get_str("category_id").unwrap().to_owned(),
                        increment_id: doc.get_i64("increment_id").unwrap().to_owned(),
                        title: doc.get_str("title").unwrap().to_owned(),
                        url_key: doc.get_str("url_key").unwrap().to_owned(),
                        type_category: doc.get_str("type_category").unwrap().to_owned(),
                        priority: doc.get_i64("priority").unwrap().to_owned(),
                    };
                    list_document.push(document)
                }
                Err(_err) => (),
            }
        }
        Ok(ListCategoriesResponse {
            message: String::from("Successfully"),
            error: false,
            data: ListCategories {
              list: list_document,
              total: total_document,
              total_page: total_page,
          }
        })
    }

    pub async fn create(&self, document: Category) -> Response {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("CATEGORIES_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let document_id = uuid::Uuid::new_v4().to_string();

        let increment_id = get_next_increment_id(&self.connection, collection_name.as_str()).await;
        let url_key = generate_url_key(&document.title);

        let _ex = db
            .collection(collection_name.as_str())
            .insert_one(
                doc! {
                    "category_id": document_id,
                    "increment_id" : increment_id,
                    "title": document.title,
                    "url_key": url_key,
                    "priority": document.priority,
                    "type_category": document.type_category,
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

    fn build_condition_query(&self, query_string: &HashMap<String, String>) -> Document {
        let mut condition_query = Document::new();

        let type_category: String = query_string
            .get("type_category")
            .unwrap_or(&String::from(""))
            .to_string();
        if type_category != "" {
            condition_query.insert("type_category".to_owned(), type_category);
        }

        let name: String = query_string
            .get("title")
            .unwrap_or(&String::from(""))
            .to_string();
        if name != "" {
            condition_query.insert("name".to_owned(), name);
        }

        condition_query
    }
}
