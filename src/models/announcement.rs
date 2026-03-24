use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Announcement {
    pub post_id: i32,
    pub author_id: Option<i32>,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub event_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnnouncement {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub event_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnnouncement {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub event_date: Option<String>,
}
