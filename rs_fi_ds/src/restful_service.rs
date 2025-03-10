pub mod task {
    use crate::rates::rates::OvernightRateType;
use crate::bond::bond::discount_factor;
    use crate::rates::rates::SwapRate;
    use crate::data_loader::data_loader::load_spot_rates;
    use crate::bond::bond::DiscountFactor;
    use crate::bond::bond::Periodicity;
    use crate::data_loader::data_loader::load_market_data;
    use crate::data_loader::data_loader::market_data_loader;

    use log::{info, warn};
    use std::fmt::*;

    use actix_web::{
        body::BoxBody,
        error::ResponseError,
        get,
        http::{header::ContentType, StatusCode},
        post, put,
        web::Data,
        web::Json,
        web::Path,
        HttpRequest, HttpResponse, Responder, Result,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct DiscountFactorsResponse {
        pub discount_factors: Vec<DiscountFactor>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SpotRatesResponse {
      pub spot_rates : Vec<SwapRate>,
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

    impl Responder for SpotRatesResponse {
      type Body = BoxBody;
      fn respond_to(self, _req : &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self.spot_rates).unwrap();
        HttpResponse::Ok()
          .content_type(ContentType::json())
          .body(body)
      }
    }

    #[get("/discount_factors")]
    pub async fn get_discount_factor() -> Result<impl Responder> {
        let discount_factors = market_data_loader(String::from("./tests/bond_data.csv")).await;
        Ok(DiscountFactorsResponse {
            discount_factors
        })
    }

    #[get("/get_spot_rates")]
    pub async fn get_spot_rates() -> Result<impl Responder> {
      info!("Running get spot rates");
      let spot_rates = load_spot_rates(String::from("./tests/spot_rates.csv"), OvernightRateType::SOFR).await;
      info!("Returning spot rates {:?}", spot_rates);
      match spot_rates {
        Ok(s_rates) => Ok(SpotRatesResponse {spot_rates : s_rates}),
        Err(_) => todo!()
      }
    }

}
