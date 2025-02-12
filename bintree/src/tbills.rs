mod tbills {
    use chrono::NaiveDate;
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
    use crate::bond::bond::Bond;

    #[derive(Clone, Copy)]
    pub enum TimeIntervalType {
        Days,
        Weeks,
        Months,
    }

    pub struct TBills {
        pub face_value: f32,
        pub time_interval_type: TimeIntervalType,
        pub discount_rate: f32,
        pub time: f32,
    }

    impl TBills {
        pub fn valuation(&self) -> f32 {
            match (self.time_interval_type) {
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
    use chrono::{Days, NaiveDate};
    use tbills::TBills;
    use tbills::TimeIntervalType;
    #[test]
    fn test_simple_price() {
        let v: TBills = TBills {
            face_value: 1000.0,
            time_interval_type: TimeIntervalType::Weeks,
            discount_rate: 0.145,
            time: 26.0,
        };

        assert_approx_eq!(999.27, v.valuation(), 0.01);
    }
}
