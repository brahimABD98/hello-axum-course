use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use data::database::tasks;
use data::database::tasks::Entity as Tasks;
use data::sea_orm::prelude::DateTimeWithTimeZone;
use data::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}
pub async fn partial_update_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(req_task): Json<ResponseTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(req_task.id),
        priority: Set(req_task.priority),
        title: Set(req_task.title),
        completed_at: Set(req_task.completed_at),
        description: Set(req_task.description),
        deleted_at: Set(req_task.deleted_at),
        user_id: Set(req_task.user_id),
        is_default: Set(req_task.is_default),
    };
    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
