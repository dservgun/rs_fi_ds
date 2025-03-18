pub mod bond {
    use chrono::{Datelike, Months, NaiveDate, ParseError};
    use filters::filter::Filter;
    use log::{debug};
    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

    /// Most products support annual, quarterly and semiannual payments.
    /// Continuous and Daily compounding are also supported.
    #[derive(Debug, Clone, Copy)]
    pub enum Periodicity {
        Quarterly,
        SemiAnnual,
        Annual,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DiscountFactor {
        pub term: f32,
        pub discount: f32,
    }

    /// Market data is assumed to be for the
    /// conventional coupon face value of USD 100.00.
    /// Also assuming that the market data is from today out into the
    /// next terms.

    /// ```
    /// use crate::bond::bond::create_bond;
    /// use crate::bond::bond::discount_factor;
    /// use crate::bond::bond::Bond;
    /// use crate::bond::bond::BondError;
    /// use crate::bond::bond::DiscountFactor;
    /// use crate::bond::bond::MarketData;
    /// use crate::bond::bond::Periodicity;
    /// use assert_approx_eq::assert_approx_eq;
    /// use chrono::{Datelike, NaiveDate, ParseError};
    /// fn create_test_market_data() -> Vec<MarketData> {
    ///     let mut result: Vec<MarketData> = Vec::new();
    ///     let md1 = MarketData {
    ///         coupon_rate: 2.875,
    ///         term: 0.5,
    ///         market_price: 101.4297,
    ///     };
    ///     result.push(md1);
    ///     let md2 = MarketData {
    ///         coupon_rate: 2.125,
    ///         term: 1.0,
    ///         market_price: 102.0662,
    ///     };
    ///     result.push(md2);
    ///     let md3 = MarketData {
    ///         coupon_rate: 1.625,
    ///         term: 1.5,
    ///         market_price: 102.2862,
    ///     };
    ///     result.push(md3);
    ///     let md4 = MarketData {
    ///         coupon_rate: 0.125,
    ///         term: 2.0,
    ///         market_price: 99.9538,
    ///     };
    ///     let md5 = MarketData {
    ///         coupon_rate: 0.250,
    ///         term: 2.5,
    ///         market_price: 100.0795,
    ///     };
    ///     let md6 = MarketData {
    ///         coupon_rate: 0.250,
    ///         term: 3.0,
    ///         market_price: 99.7670,
    ///     };
    ///     let md7 = MarketData {
    ///         coupon_rate: 2.250,
    ///         term: 3.5,
    ///         market_price: 106.3091,
    ///     };
    ///     result.push(md4);
    ///     result.push(md5);
    ///     result.push(md6);
    ///     result.push(md7);
    ///     return result;
    /// }

    /// fn test_create_discount_factor() {
    ///     let market_data: Vec<MarketData> = create_test_market_data();
    ///     let discount_factor: Vec<DiscountFactor> =
    ///         discount_factor(&market_data, Periodicity::SemiAnnual);
    ///     assert_approx_eq!(discount_factor[0].discount, 0.9999231, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[1].discount, 0.99941903, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[2].discount, 0.9985045, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[3].discount, 0.99704117, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[4].discount, 0.9945582, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[5].discount, 0.99019545, f32::EPSILON);
    ///     assert_approx_eq!(discount_factor[6].discount, 0.9847417, f32::EPSILON);
    /// }
    ///```

    #[derive(Debug, Clone, Copy)]
    pub struct MarketData {
        pub coupon_rate: f32,
        pub term: f32,
        pub market_price: f32,
    }

    /// The one-factor metrics for a Bond are:
    ///
    /// .   DV01 expands to *dollar value of an 01* and represents the change
    /// in the price of a bond for a change in rates of 0.01% or 1bps.
    ///
    /// .   Duration measures the percentage of changes in prices for a change in the yield.
    /// Both Duration and DV01 represent a rate of change.
    ///
    /// .   Convexity represents the rate of change of the DV01 and represents the
    /// second derivative of the price-rate function divided by price.

    /// These metrics are useful for hedging strategies. For example to hedge 2 portfolios,
    /// the face values and dvO1s defineds as fv1, dv01_1, fv2, dv01_2, the hedge is defined as
    ///
    ///         fv1 * dv01_1 + fv2 * dv01_2 = 0
    /// The lower the dv01_1 the lower the sensitivity to changes in interest rates, by definition. For example,
    /// if the dv01 is .241 then for every change in 1 basis point the price of the value changes by $0.241. Specifically,
    /// if the rates reduce by a basis point the price of the bond *increases* because there is an inverse relationship
    /// betwween the price of a bond at its yield.

    /// ### Additional notes on Duration, Convexity.
    ///
    ///     Yield based duration is a modified version of a Duration and is expressed
    ///
    ///     . D_c0 = T / (100.0 * (f32::pow(1 + y/2), 2T + 1.0))
    ///     . D_c100y = 1.0/y * (1.0  - 1.0 / (f32::pow(1 + y/2.0, 2 * T)))

    ///     . The duration of a bond approximately equals its term.
    ///     . The duration of a par bond increases with term but increases less linearly with term.
    ///     . The duration of a premium bond is less than the duration of a par bond.
    #[derive(Debug, Clone, Copy)]
    pub struct BondMetrics {
        pub dv01: f32,
        pub convexity: f32,
        pub duration: f32,
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
    /// References :
    ///
    ///     . Bond Math, Donald J Smith
    ///     . "Fixed Income Securities, Tools for Today's Markets" (4th ed.), Bruce Tuckman, Angel Serrat

    /// ### Some examples
    ///
    /// Example 1 : Suppose a participant buys a zero coupon corporate bond at 60 but doesn't intend to hold this
    /// bond till maturity, what is the analytics supporting selling the bond after,
    /// for example 2 years, 3 years etc.

    /// How to compute returns on a bond. Consider for example a zero coupon bond selling at 60 expiring in 10 years.
    /// ```rust
    /// let b1 = create_bond(
    /// principal,
    /// "04/15/2021",
    /// "04/15/2051",
    /// 0.0,
    /// "%m/%d/%Y",
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

    /// #### The Yield-to-maturity (ytm)
    /// Yield-to-maturity measures the investors rate of return only if the coupons
    /// are reinvested at the same yield.

    /// ### Zero Coupon Bonds
    /// [Zero Coupon Bonds](https://en.wikipedia.org/wiki/Zero-coupon_bond)
    /// there is only one payment and that is the *Face Value*, *Principal* or *Par Value*.
    /// Computing *ytm* for a zero coupon bond is a simpler expression in contrast with a
    /// coupon bond.
    ///
    /// Yield to Maturity  = f32::powf(Face Value/ Present Value, N) - 1
    ///
    /// Where N is the periodicity of the bond; bonds pay coupons if any, semi-annually. This is true for
    /// most bonds with 10, 20 and 30 years maturity.

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

    impl Eq for DiscountFactor {}
    impl PartialEq for DiscountFactor {
        fn eq(&self, other: &Self) -> bool {
            self.term == other.term
        }
    }

    impl Ord for DiscountFactor {
        fn cmp(&self, other : &Self) -> Ordering {
            if self.term < other.term {
                Ordering::Less
            } else if (self.term - other.term).abs() < f32::EPSILON {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }

    impl PartialOrd for DiscountFactor {
        fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    /// A convenience function that creates a bond with a specific [`Periodicity`]
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
                    principal,
                    issue_date: i_date_unwrapped,
                    maturity_date: maturity_date_unwrapped,
                    coupon_rate: rate,
                    periodicity,
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

    /// Creates a bond with `SemiAnnual` periodicity.
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
                    principal,
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
        /// The coupon payment adjusted to the 'periodicity' of the bond.
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

        /// TODO: Macaulay Duration or Duration is a one-factor metric
        /// for interest rate sensitivity. The duration represents a local percentage change
        /// in price for a corresponding change in rates. Duration is generally represented as a number and
        /// is used to imply the number of time periods and cannot be greater than the maturity of the bond
        /// adjusted to its periodicity.
        pub fn macaulay_duration(self) -> Option<f32> {
            None
        }

        /// The amount of the bond when re-invested at the `reinvestment_interest`
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

        /// The remaining term for the 'Bond'.
        pub fn term_remaining(self, from_date : NaiveDate) -> f32 {
            self.maturity_date.years_since(from_date).unwrap() as  f32
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

        /// Return the baseline price at a `market_price` after `years`.
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

        pub fn get_effective_rate(&self, input : f32) -> f32 {
            match self.periodicity {
                Periodicity::Annual => input,
                Periodicity::SemiAnnual => input / 2.0,
                Periodicity::Quarterly => input / 4.0
            }

        }
        pub fn get_effective_coupon_payment(&self) -> f32 {
            match self.periodicity {
                Periodicity::Annual => self.coupon_rate,
                Periodicity::SemiAnnual => self.coupon_rate / 2.0,
                Periodicity::Quarterly => self.coupon_rate / 4.0
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

    /// Given a table of `MarketData` return a discount factor table to be used for
    /// subsequent computations.
    /// Discount factors for a particular *term* gives the value today of one unit of currency
    /// at the end of that term. For instance a discount factor of 0.999419 at term 1.0 implies that
    /// the current value be discounted by this factor.
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
                debug!("Using intermediate discounts {:?}", inter_sigma);
                let numerator: f32 = md.market_price - inter_sigma;
                let denominator: f32 = 100.00 + (md.coupon_rate / interest_factor);
                let new_value = numerator / denominator;
                debug!(
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

    #[macro_export]
    macro_rules! Issue_Bond {
        [with $principal:ident $issue_date:ident $maturity_date:ident $rate:literal] => {
            create_bond($principal, $issue_date, $maturity_date, $rate, "%m/%d/%Y")
        };
        [with $principal:ident $issue_date:literal $maturity_date:literal $rate:literal] => {
            create_bond($principal, $issue_date, $maturity_date, $rate, "%m/%d/%Y")
        };
        [using $principal:literal $issue_date:literal $maturity_date:literal $rate:literal] => {
            create_bond($principal, $issue_date, $maturity_date, $rate, "%m/%d/%Y")
        };
    }


    #[macro_export]
    macro_rules! Create_Market_Data {
        [with $coupon_rate:literal at term $term:literal @ $price:literal] => {
            MarketData {
             coupon_rate : $coupon_rate,
             term : $term,
             market_price : $price
            }
        }
    }


} // End mod.

#[cfg(test)]
mod tests {
    use crate::Issue_Bond;
    use crate::Create_Market_Data;
    use crate::bond::bond::create_bond;
    use crate::bond::bond::discount_factor;
    use crate::bond::bond::Bond;
    use crate::bond::bond::BondError;
    use crate::bond::bond::DiscountFactor;
    use crate::bond::bond::MarketData;
    use crate::bond::bond::Periodicity;
    use assert_approx_eq::assert_approx_eq;
    use crate::pandl::pandl::BondTransaction;
    use chrono::{Datelike, NaiveDate, ParseError};

    fn create_zcb_principal_maturity(
        principal: f32,
        issue_date: &str,
        mat_date: &str,
    ) -> Result<Bond, BondError> {
        // return create_bond(principal, issue_date, mat_date, 0.0, "%m/%d/%Y");
        return Issue_Bond! (with principal issue_date mat_date 0.0);
    }

    fn create_zcb(principal: f32) -> Result<Bond, BondError> {
        return Issue_Bond! (with principal "04/15/2021" "04/15/2051" 0.0);
    }

    fn create_test_bond() -> Result<Bond, BondError> {
        let principal = 100.0;
        return Issue_Bond! (with principal "04/15/2014" "05/15/2024" 2.5);
    }

    #[test]
    fn test_bond_sort() {
        let b1: Result<Bond, BondError> =
            Issue_Bond!(using 100.0 "04/15/2014" "05/15/2024" 2.5);

        let b2: Result<Bond, BondError> =
            Issue_Bond!(using 100.0 "03/15/2014" "05/15/2024" 2.5);
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

        let md1 = Create_Market_Data!(with 2.875 at term 0.5 @ 101.4297);
        result.push(md1);
        let md2 = Create_Market_Data!(with 2.125 at term 1.0 @ 102.0662);
        result.push(md2);
        let md3 = Create_Market_Data!(with 1.625 at term 1.5 @ 102.2862);
        result.push(md3);
        let md4 = Create_Market_Data!(with 0.125 at term 2.0 @ 99.9538);
        let md5 = Create_Market_Data!(with 0.250 at term 2.5 @ 100.0795);
        let md6 = Create_Market_Data!(with 0.250 at term 3.0 @ 99.7670);
        let md7 = Create_Market_Data!(with 2.250 at term 3.5 @ 106.3091);
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
        assert_approx_eq!(discount_factor[0].discount, 0.9999231, f32::EPSILON);
        assert_approx_eq!(discount_factor[1].discount, 0.99941903, f32::EPSILON);
        assert_approx_eq!(discount_factor[2].discount, 0.9985045, f32::EPSILON);
        assert_approx_eq!(discount_factor[3].discount, 0.99704117, f32::EPSILON);
        assert_approx_eq!(discount_factor[4].discount, 0.9945582, f32::EPSILON);
        assert_approx_eq!(discount_factor[5].discount, 0.99019545, f32::EPSILON);
        assert_approx_eq!(discount_factor[6].discount, 0.9847417, f32::EPSILON);
    }

    #[test]
    fn test_forward_rates() {
        let market_data : Vec<MarketData> = create_test_market_data();
        let mut discount_factor : Vec<DiscountFactor> =
            discount_factor(&market_data, Periodicity::SemiAnnual);
        let b1 = create_zcb(1000.0).unwrap();
        let bt = BondTransaction {
            underlying : b1,
            purchase_date : NaiveDate::parse_from_str("11/13/2020", "%m/%d/%Y").unwrap(),
            purchase_price : 0.0,
            sale_date : NaiveDate::parse_from_str("05/14/2021", "%m/%d/%Y").unwrap(),
            sale_price: 0.0,
            term_rate : Vec::new()
        };
        discount_factor.sort();
        let result = bt.compute_term_rate(&discount_factor);
        assert_approx_eq!(result[0], 0.00015377998);
        assert_approx_eq!(result[1], 0.001008749);
        assert_approx_eq!(result[2], 0.00183177);
        assert_approx_eq!(result[3], 0.0029354095);
        assert_approx_eq!(result[4], 0.004992962);
        assert_approx_eq!(result[5], 0.008811951);
        assert_approx_eq!(result[6], 0.01107645);

    }

    #[test]
    fn test_realized_forwards() {
        let date_format = "%m/%d/%Y";
        let purchase_date = NaiveDate::parse_from_str("11/13/2020", "%m/%d/%Y").unwrap();
        let b1 = create_bond(100.0,
                "11/16/1992", "11/15/2022",
                7.625,
                date_format).unwrap();
        let spread : f32  = -0.000116; // TODO: This needs to be computed separately.
        let mut bt = BondTransaction {
            underlying : b1,
            purchase_date,
            purchase_price : 114.87654,
            sale_date : NaiveDate::parse_from_str("05/14/2021", "%m/%d/%Y").unwrap(),
            sale_price: 114.87654,
            term_rate : Vec::new()
        };
        let market_data : Vec<MarketData> = create_test_market_data();
        let term_remaining = b1.term_remaining(purchase_date);
        assert_approx_eq!(term_remaining, 2.0, f32::EPSILON);
        let result : Vec<f32> = [0.001013, 0.001746, 0.002429, 0.002185].to_vec();
        bt.set_term_rates(&result);
        assert_approx_eq!(111.11555, bt.compute_realized_forwards(1, spread).unwrap(), 0.001);
        let discount_factor : Vec<DiscountFactor> =
            discount_factor(&market_data, Periodicity::SemiAnnual);
        let mut relevant_discount_factors = Vec::new();
        for df in discount_factor {
            if df.term < term_remaining {
                relevant_discount_factors.push(df);
            }
        }
        let result = bt.compute_term_rate(&relevant_discount_factors);
        bt.set_term_rates(&result);
        assert_approx_eq!(111.29847, bt.compute_realized_forwards(0, spread).unwrap(), 0.001);
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
        let b1 = create_bond(100.00, "04/15/2021", "04/15/2051", 6.00, "%m/%d/%Y");
        let constant_yield = 0.2;
        match b1 {
            Result::Ok(val) => val.market_price_trajectory(constant_yield),
            Result::Err(err) => {
                panic!("Failed to create bond {:?}", err);
            }
        }
    }

    fn internal_yield_1() -> Vec<(NaiveDate, f32)> {
        let b1 = create_bond(100.00, "04/15/2021", "04/15/2041", 6.00, "%m/%d/%Y");
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
