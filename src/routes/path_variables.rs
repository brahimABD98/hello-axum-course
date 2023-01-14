use axum::extract::Path;

pub async fn path_variables(Path(id): Path<String>) -> String {
    id
}

pub async fn hard_coded_path() -> String {
    "you got 15".to_owned()
}
