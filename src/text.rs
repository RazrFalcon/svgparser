// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str;
use std::char;
use std::io::Write;

use {
    Error,
    Stream,
    TextFrame,
    Tokenize,
};

const BUF_END: usize = 4;

/// Spaces processing type.
///
/// Details: https://www.w3.org/TR/SVG11/text.html#WhiteSpace
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum XmlSpace {
    Default,
    Preserve,
}

/// XML escaped text to plain text converter.
///
/// Processing is done as described in: https://www.w3.org/TR/SVG11/text.html#WhiteSpace
///
/// # Examples
///
/// Allocation free version:
///
/// ```
/// use std::str;
/// use svgparser::{Tokenize, TextUnescape};
///
/// let mut v = Vec::new();
/// let mut t = TextUnescape::from_str("&gt;");
/// loop {
///     match t.parse_next().unwrap() {
///         Some(c) => v.push(c),
///         None => break,
///     }
/// }
///
/// let s = str::from_utf8(&v).unwrap();
/// assert_eq!(s, ">");
/// ```
///
/// Version which will allocate a `String`:
///
/// ```
/// use svgparser::{TextUnescape, XmlSpace};
///
/// let s = TextUnescape::unescape("&gt;", XmlSpace::Default).unwrap();
/// assert_eq!(s, ">");
/// ```
pub struct TextUnescape<'a> {
    stream: Stream<'a>,
    buf: [u8; BUF_END],
    buf_idx: usize,
    preserve_spaces: bool,
    prev: u8,
}

impl<'a> TextUnescape<'a> {
    /// Converts provided text into an unescaped one.
    pub fn unescape(text: &str, space: XmlSpace) -> Result<String, Error> {
        let mut v = Vec::new();
        let mut t = TextUnescape::from_str(text);
        t.set_xml_space(space);
        loop {
            match t.parse_next()? {
                Some(c) => v.push(c),
                None => break,
            }
        }

        Ok(str::from_utf8(&v)?.to_owned())
    }

    /// Sets the flag that prevents spaces from being striped.
    pub fn set_xml_space(&mut self, kind: XmlSpace) {
        self.preserve_spaces = kind == XmlSpace::Preserve;
    }
}

impl<'a> Tokenize<'a> for TextUnescape<'a> {
    type Token = Option<u8>;

    fn from_frame(frame: TextFrame<'a>) -> TextUnescape<'a> {
        TextUnescape {
            stream: Stream::from_frame(frame),
            buf: [0xFF; BUF_END],
            buf_idx: BUF_END,
            preserve_spaces: false,
            prev: 0,
        }
    }

    fn parse_next(&mut self) -> Result<Option<u8>, Error> {
        if self.buf_idx != BUF_END {
            let c = self.buf[self.buf_idx];

            if c != 0xFF {
                self.buf_idx += 1;
                return Ok(Some(c));
            } else {
                self.buf_idx = BUF_END;
            }
        }

        if self.stream.at_end() {
            return Ok(None);
        }

        let mut c = self.stream.curr_char_unchecked();

        // Check for XML character entity references.
        if c == b'&' {
            if let Some(l) = self.stream.len_to(b';').ok() {
                let value = self.stream.slice_next_unchecked(l + 1);

                if let Some(v) = Stream::parse_entity_reference(value) {
                    // Reset data.
                    self.buf = [0xFF; 4];

                    let ch = char::from_u32(v).unwrap_or('\u{FFFD}');
                    write!(&mut self.buf[..], "{}", ch).unwrap();

                    c = self.buf[0];
                    self.buf_idx = 1;

                    self.stream.advance_unchecked(l);
                }
            }
        }

        // \n and \t should be converted into spaces.
        c = match c {
            b'\n' | b'\t' => b' ',
            _ => c,
        };

        self.stream.advance_unchecked(1);

        // \r should be ignored.
        if c == b'\r' {
            return self.parse_next();
        }

        // Skip continuous spaces when `preserve_spaces` is not set.
        if !self.preserve_spaces && c == b' ' && c == self.prev {
            return self.parse_next();
        }

        self.prev = c;

        Ok(Some(c))
    }
}
