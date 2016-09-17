// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// TODO: maybe rename 'unsafe' to an other word

use std::cmp;
use std::fmt;
use std::str;

use super::{Length, LengthUnit, Error, ErrorPos};

/// Streaming interface for `&[u8]` data.
#[derive(PartialEq,Clone,Copy)]
pub struct Stream<'a> {
    text: &'a [u8],
    pos: usize,
    end: usize,
    parent_text: Option<&'a [u8]>,
    parent_pos: usize,
}

impl<'a> fmt::Debug for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream {{ text: {}, pos: {}, end: {}, has_parent: {}, parent_pos: {} }}",
               u8_to_str!(self.text), self.pos, self.end, self.parent_text.is_some(),
               self.parent_pos)
    }
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
        b' ' => true,
        b'\t' => true,
        b'\n' => true,
        b'\r' => true,
        _ => false,
    }
}

// Table giving binary powers of 10.
// Entry is 10^2^i. Used to convert decimal
// exponents into floating-point numbers.
const POWERS_OF_10: [f64; 9] = [
    10.0,
    100.0,
    1.0e4,
    1.0e8,
    1.0e16,
    1.0e32,
    1.0e64,
    1.0e128,
    1.0e256
];

impl<'a> Stream<'a> {
    /// Constructs a new `Stream` from data.
    #[inline]
    pub fn new(text: &[u8]) -> Stream {
        Stream {
            text: text,
            pos: 0,
            end: text.len(),
            parent_text: None,
            parent_pos: 0,
        }
    }

    /// Constructs a new `Stream` from exists.
    ///
    /// Used to properly detect a current position on the error.
    #[inline]
    pub fn sub_stream(other: &Stream<'a>, start: usize, end: usize) -> Stream<'a> {
        debug_assert!(start <= end);

        match other.parent_text {
            Some(text) => {
                Stream {
                    parent_text: Some(text),
                    text: &other.text[start..end],
                    pos: 0,
                    end: end - start,
                    parent_pos: other.parent_pos + other.pos,
                }
            }
            None => {
                Stream {
                    parent_text: Some(&other.text),
                    text: &other.text[start..end],
                    pos: 0,
                    end: end - start,
                    parent_pos: other.pos,
                }
            }
        }
    }

    /// Returns current position.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets current position.
    #[inline]
    pub fn set_pos_raw(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Returns number of chars left.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"text");
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
    /// Accessing stream after reaching end via unsafe/_raw methods will produce
    /// rust bound checking error.
    ///
    /// [`pos()`]: #method.pos
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"text");
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

