use directories::ProjectDirs;

use crate::config::Config;

#[derive(Debug)]
pub struct Application {
    config: Config,
    project_dir: ProjectDirs,
}

impl Application {
    pub async fn new() -> Application {
        todo!();
    }

    pub async fn run() {
        todo!();
    }
}
