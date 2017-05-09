// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp;
use std::fmt;
use std::str::{self, FromStr};

use {Length, LengthUnit, Error, ErrorPos};

/// An immutable string slice.
///
/// Unlike `&str` contains a full original string.
#[derive(PartialEq,Clone,Copy)]
pub struct TextFrame<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> TextFrame<'a> {
    /// Constructs a new `TextFrame` from string.
    pub fn from_str(text: &str) -> TextFrame {
        TextFrame {
            text: text,
            start: 0,
            end: text.len(),
        }
    }

    /// Constructs a new `TextFrame` from substring.
    pub fn from_substr(text: &str, start: usize, end: usize) -> TextFrame {
        debug_assert!(start <= end);

        TextFrame {
            text: text,
            start: start,
            end: end,
        }
    }

    /// Returns a start position of the frame.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns a end position of the frame.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns a length of the frame.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns a length of the frame underling string.
    pub fn full_len(&self) -> usize {
        self.text.len()
    }

    /// Returns a frame slice.
    pub fn slice(&self) -> &'a str {
        &self.text[self.start..self.end]
    }

    /// Returns an underling string.
    pub fn full_slice(&self) -> &'a str {
        &self.text
    }
}

impl<'a> fmt::Debug for TextFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' {}..{}", self.slice(), self.start, self.end)
    }
}

impl<'a> fmt::Display for TextFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.slice())
    }
}

/// Streaming text parsing interface.
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Stream<'a> {
    text: &'a str,
    pos: usize,
    end: usize,
    frame: TextFrame<'a>,
}

#[inline]
pub fn bound<T: Ord>(min: T, val: T, max: T) -> T {
    cmp::max(min, cmp::min(max, val))
}

#[inline]
fn is_digit(c: u8) -> bool {
    match c {
        b'0'...b'9' => true,
        _ => false,
    }
}

#[inline]
fn is_space(c: u8) -> bool {
    match c {
          b' '
        | b'\t'
        | b'\n'
        | b'\r' => true,
        _ => false,
    }
}

#[inline]
fn is_letter(c: u8) -> bool {
    match c {
        b'A'...b'Z' | b'a'...b'z' => true,
        _ => false,
    }
}

impl<'a> Stream<'a> {
    /// Constructs a new `Stream` from string.
    pub fn from_frame(text_frame: TextFrame<'a>) -> Stream {
        Stream {
            text: text_frame.slice(),
            pos: 0,
            end: text_frame.len(),
            frame: text_frame,
        }
    }

    /// Constructs a new `Stream` from string.
    pub fn from_str(text: &str) -> Stream {
        Stream {
            text: text,
            pos: 0,
            end: text.len(),
            frame: TextFrame::from_str(text),
        }
    }

    /// Returns current position.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets current position.
    // TODO: remove, parser should be consuming only
    #[inline]
    pub fn set_pos_raw(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Returns number of bytes left.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("text");
    /// s.advance_raw(4);
    /// assert_eq!(s.at_end(), true);
    /// assert_eq!(s.left(), 0);
    /// ```
    #[inline]
    pub fn left(&self) -> usize {
        self.end - self.pos
    }

    /// Returns `true` if we are at the end of the stream.
    ///
    /// Any [`pos()`] value larger than original text length indicates stream end.
    ///
    /// Accessing stream after reaching end via safe methods will produce `svgparser::Error`.
    ///
    /// Accessing stream after reaching end via *_raw methods will produce
    /// a Rust's bound checking error.
    ///
    /// [`pos()`]: #method.pos
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("text");
    /// s.advance_raw(2);
    /// assert_eq!(s.curr_char_raw(), b'x');
    /// assert_eq!(s.at_end(), false);
    /// s.advance_raw(2);
    /// assert_eq!(s.at_end(), true);
    /// ```
    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.end
    }

    /// Returns a char from current stream position.
    ///
    /// # Errors
    ///
    /// Returns `Error::UnexpectedEndOfStream` if we at the end of the stream.
    #[inline]
    pub fn curr_char(&self) -> Result<u8, Error> {
        if self.at_end() {
            return Err(self.gen_end_of_stream_error());
        }

        Ok(self.curr_char_raw())
    }

