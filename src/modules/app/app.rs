use crate::modules::wallet::WalletRoutes;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

use super::graphql::AppGraphql;

pub struct AppState;

impl AppState {
    pub async fn run_server(port: &str) -> Result<(), std::io::Error> {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec!["POST", "GET"])
                        .supports_credentials()
                        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                        .allowed_header(header::CONTENT_TYPE)
                        .max_age(3000),
                )
                .service(WalletRoutes::routes_handler())
                .service(
                    web::resource("/graphql")
                        .route(web::post().to(AppGraphql::graphiql))
                        .route(web::get().to(AppGraphql::graphiql)),
                )
                .service(web::resource("/playground").route(web::get().to(AppGraphql::playground)))
                .service(web::resource("/graphiql").route(web::get().to(AppGraphql::graphiql)))
                .default_service(web::to(AppGraphql::homepage))
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
