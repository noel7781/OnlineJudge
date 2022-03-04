#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use self::models::{NewProblem, NewSubmit, Problem, Submit};
use actix_web::web;
use diesel::prelude::*;
use dotenv::dotenv;
use glob::glob;
use rlimit::Resource;
use rlimit::{self, setrlimit};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct SubmitInfo {
    pub problem_id: i32,
    pub source_code: String,
    pub language: String,
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

pub fn create_submit(
    conn: &SqliteConnection,
    pid: i32,
    uid: Option<i32>,
    result: i32,
    submit_at: &str,
    language: &str,
) {
    use schema::submits;
    let new_submit = NewSubmit {
        pid,
        uid,
        result,
        submit_at,
        language,
    };

    diesel::insert_into(submits::table)
        .values(&new_submit)
        .execute(conn)
        .expect("Error creating new submit");
}

pub fn retrieve_problem<'a>(conn: &SqliteConnection, pid: i32) -> Problem {
    use schema::problems::dsl::*;

    let results = problems
        .filter(id.eq(pid))
        .load::<Problem>(conn)
        .expect("Error loading Problem");

    results[0].clone()
}

pub fn update_problem_submit_cnt<'a>(conn: &SqliteConnection, pid: i32, status: i32) {
    use schema::problems::dsl::*;

    match status {
        0 => {
            diesel::update(problems.filter(id.eq(pid)))
                .set((
                    accepted_cnt.eq(accepted_cnt + 1),
                    submit_cnt.eq(submit_cnt + 1),
                ))
                .execute(conn)
                .expect("Error updating Problem");
        }
        _ => {
            diesel::update(problems.filter(id.eq(pid)))
                .set((submit_cnt.eq(submit_cnt + 1),))
                .execute(conn)
                .expect("Error updating Problem");
        }
    }
}

pub fn retrieve_problem_list<'a>(conn: &SqliteConnection) -> Vec<Problem> {
    use schema::problems::dsl::*;

    let results = problems
        .load::<Problem>(conn)
        .expect("Error loading Problem");

    results.clone()
}

pub fn retrieve_submit_list<'a>(conn: &SqliteConnection) -> Vec<Submit> {
    use schema::submits::dsl::*;

    let mut results = submits.load::<Submit>(conn).expect("Error loading Problem");

    results.reverse();
    results.clone()
}

pub fn process_submit(req: &web::Json<SubmitInfo>) -> i32 {
    let code = req.source_code.clone();
    let language = req.language.clone();
    let pid = req.problem_id;
    let problem = retrieve_problem(&establish_connection(), pid);
    let time_lim = match problem.time_limit {
        Some(x) => x,
        None => 1,
    };
    let mem_lim = match problem.memory_limit {
        Some(x) => x,
        None => 256,
    };
    let grade_result = compile_and_run(pid, &code, &language, time_lim, mem_lim);
    match grade_result {
        GradeResult::Success => 0,
        GradeResult::Failure => 1,
    }
}

fn compile_and_run(
    pid: i32,
    code: &str,
    language: &str,
    time_lim: i32,
    mem_lim: i32,
) -> GradeResult {
    match language {
        "c" => {
            let f = File::create("test.c").unwrap();
            let mut buf_writer = BufWriter::new(f);
            buf_writer.write_all(code.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            compile_run_c(pid, time_lim, mem_lim)
        }
        "c++" => {
            let f = File::create("test.cpp").unwrap();
            let mut buf_writer = BufWriter::new(f);
            buf_writer.write_all(code.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            compile_run_cpp(pid, time_lim, mem_lim)
        }
        "python" => {
            let f = File::create("test.py").unwrap();
            let mut buf_writer = BufWriter::new(f);
            buf_writer.write_all(code.as_bytes()).unwrap();
            buf_writer.flush().unwrap();
            run_python(pid, time_lim, mem_lim)
        }
        _ => GradeResult::Failure,
    }
}

fn compile_run_c(pid: i32, time_lim: i32, mem_lim: i32) -> GradeResult {
    let mut result = true;

    let output = Command::new("gcc")
        .arg("test.c")
        .arg("-std=c11")
        .arg("-o")
        .arg("test.out")
        .output()
        .expect("failed to execute process");

    if output.stderr.len() > 0 {
        result = false;
    } else {
        for entry in glob(format!("problem/{}/input/*.in", pid).as_str()).unwrap() {
            let entry = entry.unwrap();
            let file_stem = entry.file_stem().unwrap().to_string_lossy();

            let soft_time = time_lim as u64;
            let hard_time = time_lim as u64;
            setrlimit(Resource::CPU, soft_time, hard_time).unwrap();
            let soft_mem = mem_lim as u64;
            let hard_mem = mem_lim as u64;
            setrlimit(Resource::MEMLOCK, soft_mem, hard_mem).unwrap();

            let output = Command::new("./test.out")
                .stdin(File::open(format!("problem/{}/input/{}.in", pid, file_stem)).unwrap())
                .output()
                .expect("failed to execute process");

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

            if diff.stdout.len() != 0 {
                result = false;
                break;
            }
        }
    }

    if result {
        GradeResult::Success
    } else {
        GradeResult::Failure
    }
}

fn compile_run_cpp(pid: i32, time_lim: i32, mem_lim: i32) -> GradeResult {
    // Command::new("ls")
    //     .status()
    //     .expect("ls command failed to start");

    let mut result = true;

    let output = Command::new("g++")
        .arg("test.cpp")
        .arg("-std=c++11")
        .arg("-o")
        .arg("test.out")
        .output()
        .expect("failed to execute process");
    // io::stdout().write_all(&output.stderr).unwrap();
    if output.stderr.len() != 0 {
        result = false;
    } else {
        for entry in glob(format!("problem/{}/input/*.in", pid).as_str()).unwrap() {
            let entry = entry.unwrap();
            let file_stem = entry.file_stem().unwrap().to_string_lossy();
            // println!("{}", file_stem);

            let soft_time = time_lim as u64;
            let hard_time = time_lim as u64;
            setrlimit(Resource::CPU, soft_time, hard_time).unwrap();
            let soft_mem = mem_lim as u64;
            let hard_mem = mem_lim as u64;
            setrlimit(Resource::MEMLOCK, soft_mem, hard_mem).unwrap();

            let output = Command::new("./test.out")
                .stdin(File::open(format!("problem/{}/input/{}.in", pid, file_stem)).unwrap())
                .output()
                .expect("failed to execute process");
            // io::stdout().write_all(&output.stdout).unwrap();

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

fn run_python(pid: i32, time_lim: i32, mem_lim: i32) -> GradeResult {
    let mut result = true;

    for entry in glob(format!("problem/{}/input/*.in", pid).as_str()).unwrap() {
        let entry = entry.unwrap();
        let file_stem = entry.file_stem().unwrap().to_string_lossy();

        let soft_time = time_lim as u64;
        let hard_time = time_lim as u64;
        setrlimit(Resource::CPU, soft_time, hard_time).unwrap();
        let soft_mem = mem_lim as u64;
        let hard_mem = mem_lim as u64;
        setrlimit(Resource::MEMLOCK, soft_mem, hard_mem).unwrap();

        let output = Command::new("python3")
            .arg("test.py")
            .stdin(File::open(format!("problem/{}/input/{}.in", pid, file_stem)).unwrap())
            .output()
            .expect("failed to execute process");

        if output.stderr.len() > 0 {
            result = false;
            break;
        }
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

        if diff.stdout.len() != 0 {
            result = false;
            break;
        }
    }

    if result {
        GradeResult::Success
    } else {
        GradeResult::Failure
    }
}
