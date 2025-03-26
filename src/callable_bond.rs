pub mod callable_bonds {
    use crate::bond::bond::Bond;
    use chrono::{Datelike, Months, NaiveDate, ParseError};
    use filters::filter::Filter;
    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

    /// A callable bond allows an issuer to purchase debt at
    /// favorable terms at various between the issue date and the
    /// maturity of the bond.
    /// The price of the underlying bond would depend on whether the
    /// issuer would exercise the call. The value of a callable bond
    /// will change depending on how the value of embedded options changes as
    /// interest rates change.
    pub struct CallableBond {
        pub underlying: Bond,
        pub callable_structure: Vec<CallPrice>,
    }

    /// The tuple of the `call_start` - the start date of the schedule.
    /// The `call_end` the end date for the option and the call price.
    /// A callable bond's price is composed of twwo components
    /// `
    /// Price of a callable bond = Price of the option-free bond -
    ///     price of the embedded call option`
    /// The reason being that an embedded call option benefits the issuer rather than
    /// the entity purchasing the bond.
    /// In this case, when the interest rates decline the price of the option-free bonds
    /// increase, however the price of the callable bond increases because the callable bond
    /// becomes more valuable to the issuer.
    /// `
    /// Note: The call price here is expressed as the price of the bond, though,
    /// this could also be expressed as yield.
    /// `
    pub struct CallPrice {
        pub call_start: NaiveDate,
        pub call_end: NaiveDate,
        pub call_price: f32,
    }
}
