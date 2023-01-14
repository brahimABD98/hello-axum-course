use axum::Json;
use serde::Deserialize;

pub async fn validate_data(Json(user): Json<RequestUser>) {
    dbg!(user);
}

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}
