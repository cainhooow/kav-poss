use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn estabilish_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL IS INVALID");

    Database::connect(&database_url)
    .await
    .expect("Failed to connect database")
}