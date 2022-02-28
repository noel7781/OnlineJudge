#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use self::models::{Answer, NewAnswer};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_answer<'a>(conn: &SqliteConnection, id: i32, filepath: &'a str) {
    use schema::answer;
    let new_answer = NewAnswer { id, filepath };

    diesel::insert_into(answer::table)
        .values(&new_answer)
        .execute(conn)
        .expect("Error creating new answer");
}

pub fn retrieve_answer<'a>(conn: &SqliteConnection, pid: i32) -> Answer {
    use schema::answer::dsl::*;

    let results = answer
        .filter(id.eq(pid))
        .load::<Answer>(conn)
        .expect("Error loading ANSWER");

    results[0].clone()
}
