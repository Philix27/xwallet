mod config;
mod integration;
mod models;
mod modules;
mod schema;

use crate::{config::AppEnv, modules::app::AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env");
    let port = &AppEnv::new().port[..];

    let _ = config::logger::init();
    // let pool = get_db_pool();
    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);
    AppState::run_server(port).await
}
