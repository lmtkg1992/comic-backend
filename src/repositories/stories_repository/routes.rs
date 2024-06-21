use crate::db::db::Connection;
use crate::models::response::{Response};
use crate::models::stories::stories::{Stories};
use crate::models::stories::payload::{PostStoryCategoryMapping};

use crate::repositories::stories_repository::StoriesRepository;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse};

use std::collections::HashMap;

#[post("/create")]
async fn create(document: web::Json<Stories>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: StoriesRepository = StoriesRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(document.into_inner()).await)
}

#[get("/list")]
async fn get_list(_req: HttpRequest) -> HttpResponse {
    let mut query_string: Vec<&str> = _req.query_string().split("&").collect();
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
    let _story_repository: StoriesRepository = StoriesRepository {
        connection: _connection_client,
    };
    match _story_repository.get_list(hash_query_string).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_err) => HttpResponse::Ok().json(Response {
            error: true,
            message: "Invalid Token".to_string(),
        }),
    }
}

#[get("/detail/{story_id}")]
async fn get_detail(req: HttpRequest) -> HttpResponse {
    let story_id = req.match_info().get("story_id").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: StoriesRepository = StoriesRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_id(story_id).await {
        Some(story) => HttpResponse::Ok().json(story),
        None => HttpResponse::NotFound().body("Story not found"),
    }
}

#[post("/assign_categories")]
async fn assign_categories(mapping: web::Json<PostStoryCategoryMapping>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _story_repository: StoriesRepository = StoriesRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_story_repository.assign_categories(mapping.into_inner()).await)
}

#[put("/update_path_image/{story_id}")]
async fn update_path_image(req: HttpRequest, new_path: web::Json<String>) -> HttpResponse {
    let story_id = req.match_info().get("story_id").unwrap().to_string();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: StoriesRepository = StoriesRepository {
        connection: _connection_client,
    };

    match _repository.update_path_image(&story_id, &new_path).await {
        Ok(_) => HttpResponse::Ok().body("Path image updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update path image"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get_list);
    cfg.service(get_detail);
    cfg.service(assign_categories);
    cfg.service(update_path_image);
}
