use crate::db::db::Connection;
use crate::models::chapters::chapters::{Chapters};
use crate::repositories::chapters_repository::ChaptersRepository;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

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
        Err(_err) => HttpResponse::InternalServerError().body("Failed to get chapters"),
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
        None => HttpResponse::NotFound().body("Chapter not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get_detail);
    cfg.service(get_list_by_story_id);
}
