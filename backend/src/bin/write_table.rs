extern crate backend;
extern crate diesel;

use self::backend::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is problem Number?");
    let mut problem_id = String::new();
    stdin().read_line(&mut problem_id).unwrap();
    let problem_id = problem_id.trim().parse::<i32>().unwrap();
    println!("What is the path of the answer?");
    let mut filepath = String::new();
    stdin().read_line(&mut filepath).unwrap();

    // create_problem(&connection, problem_id, &filepath);
}

// #[cfg(not(windows))]
// const EOF: &'static str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &'static str = "CTRL+Z";
