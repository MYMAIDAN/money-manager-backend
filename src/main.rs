mod infrastructure;
mod domain;
mod application;
mod adapters;
use infrastructure::tracing::init_tracing;


#[tokio::main]
async fn main() {
   init_tracing();    

   let app = infrastructure::App::new();
   app.run().await;
}
