mod infrastructure;
mod domain;
mod application;
use infrastructure::{config::Settings, tracing::init_tracing};
use clap::Parser;

#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
pub struct Args{
   #[arg(short = 'f')]
   folder_path: String,

   #[arg(short = 'm')]
   mode: String
}


#[tokio::main]
async fn main() {
   init_tracing();

   let args = Args::parse();
   let settings = Settings::new(&args.folder_path, &args.mode).unwrap();

   let app = infrastructure::App::new(settings);
   app.run().await;
}