    /// Unsafe version of [`curr_char()`].
    ///
    /// [`curr_char()`]: #method.curr_char
    #[inline]
    pub fn curr_char_raw(&self) -> u8 {
        self.get_char_raw(self.pos)
    }

    /// Compares selected char with char from current stream position.
    ///
    /// # Errors
    ///
    /// Returns `Error::UnexpectedEndOfStream` if we at the end of the stream.
    #[inline]
    pub fn is_char_eq(&self, c: u8) -> Result<bool, Error> {
        if self.at_end() {
            return Err(self.gen_end_of_stream_error());
        }

        Ok(self.curr_char_raw() == c)
    }

    /// Unsafe version of [`is_char_eq()`].
    ///
    /// [`is_char_eq()`]: #method.is_char_eq
    #[inline]
    pub fn is_char_eq_raw(&self, c: u8) -> bool {
        self.curr_char_raw() == c
    }

    /// Returns char at the position relative to current.
    ///
    /// # Errors
    ///
    /// Returns `Error::AdvanceError` if we are out of the stream bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("text");
    /// s.advance_raw(2);
    /// assert_eq!(s.char_at(-2).unwrap(), b't');
    /// assert_eq!(s.char_at(-1).unwrap(), b'e');
    /// assert_eq!(s.char_at(0).unwrap(),  b'x');
    /// assert_eq!(s.char_at(1).unwrap(),  b't');
    /// ```
    #[inline]
    pub fn char_at(&self, pos: isize) -> Result<u8, Error> {
        if pos < 0 {
            self.back_bound_check(pos)?;
        } else {
            self.adv_bound_check(pos as usize)?;
        }

        let new_pos: isize = self.pos as isize + pos;
        Ok(self.get_char_raw(new_pos as usize))
    }

    /// Moves back by `n` chars.
    // TODO: remove parser should be consuming only
    #[inline]
    pub fn back(&mut self, n: usize) -> Result<(), Error> {
        self.back_bound_check(n as isize)?;
        self.pos -= n;
        Ok(())
    }

    /// Advance by `n` chars.
    ///
    /// # Errors
    ///
    /// Returns `Error::AdvanceError` if new position beyond stream end.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::{Stream, Error, ErrorPos};
    ///
    /// let mut s = Stream::from_str("text");
    /// s.advance(2).unwrap(); // ok
    /// assert_eq!(s.pos(), 2);
    /// s.advance(2).unwrap(); // also ok, we at end now
    /// assert_eq!(s.pos(), 4);
    /// // fail
    /// assert_eq!(s.advance(2).err().unwrap(), Error::InvalidAdvance{
    ///     expected: 6,
    ///     total: 4,
    ///     pos: ErrorPos::new(1, 5),
    /// });
    /// ```
    #[inline]
    pub fn advance(&mut self, n: usize) -> Result<(), Error> {
        self.adv_bound_check(n)?;
        self.pos += n;
        Ok(())
    }

