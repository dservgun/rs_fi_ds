pub mod data_loader {

    use crate::bond::bond::discount_factor;
    use crate::bond::bond::DiscountFactor;
    use crate::bond::bond::MarketData;
    use crate::bond::bond::Periodicity;
    use crate::rates::rates::OvernightRateType;
    use crate::rates::rates::SwapRate;
    use crate::rates::rates::NextSettlementDate;
    use chrono::NaiveDate;
    use datafusion::common::arrow::array::*;
    use datafusion::error::*;
    use datafusion::prelude::*;
    use log::debug;
    use std::str::FromStr;

    pub fn parse_date(input: &str, format: &str) -> NaiveDate {
        let result = NaiveDate::parse_from_str(input, format);
        match result {
            Ok(r) => r,
            Err(_) => {
                panic!("{}", format!("Failed to parse date {}", input));
            }
        }
    }

    pub async fn load_next_settlement_dates(file_name : String) -> Result<Vec<NextSettlementDate>> {
        const START_DATE_COLUMN: &str = "start_date";
        const TERM_COLUMN: &str = "term";
        const NEXT_SETTLEMENT_DATE_COLUMN: &str = "next_settlement_date";
        const DATE_FORMAT: &str = "%m/%d/%Y";

        let ctx = SessionContext::new();
        let data_frame = ctx.read_csv(file_name, CsvReadOptions::new()).await?;
        let batches: Vec<RecordBatch> = data_frame.collect().await?;
        let mut result: Vec<NextSettlementDate> = Vec::new();

        for batch in batches {
            let num_rows = batch.num_rows();
            let dates = match batch.column_by_name(START_DATE_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", START_DATE_COLUMN)
            };

            let terms = match batch.column_by_name(TERM_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", TERM_COLUMN)
            };
            let next_settlement_dates = match batch.column_by_name(NEXT_SETTLEMENT_DATE_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", NEXT_SETTLEMENT_DATE_COLUMN),
            };

            for i in 0..num_rows {
                    let m = NextSettlementDate {
                        start_date: match dates {
                            Some(v) => parse_date(v.value(i), DATE_FORMAT),
                            None => panic!("Missing date"),
                        },
                        term: match terms {
                            Some(v) => f32::from_str(v.value(i).trim()).unwrap(),
                            None => panic!("Missing term"),
                        },
                        next_settlement_date: match next_settlement_dates {
                            Some(v) => parse_date(v.value(i).trim(), DATE_FORMAT),
                            None => panic!("Missing rates."),
                        },
                    };
                    debug!("Adding spot rate");
                    result.push(m)
                }
            }
        Ok(result)        

    }
    pub async fn load_spot_rates(
        file_name: String,
        swap_rate_type: OvernightRateType,
    ) -> Result<Vec<SwapRate>> {
        const DATE_COLUMN: &str = "Date";
        const TERM_COLUMN: &str = "Term";
        const RATE_COLUMN: &str = "Rate";
        const DATE_FORMAT: &str = "%m/%d/%Y";

        let ctx = SessionContext::new();
        let data_frame = ctx.read_csv(file_name, CsvReadOptions::new()).await?;
        let batches: Vec<RecordBatch> = data_frame.collect().await?;
        let mut result: Vec<SwapRate> = Vec::new();

        for batch in batches {
            let num_rows = batch.num_rows();
            let dates = match batch.column_by_name(DATE_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", DATE_COLUMN)
            };

            let terms = match batch.column_by_name(TERM_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", TERM_COLUMN)
            };
            let rates = match batch.column_by_name(RATE_COLUMN) {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found {}", RATE_COLUMN),
            };

            for i in 0..num_rows {
                    let m = SwapRate {
                        date: match dates {
                            Some(v) => parse_date(v.value(i), DATE_FORMAT),
                            None => panic!("Missing date"),
                        },
                        term: match terms {
                            Some(v) => f32::from_str(v.value(i).trim()).unwrap(),
                            None => panic!("Missing term"),
                        },
                        rate: match rates {
                            Some(v) => f32::from_str(v.value(i).trim()).unwrap(),
                            None => panic!("Missing rates."),
                        },
                        swap_rate_type,
                    };
                    debug!("Adding spot rate");
                    result.push(m)
                }
            }
        Ok(result)        
    }

    pub async fn load_market_data(file_name: String) -> Result<Vec<MarketData>> {
        let ctx = SessionContext::new();
        let df = ctx.read_csv(file_name, CsvReadOptions::new()).await?;
        let batches: Vec<RecordBatch> = df.collect().await?;
        let mut result: Vec<MarketData> = Vec::new();
        for batch in batches {
            let num_rows = batch.num_rows();
            let coupons = match batch.column_by_name("Coupon") {
                Some(col) => col.as_any().downcast_ref::<array::Float64Array>(),
                None => panic!("Column not found : Coupon"),
            };

            let maturity = match batch.column_by_name("Maturity") {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found : Maturity"),
            };
            let price = match batch.column_by_name("Price") {
                Some(col) => col.as_any().downcast_ref::<StringArray>(),
                None => panic!("Column not found : Price"),
            };
            for i in 0..num_rows {
                let m = MarketData {
                    coupon_rate: match coupons {
                        Some(v) => v.value(i) as f32,
                        None => 0.0,
                    },
                    term: match maturity {
                        Some(v) => f32::from_str(v.value(i).trim()).unwrap(),
                        None => 0.0,
                    },
                    market_price: match price {
                        Some(v) => f32::from_str(v.value(i).trim()).unwrap(),
                        None => 0.0,
                    },
                };
                debug!("Adding {:?}", m);
                result.push(m);
            }
        }
        Ok(result)
    }
    pub async fn market_data_loader(file_name: String) -> Vec<DiscountFactor> {
        let market_data_r: Result<Vec<MarketData>> = load_market_data(file_name).await;
        match market_data_r {
            Ok(market_data) => discount_factor(&market_data, Periodicity::SemiAnnual),
            Err(err) => {
                panic!("Error {:?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::data_loader::data_loader::load_market_data;
    use crate::data_loader::data_loader::load_spot_rates;
    use crate::rates::rates::OvernightRateType;

    #[actix_web::main]
    async fn test_load_market_data() {
        let market_data = load_market_data(String::from("tests/bond_data.csv")).await;
        println!("Market data {:?}", market_data);
    }

    #[actix_web::main]
    async fn test_load_spot_rates() {
        let spot_rates = load_spot_rates(
            String::from("tests/spot_rates.csv"),
            OvernightRateType::SOFR,
        )
        .await;
        println!("Spot rates {:?}", spot_rates);
    }
}
