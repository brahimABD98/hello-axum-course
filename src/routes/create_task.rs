use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{Extension, Json, TypedHeader};
use data::database::tasks;
use data::database::users::{self, Entity as Users};
use data::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<(), StatusCode> {
    let token = authorization.token();
    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    Ok(())
}
