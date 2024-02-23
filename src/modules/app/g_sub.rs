//! This example demonstrates asynchronous subscriptions with [`actix_web`].

use std::{env, pin::Pin, time::Duration};

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use juniper::{
    graphql_subscription, graphql_value, EmptyMutation, FieldError, GraphQLObject, RootNode,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler, subscriptions};
use sqlx::Database;

type Schema = RootNode<'static, Query, EmptyMutation<Database>, Subscription>;

fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Database>::new(), Subscription)
}

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", Some("/subscriptions")).await
}

async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", Some("/subscriptions")).await
}

async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

async fn homepage() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("content-type", "text/html"))
        .message_body(
            "<html><h1>juniper_actix/subscription example</h1>\
                   <div>visit <a href=\"/graphiql\">GraphiQL</a></div>\
                   <div>visit <a href=\"/playground\">GraphQL Playground</a></div>\
             </html>",
        )
}

#[derive(GraphQLObject)]
struct RandomHuman {
    id: String,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/subscriptions").route(web::get().to(subscriptions)))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .default_service(web::to(homepage))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
