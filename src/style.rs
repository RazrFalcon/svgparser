// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<style>`] data.
//!
//! [`<style>`]: https://www.w3.org/TR/SVG/styling.html#StyleAttribute

use std::fmt;
use std::str;

use super::{Stream, Error};

/// Style token.
#[derive(PartialEq)]
pub enum Token<'a> {
    /// Tuple contains attribute name and value.
    Attribute(&'a [u8], Stream<'a>),
    /// Tuple contains ENTITY reference. Just a name without `&` and `;`.
    EntityRef(&'a [u8]),
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Attribute(name, ref value) => {
                write!(f, "Token({}, {:?})", u8_to_str!(name), value)
            }
            Token::EntityRef(name) => {
                write!(f, "EntityRef({})", u8_to_str!(name))
            }
        }
    }
}

/// Tokenizer for \<style\> data.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
}

impl<'a> Tokenizer<'a> {
    /// Constructs a new `Tokenizer`.
    pub fn new(stream: Stream<'a>) -> Tokenizer<'a> {
        Tokenizer { stream: stream }
    }

    /// Extracts next style object from the stream.
    ///
    /// # Errors
    ///
    /// - `Error::EndOfStream` indicates end of parsing, not error.
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - By SVG spec `style` attribute can contain any style sheet language,
    ///   but we only support CSS2, which is the default.
    /// - Objects with `-` prefix will be ignored since we can't write them as XML attributes.
    ///   Library will print a warning to stdout.
    /// - All comments are automatically skipped.
    pub fn parse_next(&mut self) -> Result<Token<'a>, Error> {
        self.stream.skip_spaces();

        if self.stream.at_end() {
            return Err(Error::EndOfStream);
        }

        // skip comments inside attribute value
        if try!(self.stream.is_char_eq(b'/')) {
            try!(self.stream.advance(2)); // skip /*
            try!(self.stream.jump_to(b'*'));
            try!(self.stream.advance(2)); // skip */
            self.stream.skip_spaces();
        }

        // prefixed attributes are not supported, aka '-webkit-*'
        if try!(self.stream.is_char_eq(b'-')) {
            let l = self.stream.len_to_char_or_end(b';');
            println!("Warning: Style attribute '{}' is skipped.",
                     u8_to_str!(self.stream.slice_next_raw(l)));

            self.stream.advance_raw(l);
            if !self.stream.at_end() {
                self.stream.advance_raw(1);
            }
            return self.parse_next();
        }

        if try!(self.stream.is_char_eq(b'&')) {
            // extract 'text' from '&text;'
            try!(self.stream.advance(1)); // &
            let len = self.stream.len_to_space_or_end() - 1; // ;
            let name = self.stream.read_raw(len);
            try!(self.stream.advance(1));

            return Ok(Token::EntityRef(name));
        }

        let name = try!(self.stream.read_to(b':'));

        try!(self.stream.advance(1)); // ':'
        self.stream.skip_spaces();

        // skip start quote
        if try!(self.stream.is_char_eq(b'\'')) {
            try!(self.stream.advance(1));
        }

        let mut value_len = self.stream.len_to_char_or_end(b';');

        // skip end quote
        if try!(self.stream.char_at(value_len as isize - 1)) == b'\'' {
            value_len -= 1;
        }

        let substream = Stream::sub_stream(&self.stream, self.stream.pos(),
                                            self.stream.pos() + value_len);

        self.stream.advance_raw(value_len);

        if !self.stream.at_end() && self.stream.is_char_eq_raw(b'\'') {
            self.stream.advance_raw(1);
        }

        // ';;;' is valid style data, we need to skip it
        while !self.stream.at_end() && self.stream.is_char_eq_raw(b';') {
            self.stream.advance_raw(1);
            self.stream.skip_spaces();
        }

        Ok(Token::Attribute(name, substream))
    }
}

impl_iter_for_tokenizer!(Token<'a>);
