use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{http::StatusCode, Extension, Json};
use data::database::users::Entity as Users;
use data::sea_orm::IntoActiveModel;
use data::{
    database::users,
    sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    Json(request_user): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("ddgvfbgnbtrtyh".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Json(request_user): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = "15255062626262".to_owned();
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let save_user = user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(ResponseUser {
            username: save_user.username.unwrap(),
            id: save_user.id.unwrap(),
            token: save_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
pub async fn logout(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let mut  user= if let Some(user)=Users::find()
    .filter(users::Column::Token.eq(Some(authorization.token())))
    .one(&database)
    .await
    .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user.into_active_model()
    }
    else{
        return  Err(StatusCode::UNAUTHORIZED);
    };

    user.token=Set(None);
    user.save(&database).await.map_err(	|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    todo!()
}
