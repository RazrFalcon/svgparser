// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    FromSpan,
    Stream,
    StreamExt,
    StrSpan,
};


/// Points tokenizer.
///
/// Use it for `points` attribute of `polygon` and `polyline` elements.
pub struct Points<'a>(Stream<'a>);

impl<'a> FromSpan<'a> for Points<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Points(Stream::from_span(span))
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
