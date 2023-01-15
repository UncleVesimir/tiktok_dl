mod api;
use actix_web::{guard, middleware, middleware::Logger, web, web::Data, App, HttpServer};
use api::download::download;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let guard_domain = "localhost:8080";

        let api_scope = web::scope("/api").guard(guard::Header("Host", guard_domain));
        // .service()

        App::new()
            .wrap(logger)
            .wrap(middleware::Compress::default())
            .service(api_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
