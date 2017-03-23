// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Streaming parser/tokenizer for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
//! data format without heap allocations.
//!
//! Checkout README.md for general highlights.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![deny(unused_import_braces)]

extern crate phf;

pub use attribute_id::AttributeId;
pub use attribute_value::{AttributeValue, PaintFallback};
pub use rgbcolor::RgbColor;
pub use element_id::ElementId;
pub use error::{Error, ErrorPos};
pub use length::{Length, LengthUnit};
pub use stream::Stream;
pub use value_id::ValueId;
pub use values_list::{NumberList, LengthList};

pub mod path;
pub mod style;
pub mod svg;
pub mod transform;

mod attribute_id;
mod attribute_value;
mod colors;
mod element_id;
mod error;
mod length;
mod rgbcolor;
mod stream;
mod value_id;
mod values_list;
