mod interest_rate_swap {
    use crate::rates::rates::OvernightRateType;
    use chrono::NaiveDate;
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
    pub enum AccountingConvention {
        AC360,
        AC365,
    }

    #[derive(Debug)]
    pub struct IRS {
        pub face_value: f32,
        pub fixed_rate: f32,
        pub overnight_rate_type: OvernightRateType,
        pub time: f32,
        pub accounting_convention: AccountingConvention,
    }

    #[derive(Debug)]
    pub struct InterestRateData {
        pub time: NaiveDate,
        pub rate: f32,
        pub overnight_rate_type: OvernightRateType,
    }

    impl PartialOrd for InterestRateData {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for InterestRateData {
        fn cmp(&self, other: &Self) -> Ordering {
            self.time.cmp(&other.time)
        }
    }

    impl PartialEq for InterestRateData {
        fn eq(&self, other: &Self) -> bool {
            self.time == other.time
        }
    }
    impl Eq for InterestRateData {}

    fn compute_variable_side(irs: &IRS, overnight_data: &Vec<InterestRateData>, days: f32) -> f32 {
        let mut result: f32 = 0.0;
        let days_in_year: f32 = match irs.accounting_convention {
            AccountingConvention::AC360 => 360.0,
            AccountingConvention::AC365 => 365.0,
        };
        for i in overnight_data {
            if irs.overnight_rate_type != i.overnight_rate_type {
                panic!(
                    "Mismatched rate type irs : {:?}, market_data : {:?}",
                    irs.overnight_rate_type, i.overnight_rate_type
                );
            }
            if (result - 0.0).abs() < f32::EPSILON {
                result = irs.face_value * (i.rate / (days_in_year * 100.0));
            } else {
                result = result * (1.0 + i.rate / (days_in_year * 100.0));
            }
        }
        return result;
    }

    pub fn price_irs_at(irs: &IRS, overnight_data: &Vec<InterestRateData>, days: f32) -> f32 {
        match irs.accounting_convention {
            AccountingConvention::AC360 => {
                let fixed_side: f32 = irs.face_value
                    * (1.0 + (irs.fixed_rate / 100.0) * days / 360.0)
                    - irs.face_value;
                let variable_side: f32 = compute_variable_side(irs, overnight_data, days);
                println!(
                    "Variable side {:?} fixed_side {:?}",
                    variable_side, fixed_side
                );
                return variable_side - fixed_side;
            }
            AccountingConvention::AC365 => {
                let fixed_side: f32 =
                    irs.face_value * (1.0 + (irs.fixed_rate / 100.0) * days / 365.0);
                let variable_side: f32 = (irs.face_value
                    * compute_variable_side(irs, overnight_data, days))
                    - irs.face_value;
                return variable_side - fixed_side;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rates::rates::OvernightRateType;
    use assert_approx_eq::assert_approx_eq;
    use chrono::{Days, NaiveDate};
    use interest_rate_swap::price_irs_at;
    use interest_rate_swap::AccountingConvention;
    use interest_rate_swap::InterestRateData;
    use interest_rate_swap::IRS;

    #[test]
    fn test_price_irs() {
        let mut interest_rate_data = Vec::new();
        let mut start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let mut v1: u32 = 0;
        let mut v2: u32 = 0;
        let mut v3: u32 = 0;
        for day in 1..366 {
            if day >= 1 && day <= 5 {
                let ir: InterestRateData = InterestRateData {
                    time: start_date,
                    rate: 0.1,
                    overnight_rate_type: OvernightRateType::SOFR,
                };
                v1 = v1 + 1;
                interest_rate_data.push(ir);
            } else if day > 5 && day <= 175 {
                let ir: InterestRateData = InterestRateData {
                    time: start_date,
                    rate: 0.5,
                    overnight_rate_type: OvernightRateType::SOFR,
                };
                v2 = v2 + 1;
                interest_rate_data.push(ir);
            } else if day > 175 && day <= 366 {
                let ir: InterestRateData = InterestRateData {
                    time: start_date,
                    rate: 0.01,
                    overnight_rate_type: OvernightRateType::SOFR,
                };
                v3 = v3 + 1;
                interest_rate_data.push(ir);
            }
            start_date = start_date + Days::new(1);
        }
        interest_rate_data.sort();
        println!("5 : {:?}, 170 : {:?}, 190 : {:?}", v1, v2, v3);
        let irs: IRS = IRS {
            face_value: 100000000.00,
            fixed_rate: 0.1120,
            overnight_rate_type: OvernightRateType::SOFR,
            time: 2.0,
            accounting_convention: AccountingConvention::AC360,
        };
        let valuation: f32 = price_irs_at(&irs, &mut interest_rate_data, 365.0);
        assert_approx_eq!(valuation, -113281.55, 1.0);
    }

    #[test]
    fn test_price_irs_0() {
        let mut interest_rate_data = Vec::new();
        let mut start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let mut v1: u32 = 0;
        for _day in 0..10 {
            let ir: InterestRateData = InterestRateData {
                time: start_date,
                rate: 0.1120,
                overnight_rate_type: OvernightRateType::SOFR,
            };
            v1 = v1 + 1;
            interest_rate_data.push(ir);
            start_date = start_date + Days::new(1);
        }
        interest_rate_data.sort();
        let irs: IRS = IRS {
            face_value: 100000000.00,
            fixed_rate: 0.1120,
            overnight_rate_type: OvernightRateType::SOFR,
            time: 2.0,
            accounting_convention: AccountingConvention::AC360,
        };
        let valuation: f32 = price_irs_at(&irs, &mut interest_rate_data, 1.0);
        assert_approx_eq!(valuation, 0.00, 1.0);
    }
}
