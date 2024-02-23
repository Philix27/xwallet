use crate::modules::wallet::WalletRoutes;
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

pub struct AppState;

impl AppState {
    pub async fn run_server(port: &str) -> Result<(), std::io::Error> {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(Cors::permissive())
                .service(WalletRoutes::routes_handler())
                .service(super::graphql::graphql)
                .service(super::graphql::graphql_playground)
        })
        .workers(2)
        .bind(("127.0.0.1", port.parse().unwrap()))?
        .run()
        .await
    }
}
pub async fn index() -> String {
    format!("Welcome to XWallet Sever")
}
