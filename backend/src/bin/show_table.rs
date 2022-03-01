#[macro_use]
use diesel;
use backend;

use self::backend::*;
use self::diesel::prelude::*;
use self::models::*;
fn main() {
    use backend::schema::problems::dsl::*;

    let connection = establish_connection();
    let results = problems
        // .filter(id.eq(2))
        .load::<Problem>(&connection)
        .expect("Error loading ANSWER");

    println!("Displaying {} results", results.len());
    for ans in results {
        println!("Problem No. {}", ans.id);
        println!("Path: {}", ans.title);
        println!("----------\n");
    }
}
