pub mod database;
use dotenvy::dotenv;
pub use sea_orm;
use sea_orm::{Database, DatabaseConnection, DbErr};
pub async fn run() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    //this project is mess i can't be bothered to deal with dotenvy for not working after refactoring the run function
    let database_uri = "postgres://postgres:keyoarbcat@localhost:5432/postgres".to_owned();

    let database = Database::connect(database_uri).await;

    database
}
