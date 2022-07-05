use serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use crate::model::diesel::tik::tik_models::Todo;

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct TodoResponse {
    pub id: i64
}

impl From<&Todo> for TodoResponse {
    fn from(p: &Todo) -> Self {
        Self {
            id: p.id
        }
    }
}

