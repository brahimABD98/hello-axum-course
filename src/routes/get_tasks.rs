use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Path, Extension};
use data::database::tasks::{self, Entity as Tasks};
use data::sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    prio: Option<String>,
    desc: Option<String>,
}
pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id).one(&database).await.unwrap();
    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            prio: task.priority,
            desc: task.description,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_param): Query<GetTasksQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut prio_filter = Condition::all();
    if let Some(priority) = query_param.priority {
        prio_filter = if priority.is_empty() {
            prio_filter.add(tasks::Column::Priority.is_null())
        } else {
            prio_filter.add(tasks::Column::Priority.eq(priority))
        }
    }
    let tasks = Tasks::find()
        .filter(prio_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            prio: db_task.priority,
            desc: db_task.description,
        })
        .collect();

    Ok(Json(tasks))
}
#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}
