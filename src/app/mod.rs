use directories::ProjectDirs;

use crate::{args::Args, config::Config, database::Database, utility::project_dir};

#[derive(Debug)]
pub struct Application {
    config: Config,
    args: Args,
    database: Option<Database>,
    project_dir: ProjectDirs,
}

impl Application {
    /// Create new instance of [`Application`] it will panic if cannot find
    /// directory to store it's data and failed to connect to database
    pub async fn new(args: Args) -> Application {
        let config = Config::new(&args);
        let project_dir = project_dir().expect("(!) Cannot find suitable directory to store");
        let mut database = None;
        
        if !args.offline {
            database = Some(Database::new(&config).await.expect("(!) Failed to connect database"));
        }

        Application {
            config,
            project_dir,
            database,
            args,
        }
    }

    pub async fn run() {
        todo!();
    }
}
