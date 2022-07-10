use diesel::{RunQueryDsl, QueryDsl, BoolExpressionMethods, QueryResult};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::tik::custom_tik_models::{TodoListAdd, TodoListUpdate};
use crate::model::diesel::tik::tik_models::{ TodoList};
use crate::model::request::todo::add_todo_request::{ AddTodoRequest};
use crate::model::request::todo::del_todo_list_request::DelTodoListRequest;
use crate::utils::database::get_connection;
use crate::diesel::ExpressionMethods;
use crate::model::request::todo::update_todo_list_request::UpdateTodoListRequest;

pub fn todo_list_create(request: &Json<AddTodoRequest>, login_user_info: LoginUserInfo) -> Result<TodoList, String> {
    use crate::model::diesel::tik::tik_schema::todo_list as todo_list_table;
    let bill_book_role_add = TodoListAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        name: request.name.to_string(),
        user_id: login_user_info.userId,
        parent_id: request.parent_id,
        list_type: 0,
        color: 0,
        node_type: 0
    };
    let inserted_result = diesel::insert_into(todo_list_table::table)
        .values(&bill_book_role_add)
        .get_result::<TodoList>(&get_connection());
    return Ok(inserted_result.unwrap());
}

pub fn query_todo_list(login_user_info: LoginUserInfo) -> Vec<TodoList> {
    use crate::model::diesel::tik::tik_schema::todo_list as todo_table;
    let predicate = todo_table::dsl::user_id.eq(login_user_info.userId);
    let results = todo_table::table.filter(predicate)
        .load::<TodoList>(&get_connection())
        .expect("Error loading todo list");
    return results;
}

pub fn del_todo_list(request: &Json<DelTodoListRequest>, login_user_info: LoginUserInfo) -> QueryResult<usize> {
    use crate::model::diesel::tik::tik_schema::todo_list as todo_list_table;
    // https://github.com/diesel-rs/diesel/issues/1369
    let predicate = todo_list_table::dsl::id.eq(request.id).and(todo_list_table::dsl::user_id.eq(login_user_info.userId));
    let delete_result = diesel::delete(todo_list_table::table.filter(predicate)).execute(&get_connection());
    return delete_result;
}

pub fn update_todo_list(request: &Json<UpdateTodoListRequest>, login_user_info: LoginUserInfo) -> TodoList {
    use crate::model::diesel::tik::tik_schema::todo_list as todo_list_table;
    let predicate = todo_list_table::dsl::id.eq(request.id).and(todo_list_table::dsl::user_id.eq(login_user_info.userId));
    let update_result = diesel::update(todo_list_table::table.filter(predicate))
        .set(&TodoListUpdate{
            name: request.name.to_string()
        })
        .get_result::<TodoList>(&get_connection());
    return update_result.unwrap();
}

