use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Contest {
    pub contest_no: i32,
    pub title: String,
    pub contest_link: String,
    pub contest_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContest {
    pub title: String,
    pub contest_link: String,
    pub contest_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContest {
    pub title: Option<String>,
    pub contest_link: Option<String>,
    pub contest_date: Option<String>,
}
