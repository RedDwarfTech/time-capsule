#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};

mod common;
mod biz;
mod service;
mod model;
mod utils;

use rocket_okapi::{mount_endpoints_and_merged_docs, OpenApiError, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;

use common::health_controller;
use biz::todo::task_controller;
use biz::todo::todo_list_controller;

pub type Result<T> = std::result::Result<T, OpenApiError>;


#[rocket::main]
async fn main() {
    // the performance about rocket and Actix
    // https://www.youtube.com/watch?v=GAxxn_oGA0Y
    // https://stackoverflow.com/questions/72540558/what-is-the-difference-about-rocket-launch-and-main-entrypoint
    // https://github.com/GREsau/okapi/issues/99
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../tik/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("TikTik documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    //let custom_route_spec = (vec![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/tik".to_owned(), openapi_settings,
        "/actuator" => health_controller::get_routes_and_docs(&openapi_settings),
        "/task" => task_controller::get_routes_and_docs(&openapi_settings),
        "/list" => todo_list_controller::get_routes_and_docs(&openapi_settings)

    };

    building_rocket
}
