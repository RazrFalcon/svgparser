// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Streaming parser/tokenizer for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
//! data format without heap allocations.
//!
//! Checkout README.md for general highlights.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate phf;

pub use attribute::AttributeId;
pub use attribute_value::{AttributeValue, PaintFallback};
pub use rgbcolor::RgbColor;
pub use element::ElementId;
pub use error::{Error, ErrorPos};
pub use length::{Length, LengthUnit};
pub use stream::Stream;
pub use value_id::ValueId;
pub use values_list::{NumberList, LengthList};

// TODO: maybe use from_utf8_unchecked() as an option
/// `str::from_utf8($text).unwrap()`
#[macro_export]
macro_rules! u8_to_str {
    ($text:expr) => (str::from_utf8($text).unwrap())
}

pub mod path;
pub mod style;
pub mod svg;
pub mod transform;

mod attribute;
mod attribute_value;
mod colors;
mod element;
mod error;
mod length;
mod rgbcolor;
mod stream;
mod value_id;
mod values_list;
