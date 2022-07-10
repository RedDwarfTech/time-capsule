use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, JsonSchema,FromForm, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UpdateTodoRequest {
    pub id: i64,
    pub is_complete: Option<i32>,
    pub complete_time: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>
}
