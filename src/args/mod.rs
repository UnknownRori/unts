use std::sync::Arc;

use clap::{Parser, Subcommand};

// TODO : Review this Args and it's SubCommands
// It maybe to repetitive or plain stupid

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Database operation
    /// used for custom database url
    #[arg(short, long)]
    pub database_url: Option<Arc<str>>,

    /// should be true if one wanted not to connect to database
    #[arg(short, long, default_value_t = false)]
    pub offline: bool,

    /// force unts to push to database (only work if offline is not defined to true)
    #[arg(long, default_value_t = false)]
    pub force_push: bool,

    /// force unts to pull from database (only work if offline is not defined to true)
    #[arg(long, default_value_t = false)]
    pub force_pull: bool,

    /// Use the default config not the user defined config
    #[arg(long)]
    pub default_config: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {

    /// Configure user defined config
    Config {
        /// Remote database URL
        #[arg(short, long)]
        database_url: Option<Arc<str>>,

        /// Editor that will be open up by default
        #[arg(short, long)]
        editor: Option<Arc<str>>,
    },

    /// Command specific to note operation
    Note {
        /// Note title
        title: Arc<str>,

        /// Use this if one doesn't want to open editor
        #[arg(short, long)]
        content: Option<Arc<str>>,

        /// Specific custom editor
        #[arg(short, long)]
        editor: Option<Arc<str>>,

        /// Delete the note
        #[arg(long, default_value_t = false)]
        delete: bool,

        /// Push the note to remote it will be ignored if unts are force_pull
        #[arg(long, default_value_t = false)]
        push: bool,

        /// Pull the note to remote it will be ignored if unts are force_push
        #[arg(long, default_value_t = false)]
        pull: bool,
    },
}
