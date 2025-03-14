mod tbills {

    use chrono::NaiveDate;

    #[derive(Clone, Copy)]
    pub enum TimeIntervalType {
        Days,
        Weeks,
        Months,
    }

    /// T-Bills or Treasury Bills are securities with a shorter maturity period,
    /// typically less than 2 years. The coupon payments could be either
    /// * 13 weeks
    /// * 26 weeks
    ///
    pub struct TBills {
        pub issue_date: NaiveDate,
        pub face_value: f32,
        pub time_interval_type: TimeIntervalType,
        pub discount_rate: f32,
        pub time: f32,
        pub maturity_date: NaiveDate,
    }

    impl TBills {
        pub fn valuation(&self) -> f32 {
            match self.time_interval_type {
                TimeIntervalType::Weeks => {
                    self.face_value
                        * (1.0 - (self.time * 7.0) * (self.discount_rate / (100.0 * 360.0)))
                }
                TimeIntervalType::Days => {
                    self.face_value * (1.0 - (self.time) * (self.discount_rate / (100.0 * 360.0)))
                }
                TimeIntervalType::Months => {
                    self.face_value
                        * (1.0 - (self.time * 30.0) * (self.discount_rate / (100.0 * 360.0)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use chrono::NaiveDate;
    use tbills::TBills;
    use tbills::TimeIntervalType;

    #[test]
    fn test_simple_price() {
        let i: NaiveDate =
            NaiveDate::parse_from_str(&String::from("01/13/2025"), "%m/%d/%Y").unwrap();
        let m: NaiveDate =
            NaiveDate::parse_from_str(&String::from("04/14/2025"), "%m/%d/%Y").unwrap();
        let v: TBills = TBills {
            issue_date: i,
            face_value: 1000.0,
            time_interval_type: TimeIntervalType::Weeks,
            discount_rate: 0.145,
            time: 26.0,
            maturity_date: m,
        };
        assert_approx_eq!(999.27, v.valuation(), 0.01);
    }

    #[test]
    fn test_simple_price_2() {
        let i: NaiveDate = NaiveDate::parse_from_str("01/13/2025", "%m/%d/%Y").unwrap();
        let m: NaiveDate = NaiveDate::parse_from_str("04/14/2025", "%m/%d/%Y").unwrap();

        let v: TBills = TBills {
            issue_date: i,
            face_value: 1000.0,
            time_interval_type: TimeIntervalType::Days,
            discount_rate: 0.145,
            time: 26.0 * 7.0,
            maturity_date: m,
        };
        assert_approx_eq!(999.27, v.valuation(), 0.01);
    }

    #[test]
    fn test_simple_price_3() {
        let i: NaiveDate = NaiveDate::parse_from_str("01/13/2025", "%m/%d/%Y").unwrap();
        let m: NaiveDate = NaiveDate::parse_from_str("04/14/2025", "%m/%d/%Y").unwrap();

        let v: TBills = TBills {
            issue_date: i,
            face_value: 1000.0,
            time_interval_type: TimeIntervalType::Months,
            discount_rate: 0.145,
            time: 26.0 * 7.0 / 30.0,
            maturity_date: m,
        };
        assert_approx_eq!(999.27, v.valuation(), 0.01);
    }
}
