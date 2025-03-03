mod bintree;
mod bond;
mod data_loader;
mod interest_rate_swap;
mod pandl;
mod rates;
mod tbills;
use crate::bond::bond::DiscountFactor;
use crate::bond::bond::discount_factor;
use crate::bond::bond::Periodicity;
use crate::data_loader::data_loader::load_market_data;


#[tokio::main]
async fn main() {

let market_data_r = load_market_data(String::from("./tests/bond_data.csv")).await;
match market_data_r {
    Ok(market_data) => {
    let discount_factor: Vec<DiscountFactor> =
        discount_factor(&market_data, Periodicity::SemiAnnual);
    for i in discount_factor {
        println!(
            "Discount factor : Term {:?} -> Discount {:?}",
            i.term, i.discount
        );
    }
    }
    Err(err) => {
        panic!("Error {:?}", err);
    }
}
}
