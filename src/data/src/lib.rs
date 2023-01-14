use dotenvy::dotenv;
use sea_orm::Database;

pub async fn run() {
    dotenv().ok();

    let database_uri = dotenvy::var("DATABASE_URL").unwrap();

    let database = Database::connect(database_uri).await;
    match database {
        Ok(_) => {
            println!("Connected to database");
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
        }
    }
}
