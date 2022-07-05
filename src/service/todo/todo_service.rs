use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::tik::custom_tik_models::TodoAdd;
use crate::model::diesel::tik::tik_models::Todo;
use crate::model::request::todo::add_todo_request::AddTodoRequest;
use crate::utils::database::get_connection;

pub fn todo_create(request: &Json<AddTodoRequest>, login_user_info: LoginUserInfo) -> Result<Todo, String> {
    use crate::model::diesel::tik::tik_schema::todo as todo_table;
    let bill_book_role_add = TodoAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        name: request.name.to_string(),
        tags: "".to_string(),
        user_id: login_user_info.userId,
        is_complete: 0,
        priority: 0,
        schedule_time: 0,
        description: None
    };
    let inserted_result = diesel::insert_into(todo_table::table)
        .values(&bill_book_role_add)
        .get_result::<Todo>(&get_connection());
    return Ok(inserted_result.unwrap());
}


