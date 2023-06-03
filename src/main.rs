use clap::Parser;
use unts::app::Application;
use unts::args::Args;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    let _ = Application::new(args).await;

    Ok(())
}
