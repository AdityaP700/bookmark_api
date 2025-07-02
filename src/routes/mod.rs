use crate::models::bookmark::{Bookmark, NewBookmark};
use actix_web::{HttpResponse, Responder, delete, get, post, web};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[post("/bookmarks")]
async fn create_bookmark(
    bookmark_data: web::Json<NewBookmark>,
    store: web::Data<Mutex<HashMap<Uuid, Bookmark>>>,
) -> impl Responder {
    //creating a new Bookmark instance from the input NewBookmark
    let new_data = bookmark_data.into_inner();

    //Generate UUID and timestamp
    let new_bookmark = Bookmark {
        id: Uuid::new_v4(),
        url: new_data.url,
        title: new_data.title,
        description: new_data.description,
        created_at: Utc::now(),
    };
    //Removed unwrap as we aren't Option anyone
    println!(
        "Received new bookmark: URL:{},Title:{:?},Description{:?},ID: {}, Created At: {}",
        new_bookmark.url,
        new_bookmark.title,
        new_bookmark.description,
        new_bookmark.id,
        new_bookmark.created_at,
    );
    store
        .lock()
        .unwrap()
        .insert(new_bookmark.id, new_bookmark.clone());
    HttpResponse::Ok().body(format!(
        "Bookmark '{}' added successfully!",
        //toh since the title is an option so we made it unwrap
        new_bookmark.title.unwrap_or("Untitled".to_string())
    ))
}

#[get("/bookmarks")]
async fn get_bookmarks(store: web::Data<Mutex<HashMap<Uuid, Bookmark>>>) -> impl Responder {
    let bookmarks = store.lock().unwrap();
    let bookmarks_list: Vec<Bookmark> = bookmarks.values().cloned().collect();

    if bookmarks_list.is_empty() {
        HttpResponse::NotFound().body("No bookmarks found.")
    } else {
        HttpResponse::Ok().json(bookmarks_list)
    }
}

#[get("/bookmarks/{id}")]
async fn get_bookmark_by_id(
    id: web::Path<Uuid>,
    store: web::Data<Mutex<HashMap<Uuid, Bookmark>>>,
) -> impl Responder {
    let id = id.into_inner();
    let bookmarks = store.lock().unwrap();

    match bookmarks.get(&id) {
        Some(bookmark) => HttpResponse::Ok().json(bookmark.clone()),
        None => HttpResponse::NotFound().body("Bookmark not found."),
    }
}

#[delete("/bookmarks/{id}")]
async fn delete_bookmark(
    id: web::Path<Uuid>,
    store: web::Data<Mutex<HashMap<Uuid, Bookmark>>>,
) -> impl Responder {
    let id = id.into_inner();
    let mut bookmarks = store.lock().unwrap();
    if bookmarks.remove(&id).is_some() {
        HttpResponse::Ok().body(format!("Bookmark with ID {} deleted successfully", id))
    } else {
        HttpResponse::NotFound().body(format!("Bookmark with ID {} not found", id))
    }
}
