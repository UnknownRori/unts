use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    database_url: Option<String>,

    #[arg(short, long)]
    online: bool
}
