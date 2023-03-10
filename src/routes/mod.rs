use axum::{
    body::Body,
    http::Method,
    routing::{delete, get, patch, post, put},
    Extension, Router,
};
mod always_errors;
mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_json;
mod get_tasks;
mod hello_world;
mod manage_user;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod partial_update;
mod path_variables;
mod query_params;
mod returns_201;
mod update_task;
mod validate_data;
use always_errors::always_error;
use create_task::create_task;
use manage_user::create_user;
use manage_user::login;
// use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use partial_update::partial_update_task;

use data::sea_orm::DatabaseConnection;
use get_json::get_json;
use get_tasks::get_all_tasks;
use get_tasks::get_one_task;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::{hard_coded_path, path_variables};
use query_params::queryparams;
use returns_201::returns_201;
use tower_http::cors::{Any, CorsLayer};
use update_task::update_task;
use validate_data::validate_data;
pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
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
        // .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_one_task))
        .route("/tasks/getall", get(get_all_tasks))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/partial_update/:id", patch(partial_update_task))
        .route("/user", post(create_user))
        .route("/tasks/:id", delete(delete_task))
        .route("/user/login", post(login))
        .layer(Extension(database))
        .layer(cors)
}
