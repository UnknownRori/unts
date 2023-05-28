use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
        }
    }
}

impl<'a> Config {
    pub fn new() -> Config {
        todo!();
    }

    pub fn get_database_url(&'a self) -> &'a str {
        &self.database_url
    }
}
