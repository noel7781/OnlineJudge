#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub accepted_cnt: Option<i32>,
    pub submit_cnt: Option<i32>,
    pub description: String,
    pub input_desc: String,
    pub output_desc: String,
    pub difficulty: String,
    pub time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
}

use serde::{Deserialize, Serialize};

use super::schema::problems;
#[derive(Insertable)]
#[table_name = "problems"]
pub struct NewProblem<'a> {
    pub id: i32,
    pub title: &'a str,
    pub accepted_cnt: Option<i32>,
    pub submit_cnt: Option<i32>,
    pub description: &'a str,
    pub input_desc: &'a str,
    pub output_desc: &'a str,
    pub difficulty: &'a str,
    pub time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
}
