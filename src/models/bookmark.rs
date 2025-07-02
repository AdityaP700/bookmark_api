use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// What client sends
#[derive(Deserialize)]
pub struct NewBookmark {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

// What we store and return
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Bookmark {
    pub id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
