use actix_cors::Cors;
use actix_web::{get, post, http, App, HttpServer, HttpRequest};

#[get("/")]
async fn index(_req: HttpRequest) -> String {
    "[GET] Hello World!".to_owned()
}

#[post("/")]
async fn index_post(_req: HttpRequest) -> String {
    "[POST] Hello World!".to_owned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allowed_origin("https://127.0.0.1:3000")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
            .wrap(cors)
            .service(index)
            .service(index_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}