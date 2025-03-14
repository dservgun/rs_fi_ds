//! Interest rates are always quoted annually but adjusted to the appropriate
/// periodicity based on the underlying product.
///
/// * Par Rate - is defined as the rate that costs par and pays par at maturity. For example,
/// if, a 10-year Treasury bond has a coupon of 1.625% and costs 100.00 then the par rate is
/// 1.625%.
pub mod rates {
    use crate::bond::bond::DiscountFactor;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    use crate::bond::bond::Periodicity;
    use chrono::NaiveDate;

    /// Acronyms
    /// * SOFR - Secured Overnight Financing Rate.
    /// * SONIA - Sterling Overnight Interbank Average.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
    pub enum OvernightRateType {
        SOFR,
        SONIA,
    }

    /// The `SwapRate` for a `date` for a `term`. These elements are
    /// used to compute spot rates, discount factors, and forward rates for a
    /// term structure.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct SwapRate {
        pub date: NaiveDate,
        pub term: f32,
        pub rate: f32,
        pub swap_rate_type: OvernightRateType,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct NextSettlementDate {
        pub start_date: NaiveDate,
        pub term: f32,
        pub next_settlement_date: NaiveDate,
    }

    impl Hash for NextSettlementDate {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.start_date.hash(state);
            self.next_settlement_date.hash(state);
        }
    }
    impl PartialEq for NextSettlementDate {
        fn eq(&self, other: &Self) -> bool {
            return self.start_date == other.start_date
                && (f32::EPSILON < (self.term - other.term).abs());
        }
    }

    impl Eq for NextSettlementDate {}

    impl NextSettlementDate {
        /// Return a hash map of settlement dates.
        pub fn get_settlement_dates(
            &self,
            calendar: Vec<NextSettlementDate>,
        ) -> HashMap<NextSettlementDate, Vec<NextSettlementDate>> {
            let mut result: HashMap<NextSettlementDate, Vec<NextSettlementDate>> = HashMap::new();
            for next_date in &calendar {
                if result.contains_key(&next_date) {
                    let current = result.get(&next_date);
                    let n = match current {
                        Some(c) => {
                            let mut r = c.clone();
                            r.push(*next_date);
                            r
                        }
                        None => Vec::new(),
                    };
                    result.insert(*next_date, n);
                } else {
                    let mut current = Vec::new();
                    current.push(*next_date);
                    result.insert(*next_date, current);
                }
            }
            return result;
        }
    }

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

#[cfg(test)]
mod tests {
    use crate::rates::rates::NextSettlementDate;
    use chrono::NaiveDate;
    #[test]
    fn test_next_settlement_date() {
        let s1 = NextSettlementDate {
            start_date: NaiveDate::parse_from_str("05/14/2021", "%m/%d/%Y").unwrap(),
            term: 0.5,
            next_settlement_date: NaiveDate::parse_from_str("11/14/2021", "%m/%d/%Y").unwrap(),
        };
        let s2 = NextSettlementDate {
            start_date: NaiveDate::parse_from_str("05/14/2021", "%m/%d/%Y").unwrap(),
            term: 0.5,
            next_settlement_date: NaiveDate::parse_from_str("05/14/2022", "%m/%d/%Y").unwrap(),
        };
        let mut calendar = Vec::new();
        calendar.push(s1);
        calendar.push(s2);
        let map = s1.get_settlement_dates(calendar);
        println!("Map {:?}", map);
    }
}
