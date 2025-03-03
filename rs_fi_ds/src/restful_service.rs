pub mod task {
  use crate::bond::bond::DiscountFactor;
  use crate::bond::bond::discount_factor;
  use crate::bond::bond::Periodicity;
  use crate::data_loader::data_loader::load_market_data;

  use std::fmt::*;
  use log::{info, warn};

  use actix_web:: {
    body::BoxBody,
    get, 
    post,
    put,
    error::ResponseError,
    web::Path, 
    web::Json,
    web::Data,
    HttpResponse,
    HttpRequest,
    http::{header::ContentType, StatusCode},
    Responder,
    Result,
  };
  use serde::{Serialize, Deserialize};



  #[derive(Serialize, Deserialize, Debug, Clone)]
  #[serde(rename_all = "camelCase")]
  pub struct DiscountFactorsResponse {
    pub discount_factors : Vec<DiscountFactor>,
  }

  // Responder
  impl Responder for DiscountFactorsResponse {
      type Body = BoxBody;

      fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
          

          let body = serde_json::to_string(&self.discount_factors).unwrap();

          // Create response and set content type
          HttpResponse::Ok()
              .content_type(ContentType::json())
              .body(body)
      }
  }

  #[get("/discount_factors")]
  pub async fn get_discount_factor() -> Result<impl Responder> {
    let discount_factors = market_data_loader(String::from("./tests/bond_data.csv")).await;
    Ok(DiscountFactorsResponse { discount_factors : discount_factors})
  }

pub async fn market_data_loader(file_name : String) -> Vec<DiscountFactor> {
  let market_data_r = load_market_data(file_name).await;
  match market_data_r {
      Ok(market_data) => {
        discount_factor(&market_data, Periodicity::SemiAnnual)
      }
      Err(err) => {
          panic!("Error {:?}", err);
      }
  }

}
}