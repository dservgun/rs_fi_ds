pub mod task {
  use crate::bond::bond::DiscountFactor;
  use crate::bond::bond::discount_factor;
  use crate::bond::bond::Periodicity;
  use crate::data_loader::data_loader::load_market_data;
  use std::fmt::*;
  use log::{info, warn};

  use actix_web:: {
    get, 
    post,
    put,
    error::ResponseError,
    web::Path, 
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
  };
  use serde::{Serialize, Deserialize};

impl Display for DiscountFactor {
  fn fmt(&self, f : &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.term, self.discount)
  }
}


  #[get("/discount_factors")]
  pub async fn get_discount_factor() -> Json<String> {
    let mut result = String::new();

    for df in market_data_loader(String::from("./tests/bond_data.csv")).await {
      result.push_str(&df.to_string());
    };
    return Json(result);
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