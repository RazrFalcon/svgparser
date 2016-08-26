// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

/// Position of the error.
///
/// Position indicates row/line and column. Starting positions is 1:1.
#[derive(Clone,Copy,PartialEq)]
pub struct ErrorPos {
    #[allow(missing_docs)]
    pub row: usize,
    #[allow(missing_docs)]
    pub col: usize,
}

impl ErrorPos {
    /// Constructs a new error position.
    pub fn new(row: usize, col: usize) -> ErrorPos {
        ErrorPos {
            row: row,
            col: col,
        }
    }
}

impl fmt::Debug for ErrorPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", &self.row, &self.col)
    }
}

/// List of all supported errors.
#[derive(Clone,Copy,PartialEq)]
pub enum Error {
    /// Technically, `EndOfStream` is not an error.
    /// It's just indicates reaching the end of the stream.
    EndOfStream,
    /// The steam ended earlier than we expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in valid SVG should be handled by errors below.
    UnexpectedEndOfStream(ErrorPos),
    /// Can appear during consuming expected char.
    InvalidChar {
        /// Current char in stream.
        current: char,
        /// Expected char.
        expected: char,
        /// Absolute stream position.
        pos: ErrorPos,
    },
    /// Invalid SVG/XML token.
    InvalidSvgToken(ErrorPos),
    /// Error during a number parsing.
    InvalidNumber(ErrorPos),
    /// Error during a color parsing.
    InvalidColor(ErrorPos),
    /// Error during a transform parsing.
    InvalidTransform(ErrorPos),
    /// Invalid attribute value.
    InvalidAttributeValue(ErrorPos),
    /// Can appear during moving along the data stream.
    AdvanceError {
        /// Advance step.
        expected: isize,
        /// Full length of the steam.
        total: usize,
        /// Absolute stream position.
        pos: ErrorPos,
    },
    /// SVG Element must contain tag name.
    ElementWithoutTagName(ErrorPos),
}

impl fmt::Debug for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EndOfStream => write!(f, "End of stream"),
            Error::UnexpectedEndOfStream(ref pos) =>
                write!(f, "Unexpected end of stream at: {:?}", pos),
            Error::InvalidChar{ref current, ref expected, ref pos} =>
                write!(f, "Expected '{}', found '{}' at pos: {:?}", expected, current, pos),
            Error::InvalidSvgToken(ref pos) => write!(f, "Invalid SVG token at: {:?}", pos),
            Error::InvalidNumber(ref pos) => write!(f, "Invalid number at: {:?}", pos),
            Error::InvalidColor(ref pos) => write!(f, "Invalid color at: {:?}", pos),
            Error::InvalidTransform(ref pos) => write!(f, "Invalid transform at: {:?}", pos),
            Error::InvalidAttributeValue(ref pos) => {
                write!(f, "Invalid attribute at: {:?}", pos)
            }
            Error::AdvanceError{ref expected, ref total, ref pos} =>
                write!(f, "Attempt to advance to pos {} from {:?}, but total len is {}",
                       expected, pos, total),
            Error::ElementWithoutTagName(ref pos) =>
                write!(f, "Element without tag name at: {:?}", pos),
        }
    }
}
