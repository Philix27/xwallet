mod routes;

use actix_web::{middleware::Logger, web, App, Handler, HttpServer, Responder};

use routes::wallet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(wallet::routes_handler())
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
