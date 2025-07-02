mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;

use crate::routes::{
    create_bookmark, delete_bookmark, get_bookmark_by_id, get_bookmarks, update_bookmark,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load the DATABASE_URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Optionally create the table if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id UUID PRIMARY KEY,
            url TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now()
        );"
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(create_bookmark)
            .service(get_bookmarks)
            .service(get_bookmark_by_id)
            .service(delete_bookmark)
            .service(update_bookmark)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
