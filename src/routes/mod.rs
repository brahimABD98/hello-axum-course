use axum::{
    body::Body,
    http::Method,
    routing::{get, post},
    Extension, Router,
};
mod always_errors;
mod create_task;
mod custom_json_extractor;
mod get_json;
mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod returns_201;
mod validate_data;
use always_errors::always_error;
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use data::run;
use data::sea_orm::DatabaseConnection;
use get_json::get_json;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::{hard_coded_path, path_variables};
use query_params::queryparams;
use returns_201::returns_201;
use tower_http::cors::{Any, CorsLayer};
use validate_data::validate_data;
pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(hello_world::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(queryparams))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/always_errors", get(always_error))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
        .layer(cors)
}
