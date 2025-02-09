use chrono::{Months, NaiveDate, ParseError};
use std::cmp::{PartialEq, PartialOrd};
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
enum PaymentSchedule {
    Quarterly,
    SemiAnnual,
    Annual,
}

#[derive(Debug, Clone, Copy)]
enum ErrorType {
    InvalidDate,
    InvalidRate,
    InvalidPrincipal,
}

#[derive(Debug, Clone)]
struct BondError {
    message: String,
    message_code: ErrorType,
}

#[derive(Debug, Clone, Copy)]
struct Bond {
    principal: f32,
    issue_date: NaiveDate,
    maturity_date: NaiveDate,
    coupon_rate: f32,
    payment_schedule: PaymentSchedule,
}

#[derive(Debug, Clone, Copy)]
struct CashFlow {
    bond: Bond,
    time: NaiveDate,
    amount: f32,
}

impl PartialEq for CashFlow {
  fn eq(&self, other : &Self) -> bool {
    return self.time == other.time &&
    (f32::EPSILON < (self.amount - other.amount).abs());
  }
}

// the principal,
// the term amount.
// the term rate.
// Need a struct to capture this, problem 2

impl Bond {
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
                    payment_schedule: PaymentSchedule::SemiAnnual,
                };
                return Ok(b1);
            }
            _ => {
                return Err(BondError {
                    message: String::from("Invalid date"),
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
        let intervals = self.payment_intervals();
        let mut result = Vec::new();
        for coupon_time in &intervals {
            let cashflow: CashFlow = CashFlow {
                bond: self.clone(),
                time: coupon_time.clone(),
                amount: self.coupon_payment(),
            };
            result.push(cashflow);
        }
        return result;
    }
}

/// Test code
mod tests {
    use super::*;

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
                let cashflows = val.cashflow();
                for i in cashflows {
                  assert_approx_eq!(i.amount, 125.0, f32::EPSILON);
                }

            }
            Result::Err(_) => {
                panic!("Failed to create bond.");
            }
        }
    }
}
