use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DatabaseError {
    database_url: String,
    backtrace: Backtrace,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(!) Cannot create connection to this URL {}\nBacktrace : {}",
            self.database_url, self.backtrace,
        )
    }
}

impl Error for DatabaseError {}

impl DatabaseError {
    pub fn new(database_url: String) -> DatabaseError {
        DatabaseError {
            database_url,
            backtrace: Backtrace::capture(),
        }
    }
}
