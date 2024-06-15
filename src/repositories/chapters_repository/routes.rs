use crate::db::db::Connection;
use crate::models::chapters::chapters::Chapters;
use crate::repositories::chapters_repository::ChaptersRepository;
use actix_web::{post, web, HttpResponse};

#[post("/create")]
async fn create(chapter: web::Json<Chapters>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(chapter.into_inner()).await)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}
