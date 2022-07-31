use diesel::{RunQueryDsl, QueryDsl, BoolExpressionMethods, QueryResult};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::{get_current_millisecond, start_of_today};
use rust_wheel::config::cache::redis_util::get_str_default;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::tik::custom_tik_models::{TodoAdd, TodoUpdate};
use crate::model::diesel::tik::tik_models::{Todo};
use crate::model::request::task::add_task_request::AddTaskRequest;
use crate::model::request::task::query_task_request::QueryTaskRequest;
use crate::utils::database::get_connection;
use crate::diesel::ExpressionMethods;
use crate::model::diesel::tik::tik_schema::todo::{is_complete, schedule_time, user_id};
use crate::model::request::todo::del_task_request::DelTaskRequest;
use crate::model::request::todo::update_todo_request::UpdateTodoRequest;

pub fn task_create(request: &Json<AddTaskRequest>, login_user_info: LoginUserInfo) -> Result<Todo, String> {
    use crate::model::diesel::tik::tik_schema::todo as todo_table;
    let current_time = get_current_millisecond();
    let bill_book_role_add = TodoAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        name: request.name.to_string(),
        tags: "".to_string(),
        user_id: login_user_info.userId,
        is_complete: 0,
        priority: request.priority.unwrap_or(0),
        schedule_time: request.schedule_time.unwrap_or(current_time),
        description: request.description.clone(),
        parent: request.parent,
        complete_time: None,
        original_schedule_time: request.schedule_time.unwrap_or(current_time),
        device_id: Some(login_user_info.deviceId),
    };
    let inserted_result = diesel::insert_into(todo_table::table)
        .values(&bill_book_role_add)
        .get_result::<Todo>(&get_connection());
    return Ok(inserted_result.unwrap());
}

pub fn query_task(request: QueryTaskRequest, login_user_info: LoginUserInfo) -> Vec<Todo> {
    use crate::model::diesel::tik::tik_schema::todo as todo_table;
    let mut query = todo_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(todo_table::dsl::user_id.eq(login_user_info.userId));
    if let Some(parent_req) = &request.parent {
        query = query.filter(todo_table::dsl::parent.eq(parent_req));
    }
    if let Some(start_time_req) = &request.start_time {
        query = query.filter(todo_table::dsl::schedule_time.gt(start_time_req));
    }
    if let Some(end_time_req) = &request.end_time {
        query = query.filter(todo_table::dsl::schedule_time.lt(end_time_req));
    }
    if let Some(is_complete_req) = &request.is_complete {
        query = query.filter(is_complete.eq(is_complete_req));
    }
    query = query.filter(schedule_time.ge(start_of_today()).or(is_complete.eq(0)));
    let results = query
        .load::<Todo>(&get_connection())
        .expect("Error loading tasks");
    return results;
}

pub fn del_task(request: &Json<DelTaskRequest>, login_user_info: LoginUserInfo) -> QueryResult<usize> {
    use crate::model::diesel::tik::tik_schema::todo as todo_table;
    let predicate = todo_table::dsl::id.eq(request.id).and(user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(todo_table::table.filter(predicate)).execute(&get_connection());
    return delete_result;
}

pub fn update_task(request: &Json<UpdateTodoRequest>, login_user_info: LoginUserInfo) -> Todo {
    use crate::model::diesel::tik::tik_schema::todo as todo_list_table;
    let predicate = todo_list_table::dsl::id.eq(request.id).and(user_id.eq(login_user_info.userId));
    let update_result = diesel::update(todo_list_table::table.filter(predicate))
        .set(&TodoUpdate{
            is_complete: request.is_complete,
            complete_time: request.complete_time,
            name: request.name.clone(),
            description: request.description.clone(),
            priority: request.priority.clone(),
        })
        .get_result::<Todo>(&get_connection());
    return update_result.unwrap();
}

pub async fn probe_todo(login_user_info: LoginUserInfo) -> Result<bool, String> {
    let together = format!("{}{}", "tik:biz:user:", login_user_info.userId);
    let cached_user = get_str_default(&*together).await;
    if cached_user.as_ref().unwrap() == "null" || cached_user.as_ref().unwrap().is_empty() {
        Ok(false)
    }else{
        Ok(true)
    }
}
