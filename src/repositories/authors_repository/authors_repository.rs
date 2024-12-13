use crate::config::{Config, IConfig};
use crate::models::authors::authors::{Authors, ListAuthors};
use crate::models::authors::response::{ListAuthorsResponse};
use crate::models::response::{Response};
use mongodb::bson::doc;
use mongodb::Client;
use uuid::Uuid;
use slugify::slugify;
use mongodb::options::FindOptions;
use mongodb::error::Error;
use futures::StreamExt;

pub struct AuthorsRepository {
    pub connection: Client,
}

impl AuthorsRepository {

    /**
     * Create new author
     */
    pub async fn create(&self, author: Authors) -> Response {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let author_id = Uuid::new_v4().to_string();
        let url_key = slugify!(&author.title); // Generate url_key using slugify
        let _ex = db
            .collection(collection_name.as_str())
            .insert_one(
                doc! {
                    "author_id": author_id,
                    "title": author.title,
                    "url_key": url_key,
                    "description": author.description,
                    "created_date": author.created_date,
                    "updated_date": author.updated_date,
                },
                None,
            )
            .await;

        match _ex {
            Ok(_) => Response {
                message: "Create document successful.".to_string(),
                error: false
            },
            Err(_) => Response {
                message: "Something went wrong.".to_string(),
                error: true
            },
        }
    }

    /**
     * Get list of authors
     */
    pub async fn get_list(&self) -> Result<ListAuthorsResponse, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let total_document = db
            .collection(collection_name.as_str())
            .count_documents(None, None)
            .await
            .unwrap();

        let page = 0;
        let size = 1000;
        let mut total_page = total_document / size;
        if total_document % size > 0 {
            total_page = total_page + 1;
        }

        let find_options = FindOptions::builder()
            .skip(page * size)
            .limit(size)
            .build();

        let mut cursor = db
            .collection(collection_name.as_str())
            .find(None, find_options)
            .await
            .unwrap();

        let mut list_document: Vec<Authors> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let document = Authors {
                        author_id: doc.get_str("author_id").unwrap().to_owned(),
                        title: doc.get_str("title").unwrap().to_owned(),
                        url_key: doc.get_str("url_key").unwrap().to_owned(),
                        description: doc.get_str("description").unwrap().to_owned(),
                        created_date: doc.get_str("created_date").unwrap().to_owned(),
                        updated_date: doc.get_str("updated_date").unwrap().to_owned(),
                    };
                    list_document.push(document)
                }
                Err(_err) => (),
            }
        }
        Ok(ListAuthorsResponse {
            message: String::from("Successfully"),
            error: false,
            data: ListAuthors {
                list: list_document,
                total: total_document,
                total_page: total_page,
            }
        })
    }
    /**
     * Get detail author by author_id
     */
    pub async fn get_detail_by_id(&self, author_id: &str) -> Option<Authors> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "author_id": author_id };
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        result.and_then(|doc| bson::from_document(doc).ok())
    }

    /**
     * Get author detail by url key
     */
    pub async fn get_detail_by_url_key(&self, url_key: &str) -> Option<Authors> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("AUTHORS_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());

        let filter = doc! { "url_key": url_key };
        let result = db
            .collection(collection_name.as_str())
            .find_one(filter, None)
            .await
            .ok()
            .flatten();

        result.and_then(|doc| bson::from_document(doc).ok())
    }
}