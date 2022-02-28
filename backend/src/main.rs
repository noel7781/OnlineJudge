use actix_cors::Cors;
use actix_web::{get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use backend::*;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Info {
    testKey: String,
}

#[get("/")]
async fn index(_req: HttpRequest) -> String {
    "[GET] Hello World!".to_owned()
}

#[post("/post_test")]
async fn index_post(req: web::Json<Info>) -> String {
    println!("request: {:#?}", req);
    // // Ok(web::Json(Info{
    // //     message: info.message.clone() + "!"
    // // }))
    // format!("123")
    format!("{}!", req.testKey)
}

#[post("/Problem/{id}")]
async fn get_answer(req: HttpRequest) -> String {
    let con = establish_connection();
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let answer = retrieve_answer(&con, id);
    let filepath = answer.filepath.trim();
    let file = File::open(filepath).unwrap();
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents).unwrap();
    format!("{}", contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(index)
            .service(index_post)
            .service(get_answer)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
