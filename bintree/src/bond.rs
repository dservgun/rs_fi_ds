pub mod bond {
    use chrono::{Months, NaiveDate, ParseError};
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
    use filters::filter::Filter;
    #[derive(Debug, Clone, Copy)]
    pub enum PaymentSchedule {
        Quarterly,
        SemiAnnual,
        Annual,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct DiscountFactor {
        pub term: f32,
        pub discount: f32,
    }

    /// Market data is assumed to be for the
    /// conventional coupon face value of USD 100.00.
    /// Also assuming that the market data is from today out into the
    /// next terms.
    #[derive(Debug, Clone, Copy)]
    pub struct MarketData {
        pub coupon_rate: f32,
        pub term: f32,
        pub market_price: f32,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ErrorType {
        InvalidDate,
        InvalidRate,
        InvalidPrincipal,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct BondError {
        pub message: & 'static str,
        pub message_code: ErrorType,
    }
    

    #[derive(Debug, Clone, Copy)]
    pub struct Bond {
        pub principal: f32,
        pub issue_date: NaiveDate,
        pub maturity_date: NaiveDate,
        pub coupon_rate: f32,
        pub payment_schedule: PaymentSchedule,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CashFlow {
        pub bond: Bond,
        pub time: NaiveDate,
        pub amount: f32,
    }

    impl PartialEq for CashFlow {
        fn eq(&self, other: &Self) -> bool {
            return self.time == other.time && (f32::EPSILON < (self.amount - other.amount).abs());
        }
    }

    impl Eq for Bond {}
    impl PartialEq for Bond {
        fn eq(&self, other: &Self) -> bool {
            self.maturity_date == other.maturity_date && self.issue_date == other.issue_date
        }
    }

    impl Ord for Bond {
        fn cmp(&self, other: &Self) -> Ordering {
            let mat_date: Ordering = self.maturity_date.cmp(&other.maturity_date);
            match (mat_date) {
                Ordering::Equal => self.issue_date.cmp(&other.issue_date),
                _ => mat_date,
            }
        }
    }

    impl PartialOrd for Bond {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // the principal,
    // the term amount.
    // the term rate.
    // Need a struct to capture this, problem 2

    impl Bond {
        pub fn create_bond(
            principal: f32,
            issue_date: &str ,
            maturity_date: &str,
            rate: f32,
            date_format: &str,
        ) -> Result<Bond, BondError> {
            let m_date: Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str(maturity_date, date_format);
            let i_date: Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str(issue_date, date_format);

            match (i_date, m_date) {
                (Ok(i_date_unwrapped), Ok(maturity_date_unwrapped)) => {
                    let b1: Bond = Bond {
                        principal: principal,
                        issue_date: i_date_unwrapped,
                        maturity_date: maturity_date_unwrapped,
                        coupon_rate: rate,
                        payment_schedule: PaymentSchedule::SemiAnnual,
                    };
                    return Ok(b1);
                }
                _ => {
                    return Err(BondError {
                        message: "Invalid date",
                        message_code: ErrorType::InvalidDate,
                    });
                }
            }
        }
        pub fn coupon_payment(self) -> f32 {
            match self.payment_schedule {
                PaymentSchedule::SemiAnnual => {
                    return self.principal * (self.coupon_rate / 2.0);
                }
                PaymentSchedule::Quarterly => {
                    return self.principal * (self.coupon_rate / 4.0);
                }
                PaymentSchedule::Annual => {
                    return self.principal * (self.coupon_rate);
                }
            }
        }

        // Some helper functions
        fn get_months(self) -> u32 {
            match self.payment_schedule {
                PaymentSchedule::SemiAnnual => {
                    return 6;
                }
                PaymentSchedule::Quarterly => {
                    return 3;
                }
                PaymentSchedule::Annual => {
                    return 1;
                }
            }
        }

        fn get_months_f32(self) -> f32 {
            match self.payment_schedule {
                PaymentSchedule::Quarterly => {
                    return 3.0;
                }
                PaymentSchedule::SemiAnnual => {
                    return 6.0;
                }
                PaymentSchedule::Annual => {
                    return 1.0;
                }
            }
        }
        pub fn payment_intervals(self) -> Vec<NaiveDate> {
            let mut result = Vec::new();
            let mut st = self.issue_date;
            while st <= self.maturity_date {
                st = st + Months::new(self.get_months());
                result.push(st);
            }
            println!("{:?}", result);
            return result;
        }

        /// Simple cash flow based on the
        /// Coupon rate and paid out over the year.
        pub fn cashflow(self) -> Vec<CashFlow> {
            let intervals : &Vec<NaiveDate> = (&self.payment_intervals());
            let mut iter = intervals.into_iter().peekable();
            let mut result = Vec::new();
            while let Some(coupon_time) = iter.next() {
                if (iter.peek().is_none()){
                  let cashflow: CashFlow = CashFlow {
                      bond: self.clone(),
                      time: coupon_time.clone(),
                      amount: self.principal + self.coupon_payment(),
                  };
                  result.push(cashflow);
                } else {
                  let cashflow: CashFlow = CashFlow {
                      bond: self.clone(),
                      time: coupon_time.clone(),
                      amount: self.coupon_payment(),
                  };
                  result.push(cashflow);                  
                }

            }
            return result;
        }

        /// Return cash flow between two time intervals
        pub fn cashflow_between(self, startDate : NaiveDate, endDate : NaiveDate) -> Vec<CashFlow> {
          let inrange = (|a : &CashFlow| {a.time >= startDate}).and(|a : &CashFlow| {a.time <= endDate});
          self.cashflow().into_iter().filter(|x| inrange.filter(x)).collect()
        }
    }

    fn get_months_as_f32(payment_schedule: PaymentSchedule) -> f32 {
        match payment_schedule {
            PaymentSchedule::Quarterly => {
                return 3.0;
            }
            PaymentSchedule::SemiAnnual => {
                return 6.0;
            }
            PaymentSchedule::Annual => {
                return 12.0;
            }
        }
    }
    /// Given a table of [MarketData] return a discount factor table.
    pub fn discount_factor(
        market_data: &Vec<MarketData>,
        payment_schedule: PaymentSchedule,
    ) -> Vec<DiscountFactor> {
        let mut result: Vec<DiscountFactor> = Vec::new();
        let months_f32: f32 = get_months_as_f32(payment_schedule);
        let months_in_year: f32 = 12.0;
        let interest_factor: f32 = months_in_year / months_f32;
        let mut counter: f32 = 0.0;
        for i in 0..market_data.len() {
            if i == 0 {
                let numerator: f32 = market_data[i].market_price;
                let denominator: f32 = 100.0 + market_data[i].coupon_rate / interest_factor;
                let init_value: f32 = numerator / denominator;
                println!(
                    "Using numerator {:?} and denominator {:?}",
                    numerator, denominator,
                );
                let df: DiscountFactor = DiscountFactor {
                    term: months_f32 / months_in_year,
                    discount: init_value,
                };
                counter = counter + 1.0;
                result.push(df);
            } else {
                let md: MarketData = market_data[i];
                let mut inter_sigma = 0.0;
                for i in 0..i {
                    inter_sigma =
                        inter_sigma + (md.coupon_rate / interest_factor) * result[i].discount;
                }
                println!("Using intermediate discounts {:?}", inter_sigma);
                let numerator: f32 = md.market_price - inter_sigma;
                let denominator: f32 = 100.00 + (md.coupon_rate / interest_factor);
                let new_value = numerator / denominator;
                println!(
                    "Using numerator {:?} and denominator {:?}",
                    numerator, denominator,
                );

                let df: DiscountFactor = DiscountFactor {
                    term: counter * months_f32 / months_in_year,
                    discount: new_value,
                };
                result.push(df);
            }
            counter = counter + 1.0;
        }
        return result;
    }
} // End mod.

/// Test code
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use chrono::{Months, NaiveDate, ParseError};

    use bond::{discount_factor, Bond, BondError, DiscountFactor, MarketData, PaymentSchedule};

    fn create_test_bond() -> Result<Bond, BondError> {
        return Bond::create_bond(
            100.0,
            String::from("04/15/2014").as_str(),
            String::from("05/15/2024").as_str(),
            2.5,
            String::from("%m/%d/%Y").as_str(),
        );
    }

    #[test]
    fn test_bond_sort() {
        let b1: Result<Bond, BondError> = Bond::create_bond(
            100.0,
            String::from("04/15/2014").as_str(),
            String::from("05/15/2024").as_str(),
            2.5,
            String::from("%m/%d/%Y").as_str(),
        );

        let b2: Result<Bond, BondError> = Bond::create_bond(
            100.0,
            String::from("03/15/2014").as_str(),
            String::from("05/15/2024").as_str(),
            2.5,
            String::from("%m/%d/%Y").as_str(),
        );
        let mut bonds: Vec<Bond> = Vec::new();
        match (b1, b2) {
            (Ok(bond1), Ok(bond2)) => {
                bonds.push(bond1);
                bonds.push(bond2);
            }
            _ => {
                panic!("Test failed." );
            }
        }
        bonds.sort();
        assert_eq!(b1.unwrap(), bonds[1]);
    }
    fn create_test_market_data() -> Vec<MarketData> {
        let mut result: Vec<MarketData> = Vec::new();
        let md1 = MarketData {
            coupon_rate: 2.875,
            term: 0.5,
            market_price: 101.4297,
        };
        result.push(md1);
        let md2 = MarketData {
            coupon_rate: 2.125,
            term: 1.0,
            market_price: 102.0662,
        };
        result.push(md2);
        let md3 = MarketData {
            coupon_rate: 1.625,
            term: 1.5,
            market_price: 102.2862,
        };
        result.push(md3);

        let md4 = MarketData {
            coupon_rate: 0.125,
            term: 2.0,
            market_price: 99.9538,
        };
        let md5 = MarketData {
            coupon_rate: 0.250,
            term: 2.5,
            market_price: 100.0795,
        };
        let md6 = MarketData {
            coupon_rate: 0.250,
            term: 3.0,
            market_price: 99.7670,
        };
        let md7 = MarketData {
            coupon_rate: 2.250,
            term: 3.5,
            market_price: 106.3091,
        };
        result.push(md4);
        result.push(md5);
        result.push(md6);
        result.push(md7);
        return result;
    }

    #[test]
    fn test_create() {
        println!("{:?}", create_test_bond());
    }

    #[test]
    fn test_coupon_rate() {
        let b1 = create_test_bond();
        match b1 {
            Result::Ok(val) => {
                assert_eq!(val.coupon_payment(), 125.0);
            }
            Result::Err(_) => {
                assert_eq!("", "ABC");
            }
        }
    }

    #[test]
    fn test_payment_intervals() {
        let b1 = create_test_bond();
        match b1 {
            Result::Ok(val) => {
                let intervals = val.payment_intervals();
                assert_eq!(intervals.len(), 21);
            }
            Result::Err(_) => {
                panic!("Failed to create bond.");
            }
        }
    }

    #[test]
    fn test_cashflow() {
        let b1 = create_test_bond();
        match b1 {
            Result::Ok(val) => {
                let mut cashflows = val.cashflow().into_iter().peekable();
                while let Some(cashflow) = cashflows.next() {
                  if(cashflows.peek().is_none()) {
                    assert_approx_eq!(cashflow.amount, 225.0, f32::EPSILON);
                  } else {
                    assert_approx_eq!(cashflow.amount, 125.0, f32::EPSILON);
                  }
                }
            }
            Result::Err(_) => {
                panic!("Failed to create bond.");
            }
        }
    }

    #[test]
    fn test_cashflow_between1() {
      let b1 = create_test_bond();
      let date_format = "%m/%d/%Y";
      match b1 {
        Result::Ok(val) => {
          let start_date_opt: Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str("04/15/2014", date_format);
          let end_date_opt : Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str("04/15/2014", date_format);
          match(start_date_opt, end_date_opt) {
            (Ok(start_date), Ok(end_date)) => {
            let mut cashflows = val.cashflow_between(start_date, end_date);
            assert_eq!(0, cashflows.len());
            }
            (_, _) => {
              panic!("Failed to test cash flows");
            }
          }
        }
        Result::Err(_) => {
          panic!("Failed to create bond.")
        }
      }      
    }

    #[test]
    fn test_cashflow_between2() {
      let b1 = create_test_bond();
      let date_format = "%m/%d/%Y";
      match b1 {
        Result::Ok(val) => {
          let start_date_opt: Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str("04/15/2014", date_format);
          let end_date_opt : Result<NaiveDate, ParseError> =
                NaiveDate::parse_from_str("11/15/2014", date_format);
          match(start_date_opt, end_date_opt) {
            (Ok(start_date), Ok(end_date)) => {
            let mut cashflows = val.cashflow_between(start_date, end_date);
            assert_eq!(1, cashflows.len());
            assert_approx_eq!(125.0, cashflows[0].amount);
            println!("Cashflow {:?}", cashflows);
            }
            (_, _) => {
              panic!("Failed to test cash flows");
            }
          }
        }
        Result::Err(_) => {
          panic!("Failed to create bond.")
        }
      }      
    }


    #[test]
    fn test_create_discount_factor() {
        let market_data: Vec<MarketData> = create_test_market_data();
        let discount_factor: Vec<DiscountFactor> =
            discount_factor(&market_data, PaymentSchedule::SemiAnnual);
        for i in discount_factor {
            println!(
                "Discount factor : Term {:?} -> Discount {:?}",
                i.term, i.discount
            );
        }
    }
}
