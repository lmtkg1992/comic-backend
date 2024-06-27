#[macro_use]
extern crate bson;
extern crate tree_magic;

use actix_cors::Cors;
use actix_web::http::ContentEncoding;
use actix_web::{http, middleware, web, App, HttpServer};

mod config;
mod db;
// mod middlewares;
mod models;
mod repositories;
mod utils;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Started on port 8083");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::scope("/stories").configure(repositories::stories_repository::init_routes))
            .service(web::scope("/categories").configure(repositories::categories_repository::init_routes))
            .service(web::scope("/chapters").configure(repositories::chapters_repository::init_routes))
            .service(web::scope("/authors").configure(repositories::authors_repository::init_routes))
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}