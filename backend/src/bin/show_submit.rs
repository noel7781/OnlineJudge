#[macro_use]
use diesel;
use backend;

use self::backend::*;
use self::diesel::prelude::*;
use self::models::*;
fn main() {
    use backend::schema::submits::dsl::*;

    let connection = establish_connection();
    let results = submits
        // .filter(id.eq(2))
        .load::<Submit>(&connection)
        .expect("Error loading ANSWER");

    println!("Displaying {} results", results.len());
    for ans in results {
        println!("Correct?(0 is correct) {}", ans.result);
        println!("Submit time: {}", ans.submit_at.unwrap());
        println!("----------\n");
    }
}
