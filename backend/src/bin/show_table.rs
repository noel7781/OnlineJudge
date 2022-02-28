#[macro_use]
use diesel;
use backend;

use self::backend::*;
use self::diesel::prelude::*;
use self::models::*;
fn main() {
    use backend::schema::answer::dsl::*;

    let connection = establish_connection();
    let results = answer
        // .filter(id.eq(2))
        .load::<Answer>(&connection)
        .expect("Error loading ANSWER");

    println!("Displaying {} results", results.len());
    for ans in results {
        println!("Problem No. {}", ans.id);
        println!("Path: {}", ans.filepath);
        println!("----------\n");
    }
}
