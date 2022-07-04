use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_type_rest_response};

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![list]
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
