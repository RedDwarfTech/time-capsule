// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::tik::tik_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "check_list"]
pub struct CheckList {
    pub id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub name: String,
    pub user_id: i64,
    pub is_complete: i32,
}

#[derive(AsChangeset)]
#[table_name = "todo"]
pub struct TodoUpdate {
    pub is_complete: Option<i32>,
    pub complete_time: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "todo_list"]
pub struct TodoListUpdate {
    pub name: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "todo"]
pub struct TodoAdd {
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub name: String,
    pub tags: String,
    pub user_id: i64,
    pub is_complete: i32,
    pub priority: i32,
    pub schedule_time: i64,
    pub description: Option<String>,
    pub parent: i64,
    pub complete_time: Option<i64>
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "todo_list"]
pub struct TodoListAdd {
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub name: String,
    pub user_id: i64,
    pub parent_id: i64,
    pub list_type: i32,
    pub color: i32,
    pub node_type: i32,
}