mod bintree;
mod bond;
mod data_loader;
mod interest_rate_swap;
mod pandl;
mod rates;
mod restful_service;
mod tbills;
use actix_web::middleware::Logger;
use actix_web::App;
use actix_web::HttpServer;
use restful_service::task::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger)
          .service(get_discount_factor)
          .service(get_spot_rates)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
