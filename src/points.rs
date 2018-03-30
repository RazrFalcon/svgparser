// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use xmlparser::{
    FromSpan,
    Stream,
    StrSpan,
};

use {
    StreamExt,
};


/// Points tokenizer.
///
/// Use it for `points` attribute of `polygon` and `polyline` elements.
#[derive(Clone, Copy, PartialEq)]
pub struct Points<'a>(Stream<'a>);

impl<'a> FromSpan<'a> for Points<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Points(Stream::from_span(span))
    }
}

impl<'a> fmt::Debug for Points<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Points({:?})", self.0.span())
    }
}

impl<'a> Iterator for Points<'a> {
    type Item = (f64, f64);

    /// Extracts next coordinates pair from the stream.
    ///
    /// # Errors
    ///
    /// - Stops on a first invalid character. Follows the same rules as paths tokenizer.
    ///
    /// # Notes
    ///
    /// - If data contains an odd number of coordinates - the last one will be ignored.
    ///   As SVG spec states.
    /// - It doesn't validate that there are more than two coordinates,
    ///   which is required by the SVG spec.
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            let x = match self.0.parse_list_number() {
                Ok(x) => x,
                Err(_) => return None,
            };

            let y = match self.0.parse_list_number() {
                Ok(y) => y,
                Err(_) => return None,
            };

            Some((x, y))
        }
    }
}
