pub mod bond {
    use chrono::{Datelike, Months, NaiveDate, ParseError};
    use filters::filter::Filter;
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
    #[derive(Debug, Clone, Copy)]
    pub enum Periodicity {
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
        pub message: &'static str,
        pub message_code: ErrorType,
    }

    /// A bond with an issue date, principal and a maturity date.
    /// [Regulationss](https://treasurydirect.gov/files/laws-and-regulations/auction-regulations-uoc/auct-reg-gsr-31-cfr-356.pdf)
    /// Notes are based on Bond Math - Donald J. Smith and Bruce Tuckman.
    /// ### Some examples
    /// Example 1 : Suppose I buy a zero coupon corporate bond at 60 but I don't intend to hold this
    /// bond till maturity, what is the analytics supporting selling the bond after,
    /// for example 2 years, 3 years etc.

    /// How to compute returns on a bond. Consider for example a zero coupon bond selling at 60 expiring in 10 years.
    /// ```rust
    /// let b1 = create_bond(
    /// principal,
    /// String::from("04/15/2021").as_str(),
    /// String::from("04/15/2051").as_str(),
    /// 0.0,
    /// String::from("%m/%d/%Y").as_str(),
    /// );
    ///
    /// match b1 {
    /// Result::Ok(val) => {
    ///  let result = val.constant_yield_price_trajectory(60.0).into_iter();
    ///  let annual_returns =
    ///  result.filter(|(a, _)| a.month() == 4);
    ///  for i in annual_returns {
    ///   println!("Price Trajectory {:?}", i);
    ///  }
    /// }
    /// Result::Err(_) => {
    ///  panic!("Failed to create bond")
    ///  }
    /// }
    /// ```
    /// ### Some terms and concepts
    /// Yield is an investor's required rate of return for holding the bond till
    /// maturity and bear the default risk.
    /// ### Some relationships
    /// #### The relationship between coupon rate and the yield to maturity.
    ///   * If the *coupon rate* = *yield to maturity* the bond is priced at par.
    ///   * If the *coupon rate* < *yield to maturity* the bond is priced at a discount.
    ///   * If the *coupon rate* > *yield to maturity* the bond is priced at a premium.
    /// These above rules apply only on the coupon dates and the rest of the dates need to account for
    /// accrued interest on the bond.

