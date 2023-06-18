use std::{env, sync::Arc};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    database_url: Arc<str>,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").unwrap().into(),
        }
    }
}

impl<'a> Config {
    pub fn new() -> Config {
        todo!();
    }

    pub fn get_database_url(&'a self) -> &Arc<str> {
        &self.database_url
    }
}
