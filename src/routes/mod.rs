use actix_web::{web,post,Responder,HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::bookmark::{Bookmark,NewBookmark};

#[post("/bookmarks")]
async fn create_bookmark(
bookmark_data: web::Json<NewBookmark>,
store: web::Data<Mutex<HashMap<Uuid,Bookmark>>>,)->impl Responder{
//creating a new Bookmark instance from the input NewBookmark
let new_data = bookmark_data.into_inner();
   
//Generate UUID and timestamp
let new_bookmark = Bookmark{
id: Uuid::new_v4(),
    url:new_data.url,
    title: new_data.title,
    description:new_data.description,
    created_at:Utc::now(), 
};
//Removed unwrap as we aren't Option anyone
   println!("Received new bookmark: URL:{},Title:{:?},Description{:?},ID: {}, Created At: {}",
   new_bookmark.url,
   new_bookmark.title,
   new_bookmark.description,
   new_bookmark.id,
   new_bookmark.created_at,
);
   store.lock().unwrap().insert(new_bookmark.id, new_bookmark.clone());
   HttpResponse::Ok().body(format!("Bookmark '{}' added successfully!",
   //toh since the title is an option so we made it unwrap 
   new_bookmark.title.unwrap_or("Untitled".to_string())))
     
}