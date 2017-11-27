// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use xmlparser::{
    XmlByteExt,
};

use error::{
    Result,
};
use {
    Error,
    ErrorKind,
    LengthUnit,
    Stream,
    StreamExt,
    StrSpan,
};
use streamext::bound;
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

    /// Parses `Color` from `StrSpan`.
    ///
    /// Parsing is done according to [spec]:
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
    /// not an `integer` for percent values ([details]).
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
    /// [spec]: http://www.w3.org/TR/SVG/types.html#DataTypeColor
    /// [details]: https://lists.w3.org/Archives/Public/www-svg/2014Jan/0109.html
    pub fn from_span(span: StrSpan) -> Result<Color> {
        let mut s = Stream::from_span(span);

        s.skip_spaces();

        let start = s.pos();

        let mut color = Color::new(0, 0, 0);

        if s.curr_byte()? == b'#' {
            s.advance(1);
            let color_str = s.consume_bytes(|_, c| c.is_xml_hex_digit()).to_str().as_bytes();
            // get color data len until first space or stream end
            match color_str.len() {
                6 => {
                    // #rrggbb
                    color.red   = hex_pair(color_str[0], color_str[1]);
                    color.green = hex_pair(color_str[2], color_str[3]);
                    color.blue  = hex_pair(color_str[4], color_str[5]);
                }
                3 => {
                    // #rgb
                    color.red = short_hex(color_str[0]);
                    color.green = short_hex(color_str[1]);
                    color.blue = short_hex(color_str[2]);
                }
                _ => {
                    return Err(ErrorKind::InvalidColor(s.gen_error_pos_from(start)).into());
                }
            }
        } else if s.starts_with(b"rgb(") {
            s.advance(4);

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
            s.consume_byte(b')')?;
        } else {
            let name = s.consume_name()?;
            match rgb_color_from_name(name.to_str()) {
                Some(c) => {
                    color = c;
                }
                None => {
                    return Err(ErrorKind::InvalidColor(s.gen_error_pos_from(start)).into());
                }
            }
        }

        // Check that we are at the end of the stream. Otherwise color can be followed by icccolor,
        // which is not supported.
        s.skip_spaces();
        if !s.at_end() {
            return Err(ErrorKind::InvalidColor(s.gen_error_pos()).into());
        }

        Ok(color)
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        Color::from_span(StrSpan::from_str(text))
    }
}

#[inline]
fn from_hex(c: u8) -> u8 {
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
fn hex_pair(c1: u8, c2: u8) -> u8 {
    let h1 = from_hex(c1);
    let h2 = from_hex(c2);
    (h1 << 4) | h2
}
