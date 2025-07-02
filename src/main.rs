mod routes;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use crate::routes::{delete_bookmark, get_bookmark_by_id, get_bookmarks, create_bookmark};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::bookmark::Bookmark;
pub mod models;
#[actix_web::main]

 async fn main() {
    let store: web::Data<Mutex<_>> = web::Data::new(Mutex::new(HashMap::<Uuid, Bookmark>::new()));

    HttpServer::new(move||{
        App::new()
        .app_data(store.clone())
        .service(create_bookmark)
        .service(get_bookmarks)
            .service(get_bookmark_by_id)
            .service(delete_bookmark)

    })
    .bind("127.0.0.1:8080").unwrap()
    .run()
    .await
    .expect("Failed to run the server");
}
