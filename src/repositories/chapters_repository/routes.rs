use crate::db::db::Connection;
use crate::models::chapters::chapters::{Chapters};
use crate::repositories::chapters_repository::ChaptersRepository;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::json;

use std::collections::HashMap;

#[post("/create")]
async fn create(chapter: web::Json<Chapters>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(chapter.into_inner()).await)
}

#[get("/list/{story_id}")]
async fn get_list_by_story_id(req: HttpRequest) -> HttpResponse {
    let story_id = req.match_info().get("story_id").unwrap();

    let mut query_string: Vec<&str> = req.query_string().split("&").collect();
    let mut hash_query_string: HashMap<_, _> = HashMap::new();
    while let Some(query_string_item) = query_string.pop() {
        let hash_query_string_item: Vec<&str> = query_string_item.split("=").collect();
        if hash_query_string_item.len() > 1 {
            hash_query_string.insert(
                hash_query_string_item[0].to_string(),
                hash_query_string_item[1].to_string(),
            );
        }
    }

    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };

    match _repository.get_list_by_story_id(story_id, hash_query_string).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_err) => HttpResponse::InternalServerError().json(json!({"message": "Failed to get chapters"})),
    }
}

#[get("/detail/{chapter_id}")]
async fn get_detail(req: HttpRequest) -> HttpResponse {
    let chapter_id = req.match_info().get("chapter_id").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_id(chapter_id).await {
        Some(chapter) => HttpResponse::Ok().json(chapter),
        None => HttpResponse::NotFound().json(json!({"message": "Chapter not found"})),
    }
}

#[get("/detail/{story_id}/{ordered}")]
async fn get_detail_by_story_and_ordered(req: HttpRequest) -> HttpResponse {
    let story_id = req.match_info().get("story_id").unwrap();
    let ordered = req.match_info().get("ordered").unwrap().parse::<i64>().unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_story_and_ordered(story_id, ordered).await {
        Some(chapter) => HttpResponse::Ok().json(chapter),
        None => HttpResponse::NotFound().json(json!({"message": "Chapter not found"})),
    }
}

#[get("/detail_by_url/{story_url_key}/{chapter_url_key}")]
async fn get_detail_by_story_and_chapter_url_key(req: HttpRequest) -> HttpResponse {
    let story_url_key = req.match_info().get("story_url_key").unwrap();
    let chapter_url_key = req.match_info().get("chapter_url_key").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_story_and_chapter_url_key(story_url_key, chapter_url_key).await {
        Some(chapter) => HttpResponse::Ok().json(chapter),
        None => HttpResponse::NotFound().json(json!({"message": "Chapter not found"})),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get_detail);
    cfg.service(get_detail_by_story_and_ordered);
    cfg.service(get_detail_by_story_and_chapter_url_key);
    cfg.service(get_list_by_story_id);
}
