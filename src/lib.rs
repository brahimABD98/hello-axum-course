use data::run;
use routes::create_routes;
mod routes;

pub async fn runserver() {
    let database = run().await.unwrap();
    let app = create_routes(database);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
