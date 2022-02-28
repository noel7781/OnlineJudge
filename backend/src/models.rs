#[derive(Queryable, Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub filepath: String,
}

use super::schema::answer;
#[derive(Insertable)]
#[table_name = "answer"]
pub struct NewAnswer<'a> {
    pub id: i32,
    pub filepath: &'a str,
}
