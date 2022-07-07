use serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use crate::model::diesel::tik::tik_models::{TodoList};

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct TodoListResponse {
    pub id: i64,
    pub name: String
}

impl From<&TodoList> for TodoListResponse {
    fn from(p: &TodoList) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
        }
    }
}

