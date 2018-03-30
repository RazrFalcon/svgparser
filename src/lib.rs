// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
*svgparser* is a pull-based parser for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
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
 - [\<viewBox\>](https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute)
 - [\<list-of-points\>](https://www.w3.org/TR/SVG11/shapes.html#PointsBNF)

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
 - All keywords must be lowercase.
   Case-insensitive parsing is supported only for colors (requires allocation for named colors).
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

### Safety

 - The library should not panic. Any panic considered as a critical bug
   and should be reported.
 - The library forbids unsafe code.
*/

#![doc(html_root_url = "https://docs.rs/svgparser/0.7.0")]

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

pub extern crate xmlparser;
extern crate phf;
#[macro_use] extern crate log;
#[macro_use] extern crate failure;


macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None
        }
    }
}


pub mod path;
pub mod style;
pub mod svg;
pub mod transform;

mod aspect_ratio;
mod attribute_id;
mod attribute_value;
mod color;
mod colors;
mod element_id;
mod error;
mod length;
mod points;
mod streamext;
mod value_id;
mod values_list;


pub use aspect_ratio::*;
pub use attribute_id::*;
pub use attribute_value::*;
pub use color::*;
pub use element_id::*;
pub use error::StreamError;
pub use length::*;
pub use points::*;
pub use streamext::*;
pub use value_id::*;
pub use values_list::*;
