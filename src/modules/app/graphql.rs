use crate::{modules::wallet::WalletRoutes, schema::Schema};
use actix_cors::Cors;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};


#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(user)
}
