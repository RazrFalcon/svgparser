// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<transform-list>`] data.
//!
//! [`<transform-list>`]: https://www.w3.org/TR/SVG/coords.html#TransformAttribute

use {Stream, TextFrame, Error};

#[derive(PartialEq,Debug)]
#[allow(missing_docs)]
pub enum TransformToken {
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
    EndOfStream,
}

/// Transform tokenizer.
#[derive(Clone,PartialEq)]
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    rotate_ts: Option<(f64, f64)>,
    last_angle: Option<f64>,
}

impl<'a> Tokenizer<'a> {
    /// Constructs a new `Tokenizer`.
    pub fn from_str(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            stream: Stream::from_str(text),
            rotate_ts: None,
            last_angle: None,
        }
    }

    /// Constructs a new `Tokenizer`.
    pub fn from_frame(frame: TextFrame<'a>) -> Tokenizer<'a> {
        Tokenizer {
            stream: Stream::from_frame(frame),
            rotate_ts: None,
            last_angle: None,
        }
    }

    /// Extracts next transform from the stream.
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - There are no separate `rotate(<rotate-angle> <cx> <cy>)` type.
    ///   It will be automatically split into three `Transform` tokens:
    ///   `translate(<cx> <cy>) rotate(<rotate-angle>) translate(-<cx> -<cy>)`.
    ///   Just like the spec is stated.
    pub fn parse_next(&mut self) -> Result<TransformToken, Error> {

        if let Some(a) = self.last_angle {
            self.last_angle = None;
            return Ok(TransformToken::Rotate {
                angle: a,
            });
        }

        if let Some((x, y)) = self.rotate_ts {
                self.rotate_ts = None;
                return Ok(TransformToken::Translate {
                    tx: -x,
                    ty: -y,
                });
        }

        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            // empty attribute is still a valid value
            return Ok(TransformToken::EndOfStream);
        }

        if s.left() < 5 {
            return Err(Error::UnexpectedEndOfStream(s.gen_error_pos()));
        }

        let t = match s.slice_next_raw(5) {
            "matri" => {
                s.advance(6)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let a = s.parse_list_number()?;
                let b = s.parse_list_number()?;
                let c = s.parse_list_number()?;
                let d = s.parse_list_number()?;
                let e = s.parse_list_number()?;
                let f = s.parse_list_number()?;

                TransformToken::Matrix {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                    e: e,
                    f: f,
                }
            }
            "trans" => {
                s.advance(9)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_char_eq(b')')? {
                    // 'If <ty> is not provided, it is assumed to be zero.'
                    0.0
                } else {
                    s.parse_list_number()?
                };

                TransformToken::Translate {
                    tx: x,
                    ty: y,
                }
            }
            "scale" => {
                s.advance(5)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_char_eq(b')')? {
                    // 'If <sy> is not provided, it is assumed to be equal to <sx>.'
                    x
                } else {
                    s.parse_list_number()?
                };

                TransformToken::Scale {
                    sx: x,
                    sy: y,
                }
            }
            "rotat" => {
                s.advance(6)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let a = s.parse_list_number()?;
                s.skip_spaces();

                if !s.is_char_eq(b')')? {
                    // 'If optional parameters <cx> and <cy> are supplied, the rotate is about the
                    // point (cx, cy). The operation represents the equivalent of the following
                    // specification:
                    // translate(<cx>, <cy>) rotate(<rotate-angle>) translate(-<cx>, -<cy>).'
                    let cx = s.parse_list_number()?;
                    let cy = s.parse_list_number()?;
                    self.rotate_ts = Some((cx, cy));
                    self.last_angle = Some(a);

                    TransformToken::Translate {
                        tx: cx,
                        ty: cy,
                    }
                } else {
                    TransformToken::Rotate {
                        angle: a,
                    }
                }
            }
            "skewX" => {
                s.advance(5)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let a = s.parse_list_number()?;

                TransformToken::SkewX {
                    angle: a,
                }
            }
            "skewY" => {
                s.advance(5)?;
                s.skip_spaces();
                s.consume_char(b'(')?;

                let a = s.parse_list_number()?;

                TransformToken::SkewY {
                    angle: a,
                }
            }
            _ => {
                return Err(Error::InvalidTransform(s.gen_error_pos()));
            }
        };

        s.skip_spaces();
        s.consume_char(b')')?;

        Ok(t)
    }
}