    /// Unsafe version of [`advance()`].
    ///
    /// [`advance()`]: #method.advance
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("text");
    /// s.advance_raw(2); // ok
    /// s.advance_raw(20); // will cause panic via debug_assert!().
    /// ```
    #[inline]
    pub fn advance_raw(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    /// Checks that the current char is (white)space.
    ///
    /// Accepted chars: ' ', '\n', '\r', '\t'.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("t e x t");
    /// assert_eq!(s.is_space().unwrap(), false);
    /// s.advance_raw(1);
    /// assert_eq!(s.is_space().unwrap(), true);
    /// ```
    #[inline]
    pub fn is_space(&self) -> Result<bool, Error> {
        let c = self.curr_char()?;
        Ok(is_space(c))
    }

    /// Unsafe version of [`is_space()`].
    ///
    /// [`is_space()`]: #method.is_space
    #[inline]
    pub fn is_space_raw(&self) -> bool {
        is_space(self.curr_char_raw())
    }

    /// Skips (white)space's.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some \t\n\rtext");
    /// s.advance_raw(4);
    /// s.skip_spaces();
    /// assert_eq!(s.slice_tail(), "text");
    /// ```
    #[inline]
    pub fn skip_spaces(&mut self) {
        while !self.at_end() && self.is_space_raw() {
            self.advance_raw(1);
        }
    }

    /// Decreases the stream size by removing trailing spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text  ");
    /// assert_eq!(s.left(), 11);
    /// s.trim_trailing_spaces();
    /// assert_eq!(s.left(), 9);
    /// ```
    #[inline]
    pub fn trim_trailing_spaces(&mut self) {
        while !self.at_end() && is_space(self.get_char_raw(self.end - 1)) {
            self.end -= 1;
        }
    }

    /// Checks that the current char is a letter.
    #[inline]
    pub fn is_letter_raw(&self) -> bool {
        is_letter(self.curr_char_raw())
    }

    /// Checks that the current char is a digit.
    #[inline]
    pub fn is_digit_raw(&self) -> bool {
        is_digit(self.curr_char_raw())
    }

    /// Checks that the current char is a valid part of an ident token.
    #[inline]
    pub fn is_ident_raw(&self) -> bool {
        let c = self.curr_char_raw();
        match c {
              b'0'...b'9'
            | b'A'...b'Z'
            | b'a'...b'z'
            | b'-'
            | b'_'
            | b':' => true,
            _ => false,
        }
    }

    #[inline]
    fn get_char_raw(&self, pos: usize) -> u8 {
        // TODO: maybe via unsafe to avoid bound checking
        self.text.as_bytes()[pos]
    }

    /// Calculates length to the selected char.
    ///
    /// # Errors
    ///
    /// Returns `Error::UnexpectedEndOfStream` if no such char.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::from_str("Some long text.");
    /// assert_eq!(s.len_to(b'l').unwrap(), 5);
    /// ```
    #[inline]
    pub fn len_to(&self, c: u8) -> Result<usize, Error> {
        let mut n = 0;
        while self.pos + n != self.end {
            if self.get_char_raw(self.pos + n) == c {
                return Ok(n);
            } else {
                n += 1;
            }
        }

        Err(self.gen_end_of_stream_error())
    }

    /// Calculates length to the selected char.
    ///
    /// If char not found - returns length to the end of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::from_str("Some long text.");
    /// assert_eq!(s.len_to_or_end(b'l'), 5);
    /// assert_eq!(s.len_to_or_end(b'q'), 15);
    /// ```
    #[inline]
    pub fn len_to_or_end(&self, c: u8) -> usize {
        let mut n = 0;
        while self.pos + n != self.end {
            if self.get_char_raw(self.pos + n) == c {
                break;
            } else {
                n += 1;
            }
        }

        n
    }

    /// Calculates length to the 'space' char.
    ///
    /// Checked according to [`is_space()`] method.
    ///
    /// [`is_space()`]: #method.is_space
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::from_str("Some\ntext.");
    /// assert_eq!(s.len_to_space_or_end(), 4);
    /// ```
    #[inline]
    pub fn len_to_space_or_end(&self) -> usize {
        let mut n = 0;
        while self.pos + n != self.end {
            if is_space(self.get_char_raw(self.pos + n)) {
                break;
            } else {
                n += 1;
            }
        }

        n
    }

    /// Jumps to the selected char.
    ///
    /// # Errors
    ///
    /// Returns `Error::UnexpectedEndOfStream` if no such char.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.jump_to(b't').unwrap();
    /// assert_eq!(s.pos(), 5);
    /// ```
    #[inline]
    pub fn jump_to(&mut self, c: u8) -> Result<(), Error> {
        let l = self.len_to(c)?;
        self.advance_raw(l);
        Ok(())
    }

    /// Jumps to the selected char or the end of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.jump_to_or_end(b'q');
    /// assert_eq!(s.at_end(), true);
    /// ```
    #[inline]
    pub fn jump_to_or_end(&mut self, c: u8) {
        let l = self.len_to_or_end(c);
        self.advance_raw(l);
    }

    /// Jumps to the end of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.jump_to_end();
    /// assert_eq!(s.at_end(), true);
    /// ```
    #[inline]
    pub fn jump_to_end(&mut self) {
        let l = self.left();
        self.advance_raw(l);
    }

    /// Returns reference to data with length `len` and advance stream to the same length.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// assert_eq!(s.read_raw(4), "Some");
    /// assert_eq!(s.pos(), 4);
    /// ```
    #[inline]
    pub fn read_raw(&mut self, len: usize) -> &'a str {
        let s = self.slice_next_raw(len);
        self.advance_raw(s.len());
        s
    }

