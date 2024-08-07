use crate::db::db::Connection;
use crate::models::response::Response;
use crate::models::categories::categories::Category;
use crate::repositories::categories_repository::CategoriesRepository;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

use std::collections::HashMap;
use serde_json::json;

#[post("/create")]
async fn create(document: web::Json<Category>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: CategoriesRepository = CategoriesRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(document.into_inner()).await)
}

#[get("/list")]
async fn get_list(_req: HttpRequest) -> HttpResponse {
    let query_string: Vec<&str> = _req.query_string().split('&').collect();
    let mut hash_query_string: HashMap<_, _> = HashMap::new();
    for query_string_item in query_string {
        let hash_query_string_item: Vec<&str> = query_string_item.split('=').collect();
        if hash_query_string_item.len() > 1 {
            hash_query_string.insert(
                hash_query_string_item[0].to_string(),
                hash_query_string_item[1].to_string(),
            );
        }
    }

    let _connection_client = Connection::init().await.unwrap();
    let _category_repository: CategoriesRepository = CategoriesRepository {
        connection: _connection_client,
    };
    match _category_repository.get_list(hash_query_string).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_err) => HttpResponse::Ok().json(Response {
            error: true,
            message: "Invalid request".to_string(),
        }),
    }
}

#[get("/detail/{category_id}")]
async fn get_detail(req: HttpRequest) -> HttpResponse {
    let category_id = req.match_info().get("category_id").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: CategoriesRepository = CategoriesRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_id(category_id).await {
        Some(category) => HttpResponse::Ok().json(category),
        None => HttpResponse::NotFound().body("Category not found"),
    }
}

#[get("/detail_by_url_key/{url_key}")]
async fn get_detail_by_url_key(req: HttpRequest) -> HttpResponse {
    let url_key = req.match_info().get("url_key").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: CategoriesRepository = CategoriesRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_url_key(url_key).await {
        Some(category) => HttpResponse::Ok().json(category),
        None => HttpResponse::NotFound().json(json!({"message": "Category not found"})),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get_list);
    cfg.service(get_detail);
    cfg.service(get_detail_by_url_key);
}
