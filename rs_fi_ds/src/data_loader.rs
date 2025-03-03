pub mod data_loader {

    use crate::bond::bond::MarketData;
    use datafusion::common::arrow::array::*;
    use datafusion::error::*;
    use datafusion::prelude::*;
    use std::str::FromStr;
    use log::{info, warn, debug};

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
}

#[cfg(test)]
mod tests {

    use crate::data_loader::data_loader::load_market_data;

    #[tokio::test]
    async fn test_load_market_data() {
        let market_data = load_market_data(String::from("tests/bond_data.csv")).await;
        println!("Market data {:?}", market_data);
    }
}
