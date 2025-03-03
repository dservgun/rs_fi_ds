mod pandl {
    use crate::bond::bond::Bond;
    use chrono::NaiveDate;

    pub enum RealizedForwards {
        RealizedForwards,
        UnrealizedForwards,
    }

    pub enum Attribution {
        CashCarry,
        CashRollDown,
        Rates,
        Spread,
    }

    pub struct PriceStructure {
        pub pricing_date: NaiveDate,
        pub term_structure: TermStructure,
        pub spread: f32,
        pub price: f32,
    }

    /// The term structure associated with an attribution.
    pub struct TermStructure {
        pub starting_period: NaiveDate,
        pub term: f32, // One of the term values for the bond.
        pub spot_rate: f32,
    }

    /// The profit and loss entry for a PandL report.
    pub struct PandLEntry {
        pub pricing_date: NaiveDate,
        pub term_structure: Vec<TermStructure>,
    }

    /// The pand report for a bond at a given date.
    pub struct PandL {
        pub bond: Bond,
        pub asof: NaiveDate,
        pub attribution: Vec<PandLEntry>,
    }

    impl PriceStructure {
        pub fn change(&self) -> f32 {
            return 0.0;
        }
    }

    /// Given a term structure, return a
    pub fn forward_term_structure(
        structure: Vec<TermStructure>,
        input: NaiveDate,
    ) -> Vec<TermStructure> {
        return Vec::new();
    }

    /// Begin with a simple example of an investor
    /// buys a US 7.625s of 11/15/2022 at 114.8765 on
    /// Nov 14th, 2020. Compute the price on May 2021.
    #[derive(Debug, Clone, Copy)]
    pub struct BondTransaction {
        pub underlying: Bond,
        pub purchase_date: NaiveDate,
        pub purchase_price: f32,
        pub sale_date: NaiveDate,
        pub sale_price: f32,
    }

    impl BondTransaction {
        /// Returns the realized returns in percentage points.
        pub fn compute_realized_return(&self) -> f32 {
            let cashflows = self
                .underlying
                .cashflow_between_inclusive(self.purchase_date, self.sale_date);
            let cashflows_sum = cashflows.iter().fold(0.0, |mut sum, val| {
                sum += val.amount;
                sum
            });
            let reinvestment_amounts = self
                .underlying
                .reinvestment_amount_between(self.purchase_date, self.sale_date);
            let reinvestment_amount_sum = reinvestment_amounts.iter().fold(0.0, |mut sum, val| {
                sum += val;
                sum
            });

            println!("Reinvestment amount {:?}", reinvestment_amounts);
            println!("Cash flows {:?}", cashflows);
            println!("Cashflows sum {:?}", cashflows_sum);
            println!("Transaction Sale price {:?} : Cashflows : {:?} Reinvestment amounts {:?} Purchase price : {:?}",
            self.sale_price, cashflows_sum, reinvestment_amount_sum, self.purchase_price);
            println!(
                "Payoff : {:?} - Purchase price {:?}",
                self.sale_price + cashflows_sum + reinvestment_amount_sum,
                self.purchase_price
            );
            return (self.sale_price + cashflows_sum + reinvestment_amount_sum
                - self.purchase_price)
                / self.purchase_price;
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use chrono::NaiveDate;

    use crate::bond::bond::*;
    use crate::pandl::pandl::*;

    fn create_test_bond(interest: f32) -> Result<Bond, BondError> {
        return create_bond_with_periodicity(
            100.0,
            String::from("11/15/2012").as_str(),
            String::from("11/15/2022").as_str(),
            0.07625,
            interest,
            Periodicity::SemiAnnual,
            String::from("%m/%d/%Y").as_str(),
        );
    }

    #[test]
    fn test_realized_return() {
        let b1 = create_test_bond(0.00);
        let date_format = "%m/%d/%Y";
        let p_date_opt = NaiveDate::parse_from_str("11/01/2020", date_format);
        let s_date_opt = NaiveDate::parse_from_str("4/15/2021", date_format);
        match (b1, p_date_opt, s_date_opt) {
            (Result::Ok(val), Result::Ok(purchase_date), Result::Ok(sale_date)) => {
                let bond_transaction: BondTransaction = BondTransaction {
                    underlying: val,
                    purchase_date: purchase_date,
                    purchase_price: 114.8765,
                    sale_date: sale_date,
                    sale_price: 111.3969,
                };
                assert_approx_eq!(bond_transaction.compute_realized_return(), 0.002897, 0.0001);
            }
            (_, _, _) => {
                panic!("Failed testing realized return")
            }
        }
    }

    #[test]
    fn test_realized_return_2() {
        let b1 = create_test_bond(0.05);
        let date_format = "%m/%d/%Y";
        let p_date_opt = NaiveDate::parse_from_str("11/15/2020", date_format);
        let s_date_opt = NaiveDate::parse_from_str("5/15/2021", date_format);
        match (b1, p_date_opt, s_date_opt) {
            (Result::Ok(val), Result::Ok(purchase_date), Result::Ok(sale_date)) => {
                let bond_transaction: BondTransaction = BondTransaction {
                    underlying: val,
                    purchase_date: purchase_date,
                    purchase_price: 114.8765,
                    sale_date: sale_date,
                    sale_price: 108.00,
                };
                assert_approx_eq!(bond_transaction.compute_realized_return(), 0.0073, 0.0001);
            }
            (_, _, _) => {
                panic!("Failed testing realized return")
            }
        }
    }
}
