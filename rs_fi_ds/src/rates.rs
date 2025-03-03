pub mod spot_rates {
    use crate::bond::bond::DiscountFactor;
    use crate::bond::bond::MarketData;
    use crate::bond::bond::Periodicity;

    /// Approximate discount factors for spot rates. The number of days is assumed.
    pub fn discount_factors(
        market_data: Vec<f32>,
        periodicity: Periodicity,
        number_of_days: f32,
        term: f32,
    ) -> Vec<DiscountFactor> {
        let mut result: Vec<DiscountFactor> = Vec::new();
        return result;
    }
}
