#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use self::models::{NewProblem, Problem};
use actix_web::web;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct SubmitInfo {
    problem_id: i32,
    source_code: String,
    language: String,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_problem<'a>(
    conn: &SqliteConnection,
    id: i32,
    title: &'a str,
    accepted_cnt: Option<i32>,
    submit_cnt: Option<i32>,
    description: &'a str,
    input_desc: &'a str,
    output_desc: &'a str,
    difficulty: &'a str,
    time_limit: Option<i32>,
    memory_limit: Option<i32>,
) {
    use schema::problems;
    let new_answer = NewProblem {
        id,
        title,
        accepted_cnt,
        submit_cnt,
        description,
        input_desc,
        output_desc,
        difficulty,
        time_limit,
        memory_limit,
    };

    diesel::insert_into(problems::table)
        .values(&new_answer)
        .execute(conn)
        .expect("Error creating new answer");
}

pub fn retrieve_problem<'a>(conn: &SqliteConnection, pid: i32) -> Problem {
    use schema::problems::dsl::*;

    let results = problems
        .filter(id.eq(pid))
        .load::<Problem>(conn)
        .expect("Error loading Problem");

    results[0].clone()
}

pub fn retrieve_problem_list<'a>(conn: &SqliteConnection) -> Vec<Problem> {
    use schema::problems::dsl::*;

    let results = problems
        .load::<Problem>(conn)
        .expect("Error loading Problem");

    results.clone()
}

pub fn process_submit(req: web::Json<SubmitInfo>) {
    let code = req.source_code.clone();
    let language = req.language.clone();
    let pid = req.problem_id;
    compile_and_run(pid, &code, &language);
}

fn compile_and_run(pid: i32, code: &str, language: &str) {
    match language {
        "c" => {
            compile_run_c(pid, code);
        }
        "c++" => {
            let mut f = File::create("test.cpp").unwrap();
            let mut buf_writer = BufWriter::new(f);
            buf_writer.write_all(code.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            compile_run_cpp(pid)
        }
        "python" => {
            run_python(pid, code);
        }
        _ => println!("Unsupported language"),
    }
}

fn compile_run_c(pid: i32, code: &str) {}

fn compile_run_cpp(pid: i32) {
    // Command::new("ls")
    //     .status()
    //     .expect("ls command failed to start");

    let output = Command::new("g++")
        .arg("test.cpp")
        .arg("-std=c++11")
        .arg("-o")
        .arg("test.out")
        .output()
        .expect("failed to execute process");

    // Command::new("ls")
    //     .status()
    //     .expect("ls command failed to start");
    let output = Command::new("/bin/cat")
        .arg("./problem/1/input/1.in")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();

    let output = Command::new("./test.out")
        .arg("<")
        .stdin(File::open("problem/1/input/1.in").unwrap())
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    let f = File::create(format!("{}.out", pid)).unwrap();
    let mut buf_writer = BufWriter::new(f);
    buf_writer.write_all(&output.stdout).unwrap();
    buf_writer.flush().unwrap();

    let diff = Command::new("diff")
        // .arg("--strip-trailing-cr")
        .arg("-w")
        .arg("-B")
        .arg(format!("{}.out", pid))
        .arg(format!("problem/{}/output/1.out", pid))
        .output()
        .expect("failed to diff process");

    io::stdout().write_all(&diff.stdout).unwrap();
}

fn run_python(pid: i32, code: &str) {}
