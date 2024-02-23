use actix_web::{Error, HttpResponse, Responder};
use juniper_actix::{graphiql_handler, playground_handler};

pub struct AppGraphql;

impl AppGraphql {
    pub async fn playground() -> Result<HttpResponse, Error> {
        playground_handler("/graphql", Some("/subscriptions")).await
    }

    pub async fn graphiql() -> Result<HttpResponse, Error> {
        graphiql_handler("/graphql", Some("/subscriptions")).await
    }

    pub async fn homepage() -> impl Responder {
        HttpResponse::Ok()
            .insert_header(("content-type", "text/html"))
            .message_body(
                "<html><h1>juniper_actix/subscription example</h1>\
                   <div>visit <a href=\"/graphiql\">GraphiQL</a></div>\
                   <div>visit <a href=\"/playground\">GraphQL Playground</a></div>\
             </html>",
            )
    }

    // async fn graphql(
    //     req: HttpRequest,
    //     payload: web::Payload,
    //     schema: Data<Schema>,
    // ) -> Result<HttpResponse, ()> {
    //     // let context = Database::new();
    //     // Ok(HttpResponse::Ok().json(payload))
    //     // graphql_handler(&schema, &context, req, payload).await
    // }
}
