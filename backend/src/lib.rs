#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use self::models::{NewProblem, Problem};
use actix_web::dev::MessageBody;
use actix_web::web;
use diesel::prelude::*;
use dotenv::dotenv;
use glob::glob;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct SubmitInfo {
    problem_id: i32,
    source_code: String,
    language: String,
}

enum GradeResult {
    Success,
    Failure,
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

pub fn process_submit(req: web::Json<SubmitInfo>) -> bool {
    let code = req.source_code.clone();
    let language = req.language.clone();
    let pid = req.problem_id;
    let grade_result = compile_and_run(pid, &code, &language);
    match grade_result {
        GradeResult::Success => true,
        GradeResult::Failure => false,
    }
}

fn compile_and_run(pid: i32, code: &str, language: &str) -> GradeResult {
    match language {
        "c" => compile_run_c(pid, code),
        "c++" => {
            let mut f = File::create("test.cpp").unwrap();
            let mut buf_writer = BufWriter::new(f);
            buf_writer.write_all(code.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            compile_run_cpp(pid)
        }
        "python" => run_python(pid, code),
        _ => GradeResult::Failure,
    }
}

fn compile_run_c(pid: i32, code: &str) -> GradeResult {
    todo!()
}

fn compile_run_cpp(pid: i32) -> GradeResult {
    // Command::new("ls")
    //     .status()
    //     .expect("ls command failed to start");

    let mut result = true;

    Command::new("g++")
        .arg("test.cpp")
        .arg("-std=c++11")
        .arg("-o")
        .arg("test.out")
        .output()
        .expect("failed to execute process");

    // Command::new("ls")
    //     .status()
    //     .expect("ls command failed to start");
    // let output = Command::new("/bin/cat")
    //     .arg("./problem/1/input/1.in")
    //     .output()
    //     .expect("failed to execute process");
    // io::stdout().write_all(&output.stdout).unwrap();

    for entry in glob(format!("problem/{}/input/*.in", pid).as_str()).unwrap() {
        let entry = entry.unwrap();
        let file_stem = entry.file_stem().unwrap().to_string_lossy();
        println!("{}", file_stem);

        let output = Command::new("./test.out")
            .stdin(File::open(format!("problem/{}/input/{}.in", pid, file_stem)).unwrap())
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
        let f = File::create(format!("{}.out", file_stem)).unwrap();
        let mut buf_writer = BufWriter::new(f);
        buf_writer.write_all(&output.stdout).unwrap();
        buf_writer.flush().unwrap();

        let diff = Command::new("diff")
            .arg("-w")
            .arg("-B")
            .arg(format!("{}.out", file_stem))
            .arg(format!("problem/{}/output/{}.out", pid, file_stem))
            .output()
            .expect("failed to diff process");

        if diff.stdout.len() == 0 {
            // println!("Accepted");
        } else {
            result = false;
            break;
            // println!("Wrong Answer");
        }
        // io::stdout().write_all(&diff.stdout).unwrap();
    }

    if result {
        GradeResult::Success
    } else {
        GradeResult::Failure
    }

    // let output = Command::new("./test.out")
    //     .stdin(File::open("problem/1/input/1.in").unwrap())
    //     .output()
    //     .expect("failed to execute process");
    // io::stdout().write_all(&output.stdout).unwrap();
    // let f = File::create(format!("{}.out", pid)).unwrap();
    // let mut buf_writer = BufWriter::new(f);
    // buf_writer.write_all(&output.stdout).unwrap();
    // buf_writer.flush().unwrap();

    // let diff = Command::new("diff")
    //     .arg("-w")
    //     .arg("-B")
    //     .arg(format!("{}.out", pid))
    //     .arg(format!("problem/{}/output/1.out", pid))
    //     .output()
    //     .expect("failed to diff process");

    // io::stdout().write_all(&diff.stdout).unwrap();
}

fn run_python(pid: i32, code: &str) -> GradeResult {
    todo!()
}
