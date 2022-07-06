use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_rest_response, box_type_rest_response};

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::todo::add_todo_request::AddTodoRequest;
use crate::model::request::todo::probe_todo_request::ProbeTodoRequest;
use crate::model::response::todo::todo_response::TodoResponse;
use crate::service::todo::todo_service::{probe_todo, todo_create};

pub fn get_routes_and_docs(_settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![list, add]
}

/// # 查询待办事项列表
///
/// 返回待办事项列表
#[openapi(tag = "待办事项")]
#[get("/v1/list")]
pub fn list() -> Json<ApiResponse<&'static str>> {
    let boxed_response = box_type_rest_response("contents");
    return Json::from(boxed_response);
}

/// # 新增待办事项
///
/// 新增待办事项
#[openapi(tag = "新增待办事项")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoResponse>> {
    let todo_result = todo_create(&request, login_user_info);
    return match todo_result {
        Ok(v) => {
            let todo_res = TodoResponse::from(&v);
            Json::from(box_type_rest_response(todo_res))
        },
        Err(e) => {
            Json::from(box_type_rest_response(Default::default()))
        }
    }
}

/// # 探测变更
///
/// 探测待办事项是否变更
#[openapi(tag = "探测变更")]
#[post("/v1/add",data = "<request>")]
pub async fn probe(request: Json<ProbeTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<bool>> {
    let todo_result = probe_todo(&request, login_user_info).await;
    return match todo_result {
        Ok(v) => {
            Json::from(box_type_rest_response(v))
        },
        Err(e) => {
            Json::from(box_type_rest_response(false))
        }
    }
}