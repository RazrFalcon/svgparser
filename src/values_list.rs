// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Stream, Length, Error};

/// Iterator over list of \<number\> types.
#[derive(Copy,Clone,PartialEq)]
pub struct NumberList<'a>(pub Stream<'a>);

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

/// Iterator over list of \<length\> types.
#[derive(Copy,Clone,PartialEq)]
pub struct LengthList<'a>(pub Stream<'a>);

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
