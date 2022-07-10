use serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use crate::model::diesel::tik::tik_models::{TodoList};

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct TodoListResponse {
    pub id: i64,
    pub name: String,
    pub color: i32,
    pub parent_id: i64,
    pub node_type: i32,
    pub is_default: i32,
    pub is_sys: i32,
}

impl From<&TodoList> for TodoListResponse {
    fn from(p: &TodoList) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            color: p.color,
            parent_id: p.parent_id,
            node_type: p.node_type,
            is_default: p.is_default,
            is_sys: p.is_sys
        }
    }
}

