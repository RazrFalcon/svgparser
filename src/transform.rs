// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<transform-list>`] data.
//!
//! [`<transform-list>`]: https://www.w3.org/TR/SVG/coords.html#TransformAttribute

use super::{Stream, Error};

#[allow(missing_docs)]
#[derive(PartialEq,Debug)]
pub enum Transform {
    Matrix {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
    Translate {
        tx: f64,
        ty: f64,
    },
    Scale {
        sx: f64,
        sy: f64,
    },
    Rotate {
        angle: f64,
    },
    SkewX {
        angle: f64,
    },
    SkewY {
        angle: f64,
    },
}

/// Transform token.
#[derive(Clone,PartialEq)]
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    rotate_ts: Option<(f64, f64)>,
    last_angle: Option<f64>,
}

impl<'a> Tokenizer<'a> {
    /// Constructs a new `Tokenizer`.
    pub fn new(stream: Stream<'a>) -> Tokenizer<'a> {
        Tokenizer {
            stream: stream,
            rotate_ts: None,
            last_angle: None,
        }
    }

    /// Extracts next transform from stream.
    ///
    /// # Errors
    ///
    /// - `Error::EndOfStream` indicates end of parsing, not error.
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - There are no separate `rotate(<rotate-angle> <cx> <cy>)` type.
    ///   It will be automatically split into tree `Transform` tokens:
    ///   `translate(<cx> <cy>) rotate(<rotate-angle>) translate(-<cx> -<cy>)`.
    ///   Just like spec is stated.
    pub fn parse_next(&mut self) -> Result<Transform, Error> {

        match self.last_angle {
            Some(a) => {
                self.last_angle = None;
                return Ok(Transform::Rotate {
                    angle: a,
                });
            }
            None => {}
        }

        match self.rotate_ts {
            Some((x, y)) => {
                self.rotate_ts = None;
                return Ok(Transform::Translate {
                    tx: -x,
                    ty: -y,
                });
            }
            None => {}
        }

        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            // empty attribute is still a valid value
            return Err(Error::EndOfStream);
        }

        if s.left() < 5 {
            return Err(Error::UnexpectedEndOfStream(s.gen_error_pos()));
        }

        let t = match s.slice_next_raw(5) {
            b"matri" => {
                try!(s.advance(6));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let a = try!(s.parse_list_number());
                let b = try!(s.parse_list_number());
                let c = try!(s.parse_list_number());
                let d = try!(s.parse_list_number());
                let e = try!(s.parse_list_number());
                let f = try!(s.parse_list_number());

                Transform::Matrix {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                    e: e,
                    f: f,
                }
            }
            b"trans" => {
                try!(s.advance(9));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let x = try!(s.parse_list_number());
                s.skip_spaces();

                let y;
                if try!(s.is_char_eq(b')')) {
                    // 'If <ty> is not provided, it is assumed to be zero.'
                    y = 0.0;
                } else {
                    y = try!(s.parse_list_number());
                }

                Transform::Translate {
                    tx: x,
                    ty: y,
                }
            }
            b"scale" => {
                try!(s.advance(5));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let x = try!(s.parse_list_number());
                s.skip_spaces();

                let y;
                if try!(s.is_char_eq(b')')) {
                    // 'If <sy> is not provided, it is assumed to be equal to <sx>.'
                    y = x;
                } else {
                    y = try!(s.parse_list_number());
                }

                Transform::Scale {
                    sx: x,
                    sy: y,
                }
            }
            b"rotat" => {
                try!(s.advance(6));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let a = try!(s.parse_list_number());
                s.skip_spaces();

                if !try!(s.is_char_eq(b')')) {
                    // 'If optional parameters <cx> and <cy> are supplied, the rotate is about the
                    // point (cx, cy). The operation represents the equivalent of the following
                    // specification:
                    // translate(<cx>, <cy>) rotate(<rotate-angle>) translate(-<cx>, -<cy>).'
                    let cx = try!(s.parse_list_number());
                    let cy = try!(s.parse_list_number());
                    self.rotate_ts = Some((cx, cy));
                    self.last_angle = Some(a);

                    Transform::Translate {
                        tx: cx,
                        ty: cy,
                    }
                } else {
                    Transform::Rotate {
                        angle: a,
                    }
                }
            }
            b"skewX" => {
                try!(s.advance(5));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let a = try!(s.parse_list_number());

                Transform::SkewX {
                    angle: a,
                }
            }
            b"skewY" => {
                try!(s.advance(5));
                s.skip_spaces();
                try!(s.consume_char(b'('));

                let a = try!(s.parse_list_number());

                Transform::SkewY {
                    angle: a,
                }
            }
            _ => {
                return Err(Error::InvalidTransform(s.gen_error_pos()));
            }
        };

        s.skip_spaces();
        try!(s.consume_char(b')'));

        Ok(t)
    }
}

impl_iter_for_tokenizer!(Transform);
