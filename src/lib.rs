// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
*libsvgparser* is a streaming parser/tokenizer for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
data format without heap allocations.

It's not an XML parser since it does not only split the content into the XML nodes,
but also supports [SVG types](https://www.w3.org/TR/SVG/types.html#BasicDataTypes) parsing.

### Supported SVG types
 - [\<color\>](https://www.w3.org/TR/SVG/types.html#DataTypeColor)
 - [\<paint\>](https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint)
 - [\<path\>](https://www.w3.org/TR/SVG/paths.html#PathData)
 - [\<number\>](https://www.w3.org/TR/SVG/types.html#DataTypeNumber) and \<list-of-numbers\>
 - [\<length\>](https://www.w3.org/TR/SVG/types.html#DataTypeLength) and \<list-of-lengths\>
 - [\<coordinate\>](https://www.w3.org/TR/SVG/types.html#DataTypeCoordinate)
 - [\<IRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeIRI)
 - [\<FuncIRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI)
 - [\<transform-list\>](https://www.w3.org/TR/SVG/types.html#DataTypeTransformList)
 - [\<style\>](https://www.w3.org/TR/SVG/styling.html#StyleAttribute)

### Benefits
 - Most of the common data parsed into internal representation, and not just as string
   (unlike typical XML parser). Tag names, attribute names, attributes value, etc.
 - Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
 - [Predefined SVG values](https://www.w3.org/TR/SVG/propidx.html) for presentation attributes,
   like `auto`, `normal`, `none`, `inherit`, etc. are parsed as `enum`, not as `String`.
 - Every type can be parsed separately, so you can parse just paths or transform
   or any other SVG value.
 - Good error processing. All error types contain position (line:column) where it occurred.
 - No heap allocations.
 - Pretty fast.

### Limitations
 - All keywords must be lowercase. Case-insensitive parsing is not supported.
   Still, it's extremely rare.
 - The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
 - Only ENTITY objects are parsed from the DOCTYPE. Other ignored.
 - CSS styles does not processed. You should use an external CSS parser.
 - Comments inside attributes value supported only for the `style` attribute.
 - [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`, are not supported.
 - There is no separate `opacity-value` type. It will be parsed as `<number>`,
   but will be bound to 0..1 range.
 - Implicit path commands are not supported. All commands are parsed as explicit.
 - Implicit MoveTo commands will be automatically converted into explicit LineTo.
 - No escape support for text. It will be emitted as is.

### Differences between *libsvgparser* and SVG spec
 - `<percentage>` type is part of the `<length>` type.
*/

#![doc(html_root_url = "https://docs.rs/svgparser/0.4.3")]

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

/// Prints warnings into stderr.
#[macro_export]
macro_rules! warnln {
    ($msg:expr) => ({
        use std::io::Write; // TODO: remove
        writeln!(&mut ::std::io::stderr(), "Warning: {}", $msg).unwrap()
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write; // TODO: remove
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
