// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use {
    Error,
    LengthUnit,
    Stream,
    TextFrame,
};
use stream::bound;
use colors::rgb_color_from_name;

/// Representation of the [`<color>`] type.
/// [`<color>`]: https://www.w3.org/TR/SVG/types.html#DataTypeColor
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    #[allow(missing_docs)]
    pub red: u8,
    #[allow(missing_docs)]
    pub green: u8,
    #[allow(missing_docs)]
    pub blue: u8,
}

impl Color {
    /// Constructs a new `Color` from `red`, `green` and `blue` values.
    #[inline]
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }

    /// Parses `Color` from `TextFrame`.
    ///
    /// Parsing is done according to [`spec`]:
    ///
    /// ```text
    /// color    ::= "#" hexdigit hexdigit hexdigit (hexdigit hexdigit hexdigit)?
    ///              | "rgb(" wsp* integer comma integer comma integer wsp* ")"
    ///              | "rgb(" wsp* integer "%" comma integer "%" comma integer "%" wsp* ")"
    ///              | color-keyword
    /// hexdigit ::= [0-9A-Fa-f]
    /// comma    ::= wsp* "," wsp*
    /// ```
    /// \* The SVG spec has an error. There should be `number`,
    /// not an `integer` for percent values ([`details`]).
    ///
    /// # Errors
    ///
    ///  - Returns error if a color has an invalid format.
    ///
    ///  - Returns error if `color-keyword` or `rgb` prefix are in in the lowercase.
    ///    It's not supported.
    ///
    ///  - Returns error if `<color>` is followed by `<icccolor>`.
    ///    It's not supported.
    ///
    /// # Notes
    ///
    ///  - Any non-`hexdigit` bytes will be treated as `0`.
    ///
    /// [`spec`]: http://www.w3.org/TR/SVG/types.html#DataTypeColor
    /// [`details`]: https://lists.w3.org/Archives/Public/www-svg/2014Jan/0109.html
    pub fn from_frame(frame: TextFrame) -> Result<Color, Error> {
        let mut s = Stream::from_frame(frame);

        s.skip_spaces();

        let start = s.pos();

        let mut color = Color::new(0, 0, 0);

        if s.is_char_eq(b'#')? {
            // get color data len until first space or stream end
            match s.len_to_space_or_end() {
                7 => {
                    // #rrggbb
                    s.advance(1)?;
                    color.red = hex_pair(s.read_raw(2).as_bytes());
                    color.green = hex_pair(s.read_raw(2).as_bytes());
                    color.blue = hex_pair(s.read_raw(2).as_bytes());
                }
                4 => {
                    // #rgb
                    s.advance_raw(1);
                    color.red = short_hex(s.curr_char_raw());
                    s.advance_raw(1);
                    color.green = short_hex(s.curr_char_raw());
                    s.advance_raw(1);
                    color.blue = short_hex(s.curr_char_raw());
                    s.advance_raw(1);
                }
                _ => {
                    s.set_pos_raw(start);
                    return Err(Error::InvalidColor(s.gen_error_pos()));
                }
            }
        } else if s.starts_with(b"rgb(") {
            s.advance(4)?;

            let l = s.parse_list_length()?;

            if l.unit == LengthUnit::Percent {
                fn from_persent(v: f64) -> u8 {
                    let d = 255.0 / 100.0;
                    let n = (v * d).round() as i32;
                    bound(0, n, 255) as u8
                }

                color.red = from_persent(l.num);
                color.green = from_persent(s.parse_list_length()?.num);
                color.blue = from_persent(s.parse_list_length()?.num);
            } else {
                color.red = bound(0, l.num as i32, 255) as u8;
                color.green = bound(0, s.parse_list_integer()?, 255) as u8;
                color.blue = bound(0, s.parse_list_integer()?, 255) as u8;
            }

            s.skip_spaces();
            s.consume_char(b')')?;
        } else {
            let l = s.len_to_space_or_end();
            match rgb_color_from_name(s.slice_next_raw(l)) {
                Some(c) => {
                    color = c;
                    s.advance_raw(l);
                }
                None => {
                    s.set_pos_raw(start);
                    return Err(Error::InvalidColor(s.gen_error_pos()));
                }
            }
        }

        // Check that we are at the end of the stream. Otherwise color can be followed by icccolor,
        // which is not supported.
        s.skip_spaces();
        if !s.at_end() {
            return Err(Error::InvalidColor(s.gen_error_pos()));
        }

        Ok(color)
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Color::from_frame(TextFrame::from_str(text))
    }
}

#[inline]
fn from_hex(c: u8) -> u8 {
    // TODO: validate?
    match c {
        b'0'...b'9' => c - b'0',
        b'a'...b'f' => c - b'a' + 10,
        b'A'...b'F' => c - b'A' + 10,
        _ => b'0',
    }
}

#[inline]
fn short_hex(c: u8) -> u8 {
    let h = from_hex(c);
    (h << 4) | h
}

#[inline]
fn hex_pair(chars: &[u8]) -> u8 {
    let h1 = from_hex(chars[0]);
    let h2 = from_hex(chars[1]);
    (h1 << 4) | h2
}
