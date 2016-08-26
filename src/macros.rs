// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! impl_iter_for_tokenizer {
    ($t:ty) => (
        impl<'a> Iterator for Tokenizer<'a> {
            type Item = Result<$t, Error>;
            fn next(&mut self) -> Option<Self::Item> {
                match self.parse_next() {
                    Ok(v) => Some(Ok(v)),
                    Err(e) => {
                        match e {
                            Error::EndOfStream => None,
                            _ => Some(Err(e)),
                        }
                    }
                }
            }
        }
    )
}

/// `str::from_utf8($text).unwrap()`
#[macro_export]
macro_rules! u8_to_str {
    ($text:expr) => (str::from_utf8($text).unwrap())
}
