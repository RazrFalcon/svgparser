// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
