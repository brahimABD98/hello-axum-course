use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_vlaue = headers.get("x-message").unwrap();
    let message = message_vlaue.to_str().unwrap().to_owned();
    message
}
