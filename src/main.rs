mod infrastructure;
mod domain;
use infrastructure::tracing::init_tracing;
#[tokio::main]
async fn main() {
   init_tracing();
   let app = infrastructure::App::new();
   app.run().await;
}