    #[derive(Debug, Clone, Copy)]
    pub struct Bond {
        pub principal: f32,
        pub issue_date: NaiveDate,
        pub maturity_date: NaiveDate,
        pub coupon_rate: f32,
        pub periodicity: Periodicity,
        pub reinvestment_interest: Option<f32>,
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
            match mat_date {
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

    pub fn create_bond_with_periodicity(
        principal: f32,
        issue_date: &str,
        maturity_date: &str,
        rate: f32,
        reinvestment_interest_rate: f32,
        periodicity: Periodicity,
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
                    periodicity: periodicity,
                    reinvestment_interest: Some(reinvestment_interest_rate),
                };
                return Ok(b1);
            }
            _ => {
                return Err(BondError {
                    message: "Invalid Date",
                    message_code: ErrorType::InvalidDate,
                });
            }
        }
    }

    pub fn create_bond(
        principal: f32,
        issue_date: &str,
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
                    periodicity: Periodicity::SemiAnnual,
                    reinvestment_interest: None,
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

    impl Bond {
        pub fn coupon_payment(self) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => {
                    return self.principal * (self.coupon_rate / 4.0);
                }
                Periodicity::SemiAnnual => {
                    return self.principal * (self.coupon_rate / 2.0);
                }
                Periodicity::Annual => {
                    return self.principal * (self.coupon_rate);
                }
            }
        }

        pub fn reinvestment_amount(self) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => match self.reinvestment_interest {
                    Some(int_value) => self.coupon_payment() * int_value / 4.0,
                    None => 0.0,
                },
                Periodicity::SemiAnnual => match self.reinvestment_interest {
                    Some(int_value) => self.coupon_payment() * int_value / 2.0,
                    None => 0.0,
                },
                Periodicity::Annual => match self.reinvestment_interest {
                    Some(int_value) => self.coupon_payment() * int_value,
                    None => 0.0,
                },
            }
        }

        // Some helper functions
        fn get_months(self) -> u32 {
            match self.periodicity {
                Periodicity::SemiAnnual => {
                    return 6;
                }
                Periodicity::Quarterly => {
                    return 3;
                }
                Periodicity::Annual => {
                    return 1;
                }
            }
        }

        /// Compute the infinitely compounded rate for a specified market rate.
        pub fn infinitely_compounded_rate(self, market_price: f32) -> f32 {
            1.0 / self.total_years() * (f32::ln(self.principal / market_price))
        }

        pub fn rate_for_periodicity(self, periodicity: Periodicity, market_price: f32) -> f32 {
            let compounded_rate = self.infinitely_compounded_rate(market_price);
            let prefix = match periodicity {
                Periodicity::Quarterly => 4.0,
                Periodicity::SemiAnnual => 2.0,
                Periodicity::Annual => 1.0,
            };
            return prefix * (f32::exp(compounded_rate / prefix) - 1.0);
        }

        fn total_years(self) -> f32 {
            self.maturity_date.years_since(self.issue_date).unwrap() as f32
        }

        fn get_num_periods(self) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => self.total_years() * 4.0,
                Periodicity::SemiAnnual => self.total_years() * 2.0,
                Periodicity::Annual => self.total_years(),
            }
        }

        fn get_num_periods_for_years(self, years: f32) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => years * 4.0,
                Periodicity::SemiAnnual => years * 2.0,
                Periodicity::Annual => years,
            }
        }

        fn get_periods_per_year(self) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => 4.0,
                Periodicity::SemiAnnual => 2.0,
                Periodicity::Annual => 1.0,
            }
        }

        fn get_adj_interest_per_period(self) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => self.coupon_rate / 4.0,
                Periodicity::SemiAnnual => self.coupon_rate / 6.0,
                Periodicity::Annual => self.coupon_rate / 1.0,
            }
        }

        fn adj_interest_per_period(self, ytm: f32) -> f32 {
            match self.periodicity {
                Periodicity::Quarterly => ytm / 4.0,
                Periodicity::SemiAnnual => ytm / 2.0,
                Periodicity::Annual => ytm,
            }
        }
        pub fn is_zero_coupon_bond(self) -> bool {
            return (self.coupon_rate - 0.0).abs() < f32::EPSILON;
        }

        /// Assume the entire period of maturity from the beginning of the
        /// bond.
        pub fn yield_to_maturity(self, market_price: f32) -> Option<f32> {
            if self.is_zero_coupon_bond() {
                let num_per: f32 = self.get_num_periods();
                println!("Using num_per {:?}", num_per);
                let fv = f32::powf(self.principal / market_price, 1.0 / num_per);
                println!("Fv {:?}", fv);
                return Some((fv - 1.0) * self.get_periods_per_year());
            } else {
                None
            }
        }

        pub fn realized_return(self, purchase_price: f32, sale_price: f32, years: f32) -> f32 {
            let periods: f32 = self.get_num_periods_for_years(years);
            let rhs: f32 = f32::powf(sale_price / purchase_price, 1.0 / periods);
            return (rhs - 1.0) * self.get_periods_per_year();
        }

        /// Return the baseline price at a ['market_price'] after ['years'].
        pub fn at_the_money_yield_trajectory(self, market_price: f32, years: i32) -> f32 {
            let ytm_option: Option<f32> = self.yield_to_maturity(market_price);
            match ytm_option {
                Some(ytm) => {
                    let intervals: &Vec<NaiveDate> = &self.periodicity();
                    let interest_rate: f32 = self.adj_interest_per_period(ytm);
                    let mut iter = intervals.into_iter().peekable();
                    let mut accum = market_price;

                    while let Some(coupon_time) = iter.next() {
                        println!("Adding date {:?} accum : {:?}", coupon_time.clone(), accum);
                        if (self.issue_date.year() - coupon_time.year()).abs() < years {
                            accum = accum * (1.0 + interest_rate)
                        } else {
                            break;
                        }
                    }

                    return accum;
                }
                None => {
                    return 0.0;
                }
            }
        }

        fn market_price_at_date(self, ytm: f32, at_date: NaiveDate) -> f32 {
            let intervals: &Vec<NaiveDate> = &self.periodicity();
            let interest_rate: f32 = self.adj_interest_per_period(ytm);
            let mut iter = intervals.into_iter().peekable();
            let mut accum = 0.0;
            let mut counter = 0;

            while let Some(coupon_time) = iter.next() {
                println!(
                    "Bond : {:?} Coupon time {:?} compare {:?} using {:?}",
                    self, coupon_time, at_date, interest_rate
                );
                if *coupon_time >= at_date {
                    let den = f32::powf(1.0 + interest_rate, counter as f32);
                    println!("Time value {:?}", den);
                    if iter.peek().is_none() {
                        accum = accum + (self.coupon_rate + self.principal) / den;
                    } else {
                        accum = accum + (self.coupon_rate / den);
                    }
                }
                counter = counter + 1;
            }
            return accum;
        }

        pub fn market_price_trajectory(self, ytm: f32) -> Vec<(NaiveDate, f32)> {
            let intervals: &Vec<NaiveDate> = &self.periodicity();
            let mut iter = intervals.into_iter().peekable();
            let mut result = Vec::new();
            while let Some(coupon_time) = iter.next() {
                let mp = self.market_price_at_date(ytm, coupon_time.clone());
                result.push((coupon_time.clone(), mp));
            }
            return result;
        }

        /// A useful yardstick is the constant yield price trajectory. This is the path the bond
        /// take over time to maturity. The trajectory says the following, if the market price is above
        /// the price point in the trajectory, the investor could sell it.
        pub fn constant_yield_price_trajectory(self, market_price: f32) -> Vec<(NaiveDate, f32)> {
            let mut result: Vec<(NaiveDate, f32)> = Vec::new();
            let ytm_option: Option<f32> = self.yield_to_maturity(market_price);
            match ytm_option {
                Some(ytm) => {
                    let intervals: &Vec<NaiveDate> = &self.periodicity();
                    let interest_rate: f32 = self.adj_interest_per_period(ytm);
                    let mut iter = intervals.into_iter().peekable();
                    let mut accum = market_price;
                    while let Some(coupon_time) = iter.next() {
                        println!("Adding date {:?} accum : {:?}", coupon_time.clone(), accum);
                        result.push((coupon_time.clone(), accum));
                        accum = accum * (1.0 + interest_rate)
                    }

                    return result;
                }
                None => {
                    panic!("Failed to compute ytm");
                }
            }
        }

        pub fn periodicity(self) -> Vec<NaiveDate> {
            let mut result = Vec::new();
            let mut st = self.issue_date;
            result.push(st);
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
            let intervals: &Vec<NaiveDate> = &self.periodicity();
            let mut iter = intervals.into_iter().peekable();
            let mut result = Vec::new();
            while let Some(coupon_time) = iter.next() {
                println!("Bond : {:?}", self);
                if iter.peek().is_none() {
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
        pub fn cashflow_between(self, start_date: NaiveDate, end_date: NaiveDate) -> Vec<CashFlow> {
            let inrange =
                (|a: &CashFlow| a.time > start_date).and(|a: &CashFlow| a.time <= end_date);
            self.cashflow()
                .into_iter()
                .filter(|x| inrange.filter(x))
                .collect()
        }

        pub fn cashflow_between_inclusive(
            self,
            start_date: NaiveDate,
            end_date: NaiveDate,
        ) -> Vec<CashFlow> {
            let inrange =
                (|a: &CashFlow| a.time >= start_date).and(|a: &CashFlow| a.time <= end_date);
            self.cashflow()
                .into_iter()
                .filter(|x| inrange.filter(x))
                .collect()
        }

        /// Return the reinvestment amount for the coupon payments. Note: The last payment
        /// will be reinvested in the next term.
        pub fn reinvestment_amount_between(
            self,
            start_date: NaiveDate,
            end_date: NaiveDate,
        ) -> Vec<f32> {
            let inrange =
                (|a: &CashFlow| a.time >= start_date).and(|a: &CashFlow| a.time <= end_date);
            let cashflows: Vec<CashFlow> = self
                .cashflow()
                .into_iter()
                .filter(|x| inrange.filter(x))
                .collect();
            let mut cashflows_iter = cashflows.into_iter().peekable();
            let mut result = Vec::new();
            while let Some(_cash_flow) = cashflows_iter.next() {
                if cashflows_iter.peek().is_none() {
                    //Last cashflow, no reinvestment has been done yet.
                } else {
                    result.push(self.reinvestment_amount())
                }
            }

            return result;
        }
    }

    fn get_months_as_f32(payment_schedule: Periodicity) -> f32 {
        match payment_schedule {
            Periodicity::Quarterly => {
                return 3.0;
            }
            Periodicity::SemiAnnual => {
                return 6.0;
            }
            Periodicity::Annual => {
                return 12.0;
            }
        }
    }

    /// Given a table of ['MarketData'] return a discount factor table to be used for
    /// subsequent computations.
    pub fn discount_factor(
        market_data: &Vec<MarketData>,
        payment_schedule: Periodicity,
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
                    "Using numerator {:?} and denominator {:?} discount_factor {:?}",
                    numerator, denominator, init_value
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

#[cfg(test)]
mod tests {
    use crate::bond::bond::create_bond;
    use crate::bond::bond::discount_factor;
    use crate::bond::bond::Bond;
    use crate::bond::bond::BondError;
    use crate::bond::bond::DiscountFactor;
    use crate::bond::bond::MarketData;
    use crate::bond::bond::Periodicity;
    use assert_approx_eq::assert_approx_eq;
    use chrono::{Datelike, NaiveDate, ParseError};

    fn create_zcb_principal_maturity(
        principal: f32,
        issue_date: &str,
        mat_date: &str,
    ) -> Result<Bond, BondError> {
        return create_bond(principal, issue_date, mat_date, 0.0, "%m/%d/%Y");
    }

    fn create_zcb(principal: f32) -> Result<Bond, BondError> {
        return create_bond(
            principal,
            String::from("04/15/2021").as_str(),
            String::from("04/15/2051").as_str(),
            0.0,
            String::from("%m/%d/%Y").as_str(),
        );
    }

    fn create_test_bond() -> Result<Bond, BondError> {
        return create_bond(
            100.0,
            String::from("04/15/2014").as_str(),
            String::from("05/15/2024").as_str(),
            2.5,
            String::from("%m/%d/%Y").as_str(),
        );
    }

    #[test]
    fn test_bond_sort() {
        let b1: Result<Bond, BondError> = create_bond(
            100.0,
            String::from("04/15/2014").as_str(),
            String::from("05/15/2024").as_str(),
            2.5,
            String::from("%m/%d/%Y").as_str(),
        );

        let b2: Result<Bond, BondError> = create_bond(
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
                panic!("Test failed.");
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
                let intervals = val.periodicity();
                assert_eq!(intervals.len(), 22);
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
                    if cashflows.peek().is_none() {
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
                let end_date_opt: Result<NaiveDate, ParseError> =
                    NaiveDate::parse_from_str("04/15/2014", date_format);
                match (start_date_opt, end_date_opt) {
                    (Ok(start_date), Ok(end_date)) => {
                        let cashflows = val.cashflow_between(start_date, end_date);
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
                let end_date_opt: Result<NaiveDate, ParseError> =
                    NaiveDate::parse_from_str("11/15/2014", date_format);
                match (start_date_opt, end_date_opt) {
                    (Ok(start_date), Ok(end_date)) => {
                        let cashflows = val.cashflow_between(start_date, end_date);
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
            discount_factor(&market_data, Periodicity::SemiAnnual);
        for i in discount_factor {
            println!(
                "Discount factor : Term {:?} -> Discount {:?}",
                i.term, i.discount
            );
        }
    }

    #[test]
    fn test_ytm_zcb() {
        let b1 = create_zcb(1000.0);
        match b1 {
            Result::Ok(val) => {
                let ytm = val.yield_to_maturity(50.00).unwrap();
                assert_approx_eq!(ytm, 0.10239267, f32::EPSILON);
            }
            Result::Err(_) => {
                panic!("Failed to create bond.");
            }
        }
    }

    fn internal_yield_2() -> Vec<(NaiveDate, f32)> {
        let b1 = create_bond(
            100.00,
            String::from("04/15/2021").as_str(),
            String::from("04/15/2051").as_str(),
            6.00,
            String::from("%m/%d/%Y").as_str(),
        );
        let constant_yield = 0.2;
        match b1 {
            Result::Ok(val) => val.market_price_trajectory(constant_yield),
            Result::Err(err) => {
                panic!("Failed to create bond {:?}", err);
            }
        }
    }

    fn internal_yield_1() -> Vec<(NaiveDate, f32)> {
        let b1 = create_bond(
            100.00,
            String::from("04/15/2021").as_str(),
            String::from("04/15/2041").as_str(),
            6.00,
            String::from("%m/%d/%Y").as_str(),
        );
        let constant_yield = 0.20;
        match b1 {
            Result::Ok(val) => val.market_price_trajectory(constant_yield),
            Result::Err(err) => {
                panic!("Failed to create bond {:?}", err);
            }
        }
    }

    #[test]
    fn test_fixed_yield() {
        let mut y1 = internal_yield_1();
        let y2 = internal_yield_2();
        for i in y1.len()..y2.len() {
            let new_entry = (y2[i].0, 0.0);
            y1.push(new_entry);
        }

        for i in 0..y1.len() {
            let compare = y1[i].1 - y2[i].1;
            println!(
                "20 year - 30 year @ {:?} {:?} {:?} {:?}",
                y1[i].0, y1[i].1, y2[i].1, compare
            );
        }
    }

    #[test]
    fn test_ytm_trajectory() {
        let b1 = create_zcb_principal_maturity(100.0, "04/15/2021", "04/15/2031");
        match b1 {
            Result::Ok(val) => {
                let result = val.constant_yield_price_trajectory(60.0).into_iter();
                let annual_returns = result.filter(|(a, _)| a.month() == 4);
                for i in annual_returns {
                    println!("Price Trajectory {:?}", i);
                }
            }
            Result::Err(_) => {
                panic!("Failed to create bond")
            }
        }
    }

    /// This test case shows the return on investment after 2 years the
    /// at the money yield is
    /// ```rust
    /// atm = 66.45397
    /// sale price = 68
    /// yield = 4.879 %
    /// realized rate of return = 6.357
    /// ```
    #[test]
    fn test_ytm_at_the_money_1() {
        let b1 = create_zcb_principal_maturity(100.0, "04/15/2021", "04/15/2031");
        match b1 {
            Result::Ok(val) => {
                let result = val.at_the_money_yield_trajectory(60.0, 2);
                assert_approx_eq!(66.45397, result, f32::EPSILON);
                let realized_return = val.realized_return(60.0, 68.0, 2.0);
                assert_approx_eq!(0.063570976, realized_return, f32::EPSILON);
            }
            Err(_) => {
                panic!("Failed to create bond");
            }
        }
    }
    #[test]
    fn test_infinite_compounding() {
        let b1 = create_zcb_principal_maturity(100.0, "04/15/2021", "04/15/2031");
        match b1 {
            Result::Ok(val) => {
                let inf_compounded_rate = val.infinitely_compounded_rate(60.0);
                assert_approx_eq!(inf_compounded_rate, 0.05108256, f32::EPSILON);
                let mut rate_for_per = val.rate_for_periodicity(Periodicity::Quarterly, 60.0);
                assert_approx_eq!(rate_for_per, 0.0514102, f32::EPSILON);
                rate_for_per = val.rate_for_periodicity(Periodicity::Annual, 60.00);
                assert_approx_eq!(rate_for_per, 0.052409768);
                rate_for_per = val.rate_for_periodicity(Periodicity::SemiAnnual, 60.00);
                assert_approx_eq!(rate_for_per, 0.051740408);
            }
            Err(_) => {
                panic!("Failed to create bond");
            }
        }
    }
}
