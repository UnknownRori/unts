use crate::database::Database;

#[cfg(test)]
async fn setup_db() -> Database {
    use crate::config::Config;

    let config = Config::default();

    Database::new(&config).await.unwrap()
}

#[cfg(test)]
#[tokio::test]
async fn test_db_create_instance() {
    let db = setup_db().await;

    let _ = db.get_conn().await.unwrap();
}

#[cfg(test)]
#[tokio::test]
async fn test_upsert_1() {
    use chrono::Utc;

    use crate::{model::NotesSqlx, utility::generate_uuid};
    let db = setup_db().await;
    let mut conn = db.get_conn().await.unwrap();

    let date = Utc::now().naive_utc();
    let uuid = generate_uuid().to_string();
    let note = NotesSqlx::new(
        uuid.to_owned(),
        "Hello, world!".to_owned(),
        "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_owned(),
        date
    );

    note.upsert(&mut conn).await.unwrap();
    let note2 = NotesSqlx::find(&uuid, &mut conn).await.unwrap();

    assert_eq!(note.uuid, note2.uuid);
    assert_eq!(note.title, note2.title);
    assert_eq!(note.content, note2.content);
    // assert_eq!(note.created_at, note2.created_at); // Note: Should this be included on the test?
}
