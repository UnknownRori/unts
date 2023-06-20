use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{pool::PoolConnection, MySql};
use uuid::Uuid;

/// This struct is used when interacting with user and before interacting with Sqlx crates
#[derive(Debug, Hash)]
pub struct Notes {
    uuid: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
}

/// This struct is used when interacting with Sqlx crates
#[derive(Debug, Hash, sqlx::FromRow)]
pub struct NotesSqlx {
    pub uuid: String,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl From<Notes> for NotesSqlx {
    fn from(note: Notes) -> NotesSqlx {
        NotesSqlx {
            uuid: note.uuid.to_string(),
            title: note.title,
            content: note.content,
            created_at: note.created_at.naive_utc(),
        }
    }
}

impl NotesSqlx {
    pub fn new(
        uuid: String,
        title: String,
        content: String,
        created_at: NaiveDateTime,
    ) -> NotesSqlx {
        NotesSqlx {
            uuid,
            title,
            content,
            created_at,
        }
    }

    pub async fn all(conn: &mut PoolConnection<MySql>) -> Result<Vec<NotesSqlx>, sqlx::Error> {
    Ok(sqlx::query_as!(NotesSqlx, "SELECT * FROM notes")
        .fetch_all(conn)
        .await?)
    }

    pub async fn find(
        uuid: &String,
        conn: &mut PoolConnection<MySql>,
    ) -> Result<NotesSqlx, sqlx::Error> {
        sqlx::query_as!(NotesSqlx, "SELECT * FROM notes WHERE uuid=?", uuid)
            .fetch_one(conn)
            .await
    }

    pub async fn upsert(&self, conn: &mut PoolConnection<MySql>) -> Result<(), sqlx::Error> {
        let _ = sqlx::query_as!(
            NotesSqlx,
            "
            INSERT INTO notes (uuid, title, content, created_at) VALUES (?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE uuid=?
        ",
            self.uuid,
            self.title,
            self.content,
            self.created_at,
            self.uuid
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn delete(self, conn: &mut PoolConnection<MySql>) -> Result<(), sqlx::Error> {
        let _ = sqlx::query_as!(NotesSqlx, "DELETE FROM notes WHERE uuid=?", self.uuid)
            .execute(conn)
            .await?;

        Ok(())
    }
}
