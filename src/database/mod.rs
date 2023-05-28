pub mod error;

use sqlx::pool::PoolConnection;
use sqlx::MySql;
use sqlx::MySqlPool;

use crate::config::Config;

use self::error::DatabaseError;

pub struct Database {
    pool: MySqlPool,
    conn: PoolConnection<MySql>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Database, DatabaseError> {
        let connection_url = config.get_database_url();

        let pool = sqlx::MySqlPool::connect(connection_url)
            .await
            .map_err(|_| DatabaseError::new(connection_url.to_owned()))?;

        let conn = pool
            .acquire()
            .await
            .map_err(|_| DatabaseError::new(connection_url.to_owned()))?;

        Ok(Database { pool, conn })
    }
}
