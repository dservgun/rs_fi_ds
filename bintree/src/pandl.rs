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
        pub pricingDate: NaiveDate,
        pub termStructure: TermStructure,
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
}
