use directories::ProjectDirs;

use crate::{args::Args, config::Config, database::Database};

#[derive(Debug)]
pub struct Application {
    config: Config,
    args: Args,
    database: Database,
    project_dir: ProjectDirs,
}

impl Application {
    pub async fn new(_args: Args) -> Application {
        todo!();
    }

    pub async fn run() {
        todo!();
    }
}
