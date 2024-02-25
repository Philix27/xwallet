use super::{api_docs::ApiDocs, graphql::AppGraphql};
use crate::modules::{
    AuthRoutes, ComplianceRoutes, FxRoutes, PaymentRoutes, TransactionsRoutes, UserRoutes,
    WalletRoutes,
};
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState;

impl AppState {
    pub async fn run_server(port: &str) -> Result<(), std::io::Error> {
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                // Loggers
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec!["POST", "GET"])
                        .supports_credentials()
                        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                        .allowed_header(header::CONTENT_TYPE)
                        .max_age(3000),
                )
                // Handlers
                .service(WalletRoutes::routes_handler())
                .service(AuthRoutes::routes_handler())
                .service(TransactionsRoutes::routes_handler())
                .service(ComplianceRoutes::routes_handler())
                .service(PaymentRoutes::routes_handler())
                .service(UserRoutes::routes_handler())
                .service(FxRoutes::routes_handler())
                // Graphql
                .service(
                    web::resource("/graphql")
                        .route(web::post().to(AppGraphql::graphiql))
                        .route(web::get().to(AppGraphql::graphiql)),
                )
                .service(web::resource("/playground").route(web::get().to(AppGraphql::playground)))
                .service(web::resource("/graphiql").route(web::get().to(AppGraphql::graphiql)))
                // Swagger
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}"),
                    // .url("/api-docs/openapi.json", ApiDocs::openapi()),
                )
                .default_service(web::to(AppGraphql::homepage)) 
        })
        .workers(2)
        .bind(("127.0.0.1", port.parse().unwrap()))?
        .run()
        .await
    }
}
