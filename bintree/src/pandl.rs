mod pandl {
    use crate::bond::bond::Bond;
    use chrono::NaiveDate;

    enum RealizedForwards {
        RealizedForwards,
        UnrealizedForwards,
    }

    enum Attribution {
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

    // term could be one of
    // the 6M, 1Y, 2Y .... Term of the bond.
    pub struct TermStructure {
        pub bond: Bond,
        pub starting_period: NaiveDate,
        pub ending_period: NaiveDate,
        pub realized_forwards: Option<RealizedForwards>,
        pub term: f32, // One of the term values for the bond.
    }

    impl PriceStructure {
        pub fn change(&self) -> f32 {
            return 0.0;
        }
    }

    /// Begin with a simple example of an investor
    /// buys a US 7.625s of 11/15/2022 at 114.8765 on
    /// Nov 14th, 2020. Later on May 2021 the price of the bond
    /// is 111.3969. Compute the realized returns.
    #[derive(Debug, Clone, Copy)]
    pub struct BondTransaction  {
      pub underlying : Bond,
      pub purchaseDate : NaiveDate,
      pub purchasePrice : f32,
      pub saleDate : NaiveDate,
      pub salePrice : NaiveDate
    }



}
