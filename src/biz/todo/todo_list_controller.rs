use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{ box_type_rest_response, map_entity };

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::todo::add_task_request::AddTaskRequest;
use crate::model::request::todo::del_task_request::DelTaskRequest;
use crate::model::request::todo::update_todo_list_request::UpdateTodoListRequest;
use crate::model::response::todo::todo_list_response::TodoListResponse;
use crate::service::todo::todo_list_service::{todo_list_create, query_todo_list, update_todo_list, del_todo_list};

pub fn get_routes_and_docs(_settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![list, add, del, update]
}

/// # 查询清单列表
///
/// 返回清单列表
#[openapi(tag = "清单")]
#[get("/v1/list")]
pub fn list(login_user_info: LoginUserInfo) -> Json<ApiResponse<Vec<TodoListResponse>>> {
    let todo_list = query_todo_list(login_user_info);
    let todo_resp = map_entity(todo_list);
    let boxed_response = box_type_rest_response(todo_resp);
    return Json::from(boxed_response);
}

/// # 新增清单
///
/// 新增清单
#[openapi(tag = "清单")]
#[post("/v1/add",data = "<request>")]
pub fn add(request: Json<AddTaskRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoListResponse>> {
    let todo_result = todo_list_create(&request, login_user_info);
    return match todo_result {
        Ok(v) => {
            let todo_res = TodoListResponse::from(&v);
            Json::from(box_type_rest_response(todo_res))
        },
        Err(_e) => {
            Json::from(box_type_rest_response(Default::default()))
        }
    }
}

/// # 更新清单
///
/// 更新清单
#[openapi(tag = "清单")]
#[patch("/v1/update",data = "<request>")]
pub fn update(request: Json<UpdateTodoListRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoListResponse>> {
    let updated_todo = update_todo_list(&request, login_user_info);
    let todo_response = TodoListResponse::from(&updated_todo);
    return Json::from(box_type_rest_response(todo_response));

}

/// # 删除清单
///
/// 删除清单
#[openapi(tag = "清单")]
#[delete("/v1/del",data = "<request>")]
pub fn del(request: Json<DelTaskRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<String>> {
    del_todo_list(&request, login_user_info).expect("TODO: panic message");
    return Json::from(box_type_rest_response("ok".parse().unwrap()));

}