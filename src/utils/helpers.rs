use mongodb::bson::doc;
use mongodb::Client;
use slugify::slugify;
use crate::config::{Config, IConfig};

pub async fn get_next_increment_id(client: &Client, collection_name: &str) -> i64 {
     let _config: Config = Config {};
    let database_name = _config.get_config_with_key("DATABASE_NAME");
    let db = client.database(database_name.as_str());

    let collection_increment_ids = db.collection("increment_ids");

    let filter = doc! { "collection_name": collection_name };
    let update = doc! { "$inc": { "current_id": 1 } };
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    let result = collection_increment_ids
        .find_one_and_update(filter, update, options)
        .await
        .expect("Failed to increment URL increment ID");

    result
        .and_then(|doc| doc.get_i64("current_id").ok())
        .unwrap_or(1) // Default to 1 if no document exists
}

pub fn generate_url_key(title: &str) -> String {
    slugify!(title)
}