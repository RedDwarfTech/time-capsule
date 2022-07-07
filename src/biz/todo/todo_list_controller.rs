use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{ box_type_rest_response, map_entity };

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::todo::add_todo_request::AddTodoRequest;
use crate::model::request::todo::del_todo_request::DelTodoRequest;
use crate::model::response::todo::todo_list_response::TodoListResponse;
use crate::service::todo::todo_list_service::{todo_list_create, query_todo_list};
use crate::service::todo::todo_service::{del_todo_list};

pub fn get_routes_and_docs(_settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![list, add, del]
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
pub fn add(request: Json<AddTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<TodoListResponse>> {
    let todo_result = todo_list_create(&request, login_user_info);
    return match todo_result {
        Ok(v) => {
            let todo_res = TodoListResponse::from(&v);
            Json::from(box_type_rest_response(todo_res))
        },
        Err(e) => {
            Json::from(box_type_rest_response(Default::default()))
        }
    }
}

/// # 删除清单
///
/// 删除清单
#[openapi(tag = "清单")]
#[delete("/v1/del",data = "<request>")]
pub fn del(request: Json<DelTodoRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<String>> {
    del_todo_list(&request, login_user_info).expect("TODO: panic message");
    return Json::from(box_type_rest_response("ok".parse().unwrap()));

}