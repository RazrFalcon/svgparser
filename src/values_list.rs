// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use {
    Error,
    FromSpan,
    Length,
    Stream,
    StreamExt,
    StrSpan,
};

/// Iterator over a list of [`<number>`] values.
/// [`<number>`]: https://www.w3.org/TR/SVG/types.html#DataTypeNumber
#[derive(Copy, Clone, PartialEq)]
pub struct NumberList<'a>(Stream<'a>);

impl<'a> FromSpan<'a> for NumberList<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        NumberList(Stream::from_span(span))
    }
}

impl<'a> fmt::Debug for NumberList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NumberList({:?})", self.0.span())
    }
}

impl<'a> Iterator for NumberList<'a> {
    type Item = Result<f64, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            Some(self.0.parse_list_number())
        }
    }
}

/// Iterator over a list of [`<length>`] values.
/// [`<length>`]: https://www.w3.org/TR/SVG/types.html#DataTypeLength
#[derive(Copy, Clone, PartialEq)]
pub struct LengthList<'a>(Stream<'a>);

impl<'a> LengthList<'a> {
    /// Constructs a new `LengthList` from `StrSpan`.
    pub fn from_span(span: StrSpan<'a>) -> LengthList<'a> {
        LengthList(Stream::from_span(span))
    }
}

impl<'a> fmt::Debug for LengthList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LengthList({:?})", self.0.span())
    }
}

impl<'a> Iterator for LengthList<'a> {
    type Item = Result<Length, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.at_end() {
            None
        } else {
            Some(self.0.parse_list_length())
        }
    }
}