        Ok(self.text[self.pos])
    }

    /// Unsafe version of [`curr_char()`].
    ///
    /// [`curr_char()`]: #method.curr_char
    #[inline]
    pub fn curr_char_raw(&self) -> u8 {
        self.text[self.pos]
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
    /// let mut s = Stream::new(b"text");
    /// s.advance_raw(2);
    /// assert_eq!(s.char_at(-2).unwrap(), b't');
    /// assert_eq!(s.char_at(-1).unwrap(), b'e');
    /// assert_eq!(s.char_at(0).unwrap(),  b'x');
    /// assert_eq!(s.char_at(1).unwrap(),  b't');
    /// ```
    // #[inline]
    pub fn char_at(&self, pos: isize) -> Result<u8, Error> {
        if pos < 0 {
            try!(self.back_bound_check(pos));
        } else {
            try!(self.adv_bound_check(pos as usize));
        }

        let new_pos: isize = self.pos as isize + pos;
        Ok(self.text[new_pos as usize])
    }

    /// Moves back by `n` chars.
    // #[inline]
    pub fn back(&mut self, n: usize) -> Result<(), Error> {
        try!(self.back_bound_check(n as isize));
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
    /// let mut s = Stream::new(b"text");
    /// s.advance(2).unwrap(); // ok
    /// assert_eq!(s.pos(), 2);
    /// s.advance(2).unwrap(); // also ok, we at end now
    /// assert_eq!(s.pos(), 4);
    /// // fail
    /// assert_eq!(s.advance(2).err().unwrap(), Error::AdvanceError{
    ///     expected: 6,
    ///     total: 4,
    ///     pos: ErrorPos::new(1, 5),
    /// });
    /// ```
    #[inline]
    pub fn advance(&mut self, n: usize) -> Result<(), Error> {
        try!(self.adv_bound_check(n));
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
    /// let mut s = Stream::new(b"text");
    /// s.advance_raw(2); // ok
    /// s.advance_raw(20); // will cause panic via debug_assert!().
    /// ```
    #[inline]
    pub fn advance_raw(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    /// Checks that char at the current position is (white)space.
    ///
    /// Accepted chars: ' ', '\n', '\r', '\t'.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"t e x t");
    /// assert_eq!(s.is_space().unwrap(), false);
    /// s.advance_raw(1);
    /// assert_eq!(s.is_space().unwrap(), true);
    /// ```
    #[inline]
    pub fn is_space(&self) -> Result<bool, Error> {
        let c = try!(self.curr_char());
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
    /// let mut s = Stream::new(b"Some \t\n\rtext");
    /// s.advance_raw(4);
    /// s.skip_spaces();
    /// assert_eq!(s.slice_tail(), b"text");
    /// ```
    #[inline]
    pub fn skip_spaces(&mut self) {
        while !self.at_end() && self.is_space_raw() {
            self.advance_raw(1);
        }
    }

    #[inline]
    fn get_char_raw(&self, pos: usize) -> u8 {
        // if cfg!(feature = "use-unsafe") {
        //     // almost twice faster
        //     unsafe {
        //         *self.text.get_unchecked(pos)
        //     }
        // } else {
            self.text[pos]
        // }
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
    /// let s = Stream::new(b"Some long text.");
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
    /// let s = Stream::new(b"Some long text.");
    /// assert_eq!(s.len_to_char_or_end(b'l'), 5);
    /// assert_eq!(s.len_to_char_or_end(b'q'), 15);
    /// ```
    #[inline]
    pub fn len_to_char_or_end(&self, c: u8) -> usize {
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
    /// let s = Stream::new(b"Some\ntext.");
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
    /// let mut s = Stream::new(b"Some text.");
    /// s.jump_to(b't').unwrap();
    /// assert_eq!(s.pos(), 5);
    /// ```
    #[inline]
    pub fn jump_to(&mut self, c: u8) -> Result<(), Error> {
        let l = try!(self.len_to(c));
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
    /// let mut s = Stream::new(b"Some text.");
    /// s.jump_to_char_or_end(b'q');
    /// assert_eq!(s.at_end(), true);
    /// ```
    #[inline]
    pub fn jump_to_char_or_end(&mut self, c: u8) {
        let l = self.len_to_char_or_end(c);
        self.advance_raw(l);
    }

    /// Jumps to the end of the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"Some text.");
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
    /// let mut s = Stream::new(b"Some text.");
    /// assert_eq!(s.read_raw(4), b"Some");
    /// assert_eq!(s.pos(), 4);
    /// ```
    #[inline]
    pub fn read_raw(&mut self, len: usize) -> &'a [u8] {
        let s = self.slice_next_raw(len);
        self.advance_raw(s.len());
        s
    }

    /// Returns reference to data until selected char and advance stream by the data length.
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
    /// let mut s = Stream::new(b"Some text.");
    /// assert_eq!(s.read_to(b'm').unwrap(), b"So");
    /// assert_eq!(s.pos(), 2);
    /// ```
    #[inline]
    pub fn read_to(&mut self, c: u8) -> Result<&'a [u8], Error> {
        let len = try!(self.len_to(c));
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
    /// let s = Stream::new(b"Text");
    /// assert_eq!(s.slice_next_raw(3), b"Tex");
    /// ```
    #[inline]
    pub fn slice_next_raw(&self, len: usize) -> &'a [u8] {
        &self.text[self.pos..(self.pos + len)]
    }

    /// Returns data of stream within selected region.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let s = Stream::new(b"Text");
    /// assert_eq!(s.slice_region_raw(1, 3), b"ex");
    /// ```
    #[inline]
    pub fn slice_region_raw(&self, start: usize, end: usize) -> &'a [u8] {
        &self.text[start..end]
    }

    /// Returns complete data of stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"Text");
    /// s.advance(2).unwrap();
    /// assert_eq!(s.slice(), b"Text");
    /// ```
    #[inline]
    pub fn slice(&self) -> &'a [u8] {
        &self.text[..]
    }

    /// Returns tail data of stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"Some text.");
    /// s.advance(5).unwrap();
    /// assert_eq!(s.slice_tail(), b"text.");
    /// ```
    #[inline]
    pub fn slice_tail(&self) -> &'a [u8] {
        &self.text[self.pos..]
    }

    /// Returns `true` if stream data at current position starts with selected text.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"Some text.");
    /// s.advance(5).unwrap();
    /// assert_eq!(s.starts_with(b"text"), true);
    /// assert_eq!(s.starts_with(b"long"), false);
    /// ```
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.slice_tail().starts_with(text)
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
    /// let mut s = Stream::new(b"Some text.");
    /// s.consume_char(b'S').unwrap();
    /// s.consume_char(b'o').unwrap();
    /// s.consume_char(b'm').unwrap();
    /// // s.consume_char(b'q').unwrap(); // will produce error
    /// ```
    #[inline]
    pub fn consume_char(&mut self, c: u8) -> Result<(), Error> {
        if !try!(self.is_char_eq(c)) {
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
    /// We use own parser instead of Rust parser because we didn't know number length,
    /// to pass it to Rust parser and it will not return a number of consumed chars, which we need.
    /// And to detect length we need to actually parse the number,
    /// so getting only a length will be pointless.
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
    /// let mut s = Stream::new(b"3.14");
    /// assert_eq!(s.parse_number().unwrap(), 3.14);
    /// assert_eq!(s.at_end(), true);
    /// ```
    pub fn parse_number(&mut self) -> Result<f64, Error> {
        // strip off leading blanks and check for a sign
        self.skip_spaces();

        if self.at_end() {
            // empty string
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let start = self.pos();

        // NOTE: code below is port of 'strtod.c' to Rust.
        // Source: https://opensource.apple.com/source/tcl/tcl-10/tcl/compat/strtod.c
        // License: https://opensource.apple.com/source/tcl/tcl-10/tcl/license.terms

        let mut sign = false;
        match try!(self.curr_char()) {
            b'-' => {
                sign = true;
                self.advance_raw(1);
            }
            b'+' => {
                self.advance_raw(1);
            }
            _ => {}
        }

        if !is_digit(try!(self.curr_char())) && !try!(self.is_char_eq(b'.')) {
            // back to start
            self.pos = start;
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        // Count the number of digits in the mantissa (including the decimal
        // point), and also locate the decimal point.

        // Number of digits in mantissa.
        let mut mant_size: i32 = 0;
        // Number of mantissa digits before decimal point.
        let mut dec_pt: i32 = -1;
        while !self.at_end() {
            if !is_digit(self.curr_char_raw()) {
                if self.curr_char_raw() != b'.' || dec_pt >= 0 {
                    break;
                }
                dec_pt = mant_size;
            }

            mant_size += 1;
            self.advance_raw(1);
        }

        // Now suck up the digits in the mantissa.  Use two integers to
        // collect 9 digits each (this is faster than using floating-point).
        // If the mantissa has more than 18 digits, ignore the extras, since
        // they can't affect the value anyway.

        // Temporarily holds location of exponent in string.
        let p_exp = self.pos();
        self.pos = self.pos - mant_size as usize;

        if dec_pt < 0 {
            dec_pt = mant_size;
        } else {
            // one of the digits was the point
            mant_size -= 1;
        }

        // Exponent that derives from the fractional
        // part.  Under normal circumstances, it is
        // the negative of the number of digits in F.
        // However, if I is very long, the last digits
        // of I get dropped (otherwise a long I with a
        // large negative exponent could cause an
        // unnecessary overflow on I alone).  In this
        // case, frac_exp is incremented one for each
        // dropped digit.
        let frac_exp: i32;
        if mant_size > 18 {
            frac_exp = dec_pt - 18;
            mant_size = 18;
        } else {
            frac_exp = dec_pt - mant_size;
        }

        let mut fraction: f64;
        if mant_size == 0 {
            return Ok(0.0);
        } else {
            let mut frac1 = 0.0;
            let mut frac2 = 0.0;

            while mant_size > 9 {
                let mut c = try!(self.curr_char());
                try!(self.advance(1));
                if c == b'.' {
                    c = try!(self.curr_char());
                    try!(self.advance(1));
                }

                frac1 = 10.0 * frac1 + (c - b'0') as f64;
                mant_size -= 1;
            }

            while mant_size > 0 {
                let mut c = try!(self.curr_char());
                try!(self.advance(1));
                if c == b'.' {
                    c = try!(self.curr_char());
                    try!(self.advance(1));
                }

                frac2 = 10.0 * frac2 + (c - b'0') as f64;
                mant_size -= 1;
            }

            fraction = (1.0e9 * frac1) + frac2;
        }

        // skim off the exponent
        self.pos = p_exp;

        // exponent read from "EX" field
        let mut exp: i32 = 0;
        let mut exp_sign = false;

        // check that 'e' does not belong to em/ex
        if !self.at_end() && self.left() > 1 {
            let c1 = try!(self.curr_char());
            let c2 = try!(self.char_at(1));
            if (c1 == b'E' || c1 == b'e') && c2 != b'm' && c2 != b'x' {
                try!(self.advance(1));
                if try!(self.is_char_eq(b'-')) {
                    exp_sign = true;
                    try!(self.advance(1));
                } else if try!(self.is_char_eq(b'+')) {
                    try!(self.advance(1));
                }

                while !self.at_end() && is_digit(try!(self.curr_char())) {
                    exp = exp * 10 + (self.curr_char_raw() - b'0') as i32;
                    try!(self.advance(1));
                }
            }
        }

        if exp_sign {
            exp = frac_exp - exp;
        } else {
            exp = frac_exp + exp;
        }

        // Generate a floating-point number that represents the exponent.
        // Do this by processing the exponent one bit at a time to combine
        // many powers of 2 of 10. Then combine the exponent with the
        // fraction.

        if exp < 0 {
            exp_sign = true;
            exp = -exp;
        } else {
            exp_sign = false;
        }

        // Largest possible base 10 exponent.  Any
        // exponent larger than this will already
        // produce underflow or overflow, so there's
        // no need to worry about additional digit.
        let max_exponent = 511;

        if exp > max_exponent {
            exp = max_exponent;
        }

        let mut dbl_exp = 1.0;

        let mut i = 0;
        while exp != 0 {
            if (exp & 01) > 0 {
                dbl_exp *= POWERS_OF_10[i];
            }

            exp >>= 1;
            i += 1;
        }

        if exp_sign {
            fraction /= dbl_exp;
        } else {
            fraction *= dbl_exp;
        }

        if sign {
            fraction = -fraction;
        }

        Ok(fraction)
    }

    /// Parses number from the list of numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::Stream;
    ///
    /// let mut s = Stream::new(b"3.14, 12,5 , 20-4");
    /// assert_eq!(s.parse_list_number().unwrap(), 3.14);
    /// assert_eq!(s.parse_list_number().unwrap(), 12.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 5.0);
    /// assert_eq!(s.parse_list_number().unwrap(), 20.0);
    /// assert_eq!(s.parse_list_number().unwrap(), -4.0);
    /// ```
    pub fn parse_list_number(&mut self) -> Result<f64, Error> {
        let n = try!(self.parse_number());
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
            // return Err(Error::InvalidNumber(str::from_utf8(self.slice_tail()).unwrap().to_string()));
            return Err(Error::InvalidNumber(self.gen_error_pos()));
        }

        let mut sign = false;
        match try!(self.curr_char()) {
            b'-' => {
                sign = true;
                self.advance_raw(1);
            }
            b'+' => {
                self.advance_raw(1);
            }
            _ => {}
        }

        let mut v = 0i32;
        // TODO: detect overflow
        while is_digit(try!(self.curr_char())) {
            v = 10 * v + (self.curr_char_raw() - b'0') as i32;
            self.advance_raw(1);
        }

        if sign {
            v = -v;
        }

        Ok(v)
    }

    /// Parses integer from the list of numbers.
    pub fn parse_list_integer(&mut self) -> Result<i32, Error> {
        let n = try!(self.parse_integer());
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
    /// let mut s = Stream::new(b"30%");
    /// assert_eq!(s.parse_length().unwrap(), Length::new(30.0, LengthUnit::Percent));
    /// ```
    ///
    /// # Notes
    ///
    /// - Suffix must be lowercase, otherwise it will be skipped.
    pub fn parse_length(&mut self) -> Result<Length, Error> {
        self.skip_spaces();

        let n = try!(self.parse_number());

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
            // TODO: should be error
            u = LengthUnit::None;
        }

        match u {
            LengthUnit::Percent => try!(self.advance(1)),
            LengthUnit::None => {},
            _ => try!(self.advance(2)),
        }

        Ok(Length::new(n, u))
    }

    /// Parses length from the list of lengths.
    pub fn parse_list_length(&mut self) -> Result<Length, Error> {
        let l = try!(self.parse_length());
        self.skip_spaces();
        self.parse_list_separator();
        Ok(l)
    }

    // #[inline]
    fn parse_list_separator(&mut self) {
        // manually check for end, because reaching the end is not error for this function
        if !self.at_end() && self.is_char_eq_raw(b',') {
            self.advance_raw(1);
        }
    }

    // #[inline]
    fn get_parent_text(&self) -> &[u8] {
        match self.parent_text {
            Some(text) => text,
            None => self.text,
        }
    }

    fn calc_current_row(&self) -> usize {
        let text = self.get_parent_text();
        let mut row = 1;
        let end = self.pos + self.parent_pos;
        for n in 0..end {
            if text[n] == b'\n' {
                row += 1;
            }
        }

        row
    }

    fn calc_current_col(&self) -> usize {
        let text = self.get_parent_text();
        let end = self.pos + self.parent_pos;
        let mut col = 1;
        for n in 0..end {
            if n > 0 && text[n-1] == b'\n' {
                col = 2;
            } else {
                col += 1;
            }
        }

        col
    }

    /// Calculates a current absolute position.
    // #[inline]
    pub fn gen_error_pos(&self) -> ErrorPos {
        let row = self.calc_current_row();
        let col = self.calc_current_col();
        ErrorPos::new(row, col)
    }

    /// Generates a new `UnexpectedEndOfStream` error from the current position.
    // #[inline]
    pub fn gen_end_of_stream_error(&self) -> Error {
        Error::UnexpectedEndOfStream(self.gen_error_pos())
    }

    // #[inline]
    fn adv_bound_check(&self, n: usize) -> Result<(), Error> {
        let new_pos = self.pos + n;
        if new_pos > self.end {
            return Err(Error::AdvanceError{
                expected: new_pos as isize,
                total: self.end,
                pos: self.gen_error_pos(),
            });
        }

        Ok(())
    }

    // #[inline]
    fn back_bound_check(&self, n: isize) -> Result<(), Error> {
        let new_pos: isize = self.pos as isize + n;
        if new_pos < 0 {
            return Err(Error::AdvanceError{
                expected: new_pos,
                total: self.end,
                pos: self.gen_error_pos(),
            });
        }

        Ok(())
    }
}
