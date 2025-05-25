use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};
use std::env;

pub async fn connect_db()->PgPool{
    let db_url=env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to database")
}