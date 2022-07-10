use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{ box_type_rest_response, map_entity };

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::task::add_task_request::AddTaskRequest;
use crate::model::request::task::query_task_request::QueryTaskRequest;
use crate::model::request::todo::del_task_request::DelTaskRequest;
use crate::model::request::todo::probe_todo_request::ProbeTodoRequest;
use crate::model::request::todo::update_todo_request::UpdateTodoRequest;
use crate::model::response::todo::todo_response::TodoResponse;
use crate::service::todo::task_service::{del_task, probe_todo, query_task, task_create, update_task};

pub fn get_routes_and_docs(_settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![list, add, probe, del, update]
}

/// # 查询待办事项列表
///
/// 返回待办事项列表
#[openapi(tag = "待办事项")]
#[get("/v1/list?<query..>")]
pub fn list(query: QueryTaskRequest, login_user_info: LoginUserInfo) -> Json<ApiResponse<Vec<TodoResponse>>> {
    let todo_list = query_task(query, login_user_info);
    let todo_resp = map_entity(todo_list);
    let boxed_response = box_type_rest_response(todo_resp);
    return Json::from(boxed_response);
}

/// # 新增待办事项
///
/// 新增待办事项
#[openapi(tag = "待办事项")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddTaskRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoResponse>> {
    let todo_result = task_create(&request, login_user_info);
    return match todo_result {
        Ok(v) => {
            let todo_res = TodoResponse::from(&v);
            Json::from(box_type_rest_response(todo_res))
        },
        Err(_e) => {
            Json::from(box_type_rest_response(Default::default()))
        }
    }
}

/// # 删除待办事项
///
/// 删除待办事项
#[openapi(tag = "待办事项")]
#[delete("/v1/del",data = "<request>")]
pub fn del(request: Json<DelTaskRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<String>> {
    del_task(&request, login_user_info).expect("TODO: panic message");
    return Json::from(box_type_rest_response("ok".parse().unwrap()));

}

/// # 更新待办事项
///
/// 更新待办事项
#[openapi(tag = "待办事项")]
#[patch("/v1/update",data = "<request>")]
pub fn update(request: Json<UpdateTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoResponse>> {
    let updated_todo = update_task(&request, login_user_info);
    let todo_response = TodoResponse::from(&updated_todo);
    return Json::from(box_type_rest_response(todo_response));

}

/// # 探测变更
///
/// 探测待办事项是否变更
#[openapi(tag = "待办事项")]
#[post("/v1/probe",data = "<_request>")]
pub async fn probe(_request: Json<ProbeTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<bool>> {
    let todo_result = probe_todo(login_user_info).await;
    return match todo_result {
        Ok(v) => {
            Json::from(box_type_rest_response(v))
        },
        Err(_e) => {
            Json::from(box_type_rest_response(false))
        }
    }
}