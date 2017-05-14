// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Streaming parser/tokenizer for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
//! data format without heap allocations.
//!
//! Checkout README.md for general highlights.

#![doc(html_root_url = "https://docs.rs/svgparser/0.4.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![deny(unused_import_braces)]

extern crate phf;

pub use attribute_id::AttributeId;
pub use attribute_value::{AttributeValue, PaintFallback};
pub use color::Color;
pub use element_id::ElementId;
pub use error::{Error, ErrorPos};
pub use length::{Length, LengthUnit};
pub use stream::{TextFrame, Stream};
pub use tokenize::Tokenize;
pub use value_id::ValueId;
pub use values_list::{NumberList, LengthList};

#[macro_export]
macro_rules! warnln {
    ($msg:expr) => ({
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), "Warning: {}", $msg).unwrap()
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), concat!("Warning: ", $fmt), $($arg)*).unwrap()
    });
}

pub mod path;
pub mod style;
pub mod svg;
pub mod transform;

mod attribute_id;
mod attribute_value;
mod color;
mod colors;
mod element_id;
mod error;
mod length;
mod stream;
mod tokenize;
mod value_id;
mod values_list;

// TODO: add a prelude module
