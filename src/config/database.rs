use dotenvy::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool}; 

pub fn get_database_url() -> String {
    dotenv().ok(); 
    env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL in the .env file. Did you create it?")
}

pub async fn connect() -> PgPool {
    let database_url = get_database_url();

    // Create a connection pool to your Neon Postgres database
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the Postgres database")
}