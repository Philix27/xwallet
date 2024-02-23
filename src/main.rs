mod config;
mod integration;
mod models;
mod modules;
mod schema;

use crate::modules::{app::AppState, wallet::WalletRoutes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = "9000";

    config::logger::init();
    // let pool = get_db_pool();
    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);
    AppState::run_server(port)
}
