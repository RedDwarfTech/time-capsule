use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct QueryTaskRequest {
    pub name: Option<String>,
    pub parent: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub is_complete: Option<i32>
}
