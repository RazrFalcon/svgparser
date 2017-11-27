// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str;

use error_chain;
use xmlparser;

use {
    ErrorPos,
};


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        Xml(xmlparser::Error, xmlparser::ErrorKind) #[doc = "xmlparser errors"];
        Stream(xmlparser::StreamError, xmlparser::StreamErrorKind) #[doc = "'Stream' errors"];
    }

    errors {
        /// An invalid number.
        InvalidNumber(pos: ErrorPos) {
            display("invalid number at {}", pos)
        }

        /// An invalid length.
        InvalidLength(pos: ErrorPos) {
            display("invalid length at {}", pos)
        }

        /// An invalid color.
        InvalidColor(pos: ErrorPos) {
            display("invalid color at {}", pos)
        }

        /// An invalid transform.
        InvalidTransform(pos: ErrorPos) {
            display("invalid transform at {}", pos)
        }

        /// An invalid attribute value.
        InvalidAttributeValue(pos: ErrorPos) {
            display("invalid attribute value at {}", pos)
        }
    }
}


/// `ChainedError` additional methods.
pub trait ChainedErrorExt {
    /// Shorthand for `display_chain().to_string().trim()`.
    fn full_chain(&self) -> String;
}

impl<T: error_chain::ChainedError> ChainedErrorExt for T {
    fn full_chain(&self) -> String {
        self.display_chain().to_string().trim().to_owned()
    }
}
