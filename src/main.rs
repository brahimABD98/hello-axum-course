use hello_axum::runserver;

#[tokio::main]
async fn main() {
    runserver().await;
}
