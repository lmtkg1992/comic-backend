use crate::db::db::Connection;
use crate::repositories::top_story_repository::TopStoryRepository;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::json;
use std::collections::HashMap;

#[get("/list_by_period/{period}")]
async fn get_list_by_period(req: HttpRequest) -> HttpResponse {
    let period = req.match_info().get("period").unwrap();

    // Parse the query string into a HashMap
    let mut query_string: Vec<&str> = req.query_string().split("&").collect();
    let mut hash_query_string: HashMap<String, String> = HashMap::new();
    while let Some(query_string_item) = query_string.pop() {
        let hash_query_string_item: Vec<&str> = query_string_item.split("=").collect();
        if hash_query_string_item.len() > 1 {
            hash_query_string.insert(
                hash_query_string_item[0].to_string(),
                hash_query_string_item[1].to_string(),
            );
        }
    }

    // Extract category_id from the hash_query_string if it exists
    let category_id = hash_query_string.get("category_id").cloned();

    let _connection_client = Connection::init().await.unwrap();
    let _repository: TopStoryRepository = TopStoryRepository {
        connection: _connection_client,
    };

    match _repository.get_list_by_period(period, category_id.as_deref(), hash_query_string).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_err) => HttpResponse::InternalServerError().json(json!({"message": "Failed to get top stories"})),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_list_by_period);
}