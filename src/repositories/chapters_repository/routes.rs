use crate::db::db::Connection;
use crate::models::chapters::chapters::Chapters;
use crate::repositories::chapters_repository::ChaptersRepository;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

#[post("/create")]
async fn create(chapter: web::Json<Chapters>) -> HttpResponse {
    let _connection_client = Connection::init().await.unwrap();
    let _repository: ChaptersRepository = ChaptersRepository {
        connection: _connection_client,
    };
    HttpResponse::Ok().json(_repository.create(chapter.into_inner()).await)
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
}
