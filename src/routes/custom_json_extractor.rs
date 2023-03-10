// use axum::{
//     async_trait,
//     body::HttpBody,
//     extract::FromRequest,
//     http::{Request, StatusCode},
//     BoxError, Json, RequestExt, RequestPartsExt,
// };
// use serde::Deserialize;
// use validator::Validate;

// #[derive(Deserialize, Debug, Validate)]
// pub struct RequestUser {
//     pub username: String,
//     pub password: String,
// }

// #[async_trait]
// impl< B> FromRequest< B> for RequestUser
// where
   
//     B: HttpBody + Send + 'static,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = (StatusCode, String);
//     async fn from_request(request: RequestExt<B>) -> Result<Self, Self::Rejection> {
//         let Json(user) = request
//             .extract::<Json<RequestUser>, _>()
//             .await
//             .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;
//         Ok(user)
//     }
// }

// pub async fn custom_json_extractor(user: RequestUser) {
//     dbg!(user);
// }
