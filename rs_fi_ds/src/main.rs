mod bintree;
mod bond;
mod data_loader;
mod interest_rate_swap;
mod pandl;
mod rates;
mod tbills;
mod restful_service;
use restful_service::task::*;
use crate::bond::bond::DiscountFactor;
use crate::bond::bond::discount_factor;
use crate::bond::bond::Periodicity;
use crate::data_loader::data_loader::load_market_data;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_discount_factor)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
