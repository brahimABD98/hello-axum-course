use axum::Json;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct RandomData {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<RandomData> {
    let data = RandomData {
        message: "Hello World".to_string(),
        count: 42,
        username: "JohnDoe".to_string(),
    };
    Json(data)
}
