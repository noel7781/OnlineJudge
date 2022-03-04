use actix_cors::Cors;
use actix_web::{
    get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use backend::*;
use chrono::Local;

// #[derive(Debug, Deserialize)]
// struct Info {
//     testKey: String,
// }

// #[post("/post_test")]
// async fn index_post(req: web::Json<Info>) -> String {
//     println!("request: {:#?}", req);
//     Ok(web::Json(Info{
//         message: info.message.clone() + "!"
//     }))
//     format!("{}!", req.testKey)
// }

#[get("/problems")]
async fn get_problem_list(_req: HttpRequest) -> Result<impl Responder> {
    let con = establish_connection();
    let problem = retrieve_problem_list(&con);
    Ok(web::Json(problem))
}

#[get("/problem/{id}")]
async fn get_problem(req: HttpRequest) -> Result<impl Responder> {
    let con = establish_connection();
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let problem = retrieve_problem(&con, id);
    Ok(web::Json(problem))
}

#[post("/submit")]
async fn post_submit(req: web::Json<SubmitInfo>) -> String {
    // println!("request: {:#?}", req);
    let is_correct = process_submit(&req);
    let local_time = Local::now();
    let pid = req.problem_id;
    let con = establish_connection();
    create_submit(
        &con,
        pid,
        Some(-1),
        is_correct,
        &local_time.to_rfc3339(),
        &req.language,
    );
    update_problem_submit_cnt(&con, pid, is_correct);
    match is_correct {
        0 => "Success".to_owned(),
        1 => "Failure".to_owned(),
        _ => "Not a valid result".to_owned(),
    }
}
// #[post("/Problem/{id}")]
// async fn get_answer(req: HttpRequest) -> String {
//     let con = establish_connection();
//     let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
//     let answer = retrieve_answer(&con, id);
//     let filepath = answer.filepath.trim();
//     let file = File::open(filepath).unwrap();
//     let mut contents = String::new();
//     let mut buf_reader = BufReader::new(file);
//     buf_reader.read_to_string(&mut contents).unwrap();
//     format!("{}", contents)
// }

#[get("/status")]
async fn get_submit_list(_req: HttpRequest) -> Result<impl Responder> {
    let con = establish_connection();
    let submit_list = retrieve_submit_list(&con);
    Ok(web::Json(submit_list))
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
            .service(get_problem)
            .service(get_problem_list)
            .service(post_submit)
            .service(get_submit_list)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
