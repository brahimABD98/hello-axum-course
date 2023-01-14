use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "This message was sent from the server".to_string(),
    })
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}
