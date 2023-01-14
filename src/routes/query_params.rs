use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

pub async fn    queryparams(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: i32,
}
