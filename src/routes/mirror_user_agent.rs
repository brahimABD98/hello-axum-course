use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(ua): TypedHeader<UserAgent>) -> String {
    ua.to_string()
}
