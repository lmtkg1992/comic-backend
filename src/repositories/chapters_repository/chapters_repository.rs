use crate::config::{Config, IConfig};
use crate::models::response::Response;
use crate::models::chapters::chapters::Chapters;
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

        let _ex = db
            .collection(collection_name.as_str())
            .insert_one(
                doc! {
                    "chapter_id": chapter_id,
                    "story_id": chapter.story_id,
                    "title": chapter.title,
                    "content": chapter.content,
                    "ordered": chapter.ordered,
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
}