    /// Returns reference to the data until selected char and advance stream by the data length.
    ///
    /// Shorthand for: [`len_to()`] + [`read_raw()`].
    ///
    /// [`len_to()`]: #method.len_to
    /// [`read_raw()`]: #method.read_raw
    ///
    /// # Errors
    ///
    /// Returns `Error::UnexpectedEndOfStream` if no such char.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// assert_eq!(s.read_to(b'm').unwrap(), "So");
    /// assert_eq!(s.pos(), 2);
    /// ```
    #[inline]
    pub fn read_to(&mut self, c: u8) -> Result<&'a str, Error> {
        let len = self.len_to(c)?;
        let s = self.read_raw(len);
        Ok(s)
    }

    /// Returns next data of stream with selected length.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::from_str("Text");
    /// assert_eq!(s.slice_next_raw(3), "Tex");
    /// ```
    #[inline]
    pub fn slice_next_raw(&self, len: usize) -> &'a str {
        &self.text[self.pos..(self.pos + len)]
    }

    /// Returns data of the stream within selected region.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::from_str("Text");
    /// assert_eq!(s.slice_region_raw(1, 3), "ex");
    /// ```
    #[inline]
    pub fn slice_region_raw(&self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

    /// Returns data of the stream within selected region as `TextFrame`.
    pub fn slice_frame_raw(&self, start: usize, end: usize) -> TextFrame<'a> {
        debug_assert!(start <= end);

        TextFrame::from_substr(self.frame.slice(), start, end)
    }

    /// Returns complete data of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Text");
    /// s.advance(2).unwrap();
    /// assert_eq!(s.slice(), "Text");
    /// ```
    #[inline]
    pub fn slice(&self) -> &'a str {
        &self.text[..self.end]
    }

    /// Returns complete data of the stream as `TextFrame`.
    #[inline]
    pub fn slice_frame(&self) -> TextFrame<'a> {
        TextFrame::from_substr(self.frame.slice(), self.frame.start(), self.frame.end())
    }

    /// Returns tail data of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.advance(5).unwrap();
    /// assert_eq!(s.slice_tail(), "text.");
    /// ```
    #[inline]
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..self.end]
    }

    /// Returns tail data of the stream as `TextFrame`.
    #[inline]
    pub fn slice_tail_frame(&self) -> TextFrame<'a> {
        TextFrame::from_substr(self.frame.slice(), self.pos, self.end)
    }

    /// Returns `true` if stream data at current position starts with selected text.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.advance(5).unwrap();
    /// assert_eq!(s.starts_with(b"text"), true);
    /// assert_eq!(s.starts_with(b"long"), false);
    /// ```
    // we are using &[u8] instead of &str for performance reasons
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.slice_tail().as_bytes().starts_with(text)
    }

    /// Consumes selected char.
    ///
    /// # Errors
    ///
    /// If current char is not equal to selected - we will get `Error::InvalidChar`.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("Some text.");
    /// s.consume_char(b'S').unwrap();
    /// s.consume_char(b'o').unwrap();
    /// s.consume_char(b'm').unwrap();
    /// // s.consume_char(b'q').unwrap(); // will produce error
    /// ```
    #[inline]
    pub fn consume_char(&mut self, c: u8) -> Result<(), Error> {
        if !self.is_char_eq(c)? {
            return Err(Error::InvalidChar {
                current: self.curr_char_raw() as char,
                expected: c as char,
                pos: self.gen_error_pos(),
            });
        }
        self.advance_raw(1);
        Ok(())
    }

    /// Parses number from the stream.
    ///
    /// This method will detect a number length and then
    /// will pass a substring to the `std::from_str` method.
    ///
    /// https://www.w3.org/TR/SVG/types.html#DataTypeNumber
    ///
    /// # Errors
    ///
    /// Can return most of the `Error` errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("3.14");
    /// assert_eq!(s.parse_number().unwrap(), 3.14);
    /// assert_eq!(s.at_end(), true);
    /// ```
    pub fn parse_number(&mut self) -> Result<f64, Error> {
        // strip off leading blanks
        self.skip_spaces();

        if self.at_end() {
            // empty string
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let start = self.pos();

        macro_rules! gen_err {
            () => ({
                // back to start
                self.pos = start;
                Err(Error::InvalidNumber(self.gen_error_pos()))
            })
        }

        // consume sign
        match self.curr_char()? {
            b'+' | b'-' => self.advance_raw(1), // skip sign
            _ => {}
        }

        // consume integer
        {
            // current char must be a digit or a dot

            let c = self.curr_char()?;
            if is_digit(c) {
                self.consume_digits();
            } else if c == b'.' {
                // nothing
            } else {
                return gen_err!();
            }
        }

        let mut check_exponent = false;

        // consume fraction
        if !self.at_end() {
            // current char must be a dot or an exponent sign

            let mut c = self.curr_char_raw();
            if c == b'.' {
                self.advance_raw(1); // skip dot
                self.consume_digits();
                if !self.at_end() {
                    // Could have an exponent component.
                    c = self.curr_char_raw();
                }
            }
            if c == b'e' || c == b'E' {
                check_exponent = true;
            } else {
                // do nothing
            }
        }

        // consume exponent
        if check_exponent && !self.at_end() {
            let c = self.curr_char_raw();

            if c == b'e' || c == b'E' {
                self.advance_raw(1); // skip 'e'

                let c = self.curr_char()?;
                if c == b'+' || c == b'-' {
                    self.advance_raw(1); // skip sign
                    self.consume_digits();
                } else if is_digit(c) {
                    self.consume_digits();
                } else {
                    // not an exponent
                    // probably 'ex' or 'em'
                    self.back(1)?;
                }
            } else {
                // no exponent
            }
        }

        // use default f64 parser now
        let s = self.slice_region_raw(start, self.pos());
        match f64::from_str(s) {
            Ok(n) => {
                if n.is_finite() {
                    Ok(n)
                } else {
                    // inf, nan, etc. are an error
                    gen_err!()
                }
            }
            Err(_) => gen_err!(),
        }
    }

    #[inline]
    fn consume_digits(&mut self) {
        while !self.at_end() && self.is_digit_raw() {
            self.advance_raw(1);
        }
    }

    /// Parses number from the list of numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::from_str("3.14, 12,5 , 20-4");
    /// assert_eq!(s.parse_list_number().unwrap(), 3.14);
    /// assert_eq!(s.parse_list_number().unwrap(), 12.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 5.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 20.0);
    /// assert_eq!(s.parse_list_number().unwrap(), -4.0);
    /// ```
    pub fn parse_list_number(&mut self) -> Result<f64, Error> {
        if self.at_end() {
            return Err(self.gen_end_of_stream_error());
        }

        let n = self.parse_number()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(n)
    }

    /// Parses integer number from the stream.
    ///
    /// Same as [`parse_number()`], but only for integer. Does not refer to any SVG type.
    /// [`parse_number()`]: #method.parse_number
    pub fn parse_integer(&mut self) -> Result<i32, Error> {
        self.skip_spaces();

        if self.at_end() {
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let start = self.pos();

        macro_rules! gen_err {
            () => ({
                // back to start
                self.pos = start;
                Err(Error::InvalidNumber(self.gen_error_pos()))
            })
        }

        // consume sign
        match self.curr_char()? {
            b'+' | b'-' => self.advance_raw(1),
            _ => {}
        }

        // current char must be a digit
        if !is_digit(self.curr_char()?) {
            return gen_err!();
        }

        self.consume_digits();

        // use default i32 parser now
        let s = self.slice_region_raw(start, self.pos());
        match i32::from_str(s) {
            Ok(n) => Ok(n),
            Err(_) => gen_err!(),
        }
    }

    /// Parses integer from the list of numbers.
    pub fn parse_list_integer(&mut self) -> Result<i32, Error> {
        if self.at_end() {
            return Err(self.gen_end_of_stream_error());
        }

        let n = self.parse_integer()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(n)
    }

    /// Parses length from the stream.
    ///
    /// https://www.w3.org/TR/SVG/types.html#DataTypeLength
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::{Stream, Length, LengthUnit};
    ///
    /// let mut s = Stream::from_str("30%");
    /// assert_eq!(s.parse_length().unwrap(), Length::new(30.0, LengthUnit::Percent));
    /// ```
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be an error.
    pub fn parse_length(&mut self) -> Result<Length, Error> {
        self.skip_spaces();

        let n = self.parse_number()?;

        if self.at_end() {
            return Ok(Length::new(n, LengthUnit::None));
        }

        let u;
        if self.starts_with(b"%") {
            u = LengthUnit::Percent;
        } else if self.starts_with(b"em") {
            u = LengthUnit::Em;
        } else if self.starts_with(b"ex") {
            u = LengthUnit::Ex;
        } else if self.starts_with(b"px") {
            u = LengthUnit::Px;
        } else if self.starts_with(b"in") {
            u = LengthUnit::In;
        } else if self.starts_with(b"cm") {
            u = LengthUnit::Cm;
        } else if self.starts_with(b"mm") {
            u = LengthUnit::Mm;
        } else if self.starts_with(b"pt") {
            u = LengthUnit::Pt;
        } else if self.starts_with(b"pc") {
            u = LengthUnit::Pc;
        } else {
            u = LengthUnit::None;
        }

        match u {
            LengthUnit::Percent => self.advance(1)?,
            LengthUnit::None => {}
            _ => self.advance(2)?,
        }

        Ok(Length::new(n, u))
    }

    /// Parses length from the list of lengths.
    pub fn parse_list_length(&mut self) -> Result<Length, Error> {
        if self.at_end() {
            return Err(self.gen_end_of_stream_error());
        }

        let l = self.parse_length()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(l)
    }

    #[inline]
    fn parse_list_separator(&mut self) {
        // manually check for end, because reaching the end is not error for this function
        if !self.at_end() && self.is_char_eq_raw(b',') {
            self.advance_raw(1);
        }
    }

    fn calc_current_row(&self) -> usize {
        let text = self.frame.full_slice();
        let mut row = 1;
        let end = self.pos + self.frame.start();
        row += text.as_bytes().iter()
            .take(end)
            .filter(|c| **c == b'\n')
            .count();
        row
    }

    fn calc_current_col(&self) -> usize {
        let text = self.frame.full_slice();
        let bytes = text.as_bytes();
        let end = self.pos + self.frame.start();
        let mut col = 1;
        for n in 0..end {
            if n > 0 && bytes[n - 1] == b'\n' {
                col = 2;
            } else {
                col += 1;
            }
        }

        col
    }

    /// Calculates a current absolute position.
    pub fn gen_error_pos(&self) -> ErrorPos {
        let row = self.calc_current_row();
        let col = self.calc_current_col();
        ErrorPos::new(row, col)
    }

    /// Generates a new `UnexpectedEndOfStream` error from the current position.
    #[inline]
    pub fn gen_end_of_stream_error(&self) -> Error {
        Error::UnexpectedEndOfStream(self.gen_error_pos())
    }

    fn adv_bound_check(&self, n: usize) -> Result<(), Error> {
        let new_pos = self.pos + n;
        if new_pos > self.end {
            return Err(Error::InvalidAdvance{
                expected: new_pos as isize,
                total: self.end,
                pos: self.gen_error_pos(),
            });
        }

        Ok(())
    }

    fn back_bound_check(&self, n: isize) -> Result<(), Error> {
        let new_pos: isize = self.pos as isize + n;
        if new_pos < 0 {
            return Err(Error::InvalidAdvance{
                expected: new_pos,
                total: self.end,
                pos: self.gen_error_pos(),
            });
        }

        Ok(())
    }
}
