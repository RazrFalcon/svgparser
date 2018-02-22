// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module for parsing [`<transform-list>`] data.
//!
//! [`<transform-list>`]: https://www.w3.org/TR/SVG/coords.html#TransformAttribute

use std::fmt;

use error::{
    Result,
};
use {
    ErrorKind,
    FromSpan,
    Stream,
    StreamExt,
    StrSpan,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum Token {
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

/// Transform tokenizer.
#[derive(Clone, Copy, PartialEq)]
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    rotate_ts: Option<(f64, f64)>,
    last_angle: Option<f64>,
}

impl<'a> FromSpan<'a> for Tokenizer<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Tokenizer {
            stream: Stream::from_span(span),
            rotate_ts: None,
            last_angle: None,
        }
    }
}

impl<'a> fmt::Debug for Tokenizer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransformTokenizer({:?})", self.stream.span())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token>;

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
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.last_angle {
            self.last_angle = None;
            return Some(Ok(Token::Rotate {
                angle: a,
            }));
        }

        if let Some((x, y)) = self.rotate_ts {
            self.rotate_ts = None;
            return Some(Ok(Token::Translate {
                tx: -x,
                ty: -y,
            }));
        }

        self.stream.skip_spaces();

        if self.stream.at_end() {
            // empty attribute is still a valid value
            return None;
        }

        let ts = self.parse_next();
        if ts.is_err() {
            self.stream.jump_to_end();
        }

        Some(ts)
    }
}

impl<'a> Tokenizer<'a> {
    fn parse_next(&mut self) -> Result<Token> {
        let s = &mut self.stream;

        let start = s.pos();
        let name = s.consume_name()?;
        s.skip_spaces();
        s.consume_byte(b'(')?;

        let t = match name.as_bytes() {
            b"matrix" => {
                Token::Matrix {
                    a: s.parse_list_number()?,
                    b: s.parse_list_number()?,
                    c: s.parse_list_number()?,
                    d: s.parse_list_number()?,
                    e: s.parse_list_number()?,
                    f: s.parse_list_number()?,
                }
            }
            b"translate" => {
                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_curr_byte_eq(b')') {
                    // 'If <ty> is not provided, it is assumed to be zero.'
                    0.0
                } else {
                    s.parse_list_number()?
                };

                Token::Translate {
                    tx: x,
                    ty: y,
                }
            }
            b"scale" => {
                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_curr_byte_eq(b')') {
                    // 'If <sy> is not provided, it is assumed to be equal to <sx>.'
                    x
                } else {
                    s.parse_list_number()?
                };

                Token::Scale {
                    sx: x,
                    sy: y,
                }
            }
            b"rotate" => {
                let a = s.parse_list_number()?;
                s.skip_spaces();

                if !s.is_curr_byte_eq(b')') {
                    // 'If optional parameters <cx> and <cy> are supplied, the rotate is about the
                    // point (cx, cy). The operation represents the equivalent of the following
                    // specification:
                    // translate(<cx>, <cy>) rotate(<rotate-angle>) translate(-<cx>, -<cy>).'
                    let cx = s.parse_list_number()?;
                    let cy = s.parse_list_number()?;
                    self.rotate_ts = Some((cx, cy));
                    self.last_angle = Some(a);

                    Token::Translate {
                        tx: cx,
                        ty: cy,
                    }
                } else {
                    Token::Rotate {
                        angle: a,
                    }
                }
            }
            b"skewX" => {
                Token::SkewX {
                    angle: s.parse_list_number()?,
                }
            }
            b"skewY" => {
                Token::SkewY {
                    angle: s.parse_list_number()?,
                }
            }
            _ => {
                let pos = s.gen_error_pos_from(start);
                return Err(ErrorKind::InvalidTransform(pos).into());
            }
        };

        s.skip_spaces();
        s.consume_byte(b')')?;
        s.skip_spaces();

        if s.is_curr_byte_eq(b',') {
            s.advance(1);
        }

        Ok(t)
    }
}
