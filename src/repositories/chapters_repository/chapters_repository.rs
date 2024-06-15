use crate::config::{Config, IConfig};
use crate::models::response::Response;
use crate::models::chapters::chapters::Chapters;
use crate::utils::helpers::{get_next_increment_id, generate_url_key};

use mongodb::bson::doc;
use mongodb::Client;

pub struct ChaptersRepository {
    pub connection: Client,
}

impl ChaptersRepository {
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
}