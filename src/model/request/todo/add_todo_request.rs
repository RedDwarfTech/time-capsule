use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, JsonSchema, Serialize)]
#[allow(non_snake_case)]
pub struct AddTodoRequest {
    pub name: String
}
