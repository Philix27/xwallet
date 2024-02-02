mod models;
mod routes;
mod schema;
mod services;

use actix_cors::Cors;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;

use crate::schema::Schema;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use routes::wallet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = "9000";

    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let pool = get_db_pool();
    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(wallet::routes_handler())
            .service(graphql)
            .service(graphql_playground)
    })
    .workers(2)
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(user)
}
