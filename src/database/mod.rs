pub mod error;

use std::sync::Arc;

use sqlx::Transaction;
use sqlx::pool::PoolConnection;
use sqlx::MySql;
use sqlx::MySqlPool;

use crate::config::Config;

use self::error::DatabaseError;

#[derive(Debug)]
pub struct Database {
    pool: MySqlPool,
    connection_url: Arc<str>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Database, DatabaseError> {
        let connection_url = Arc::clone(config.get_database_url());

        let pool = sqlx::MySqlPool::connect(&connection_url)
            .await
            .map_err(|_| DatabaseError::new((*connection_url).to_owned()))?;

        Ok(Database {
            pool,
            connection_url,
        })
    }

    pub async fn get_conn(&self) -> Result<PoolConnection<MySql>, DatabaseError> {
        self.pool
            .acquire()
            .await
            .map_err(|_| DatabaseError::new((*self.connection_url).to_owned()))
    }

    pub async fn get_transaction(&self) -> Result<Transaction<MySql>, DatabaseError> {
        self.pool
            .begin()
            .await
            .map_err(|_| DatabaseError::new((*self.connection_url).to_owned()))
    }
}
