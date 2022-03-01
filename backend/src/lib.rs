#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use self::models::{NewProblem, Problem};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

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
