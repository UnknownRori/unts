#[cfg(test)]
#[tokio::test]
async fn test_db_create_instance() {
    use crate::{config::Config, database::Database};

    let config = Config::default();

    let _ = Database::new(&config).await.unwrap();
}
