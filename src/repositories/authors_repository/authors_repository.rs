use crate::config::{Config, IConfig};
use crate::models::authors::authors::Authors;
use crate::models::response::{Response};
use mongodb::bson::doc;
use mongodb::Client;
use uuid::Uuid;
use slugify::slugify;

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