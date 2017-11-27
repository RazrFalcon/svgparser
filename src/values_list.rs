// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    Error,
    Length,
    Stream,
    StreamExt,
    StrSpan,
};

/// Iterator over a list of [`<number>`] values.
/// [`<number>`]: https://www.w3.org/TR/SVG/types.html#DataTypeNumber
#[derive(Copy, Clone, PartialEq)]
pub struct NumberList<'a>(Stream<'a>);

impl<'a> NumberList<'a> {
    /// Constructs a new `NumberList` from `StrSpan`.
    pub fn from_span(span: StrSpan<'a>) -> NumberList<'a> {
        NumberList(Stream::from_span(span))
    }

    /// Returns an underling string.
    pub fn data(&self) -> &str {
        self.0.span().to_str()
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

    /// Returns an underling string.
    pub fn data(&self) -> &str {
        self.0.span().to_str()
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
