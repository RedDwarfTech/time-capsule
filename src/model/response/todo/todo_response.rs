use serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use crate::model::diesel::tik::tik_models::Todo;

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct TodoTaskResponse {
    pub id: i64,
    pub name: String,
    pub is_complete: i32,
    pub parent: i64,
    pub description: Option<String>,
    pub priority: i32,
    pub schedule_time: i64,
    pub complete_time: i64
}

impl From<&Todo> for TodoTaskResponse {
    fn from(p: &Todo) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            is_complete: p.is_complete,
            parent: p.parent,
            description: p.description.clone(),
            priority: p.priority,
            schedule_time: p.schedule_time,
            complete_time: p.complete_time.unwrap_or_default(),
        }
    }
}

