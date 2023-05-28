use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

/// This struct is used when interacting with user and before interacting with Sqlx crates
#[derive(Debug, Hash)]
pub struct Notes {
    uuid: Uuid,
    title: String,
    content: String,
    date: DateTime<Utc>,
}

/// This struct is used when interacting with Sqlx crates
#[derive(Debug, Hash, sqlx::FromRow)]
pub struct NotesSqlx {
    pub uuid: String,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}
