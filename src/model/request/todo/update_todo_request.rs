use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, JsonSchema,FromForm, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct UpdateTodoRequest {
    pub id: i64,
    pub is_complete: Option<i32>,
    pub name: Option<String>,
}
