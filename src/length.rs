// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// List of all SVG length units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum LengthUnit {
    None,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Percent,
}

/// Representation of the [`<length>`] type.
/// [`<length>`]: https://www.w3.org/TR/SVG/types.html#DataTypeLength
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Length {
    #[allow(missing_docs)]
    pub num: f64,
    #[allow(missing_docs)]
    pub unit: LengthUnit,
}

impl Length {
    /// Constructs a new length.
    pub fn new(num: f64, unit: LengthUnit) -> Length {
        Length {
            num: num,
            unit: unit,
        }
    }
}
