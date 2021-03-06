use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, JsonSchema, Serialize)]
#[allow(non_snake_case)]
pub struct AddTaskRequest {
    pub name: String,
    pub parent: i64,
    pub schedule_time: Option<i64>,
    pub description: Option<String>,
    pub priority: Option<i32>,
}
