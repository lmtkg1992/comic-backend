use crate::db::db::Connection;
use crate::models::authors::authors::Authors;
use crate::repositories::authors_repository::AuthorsRepository;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::json;

#[post("/create")]
async fn create(author: web::Json<Authors>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: AuthorsRepository = AuthorsRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(author.into_inner()).await)
}

#[get("/detail/{author_id}")]
async fn get_detail(req: HttpRequest) -> HttpResponse {
    let author_id = req.match_info().get("author_id").unwrap();
    let _connection_client = Connection::init().await.unwrap();
    let _repository: AuthorsRepository = AuthorsRepository {
        connection: _connection_client,
    };

    match _repository.get_detail_by_id(author_id).await {
        Some(author) => HttpResponse::Ok().json(author),
        None => HttpResponse::NotFound().json(json!({"message": "Author not found"}))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get_detail);
}