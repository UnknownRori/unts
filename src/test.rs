#[cfg(test)]
#[allow(unused_variables)]
#[tokio::test]
async fn test_db_create_instance() {
    use crate::{config::Config, database::Database};

    let config = Config::default();

    let db = Database::new(&config).await.unwrap();
}
