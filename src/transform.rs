// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<transform-list>`] data.
//!
//! [`<transform-list>`]: https://www.w3.org/TR/SVG/coords.html#TransformAttribute

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

        let t = match s.consume_name()?.to_str() {
            "matrix" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let a = s.parse_list_number()?;
                let b = s.parse_list_number()?;
                let c = s.parse_list_number()?;
                let d = s.parse_list_number()?;
                let e = s.parse_list_number()?;
                let f = s.parse_list_number()?;

                Token::Matrix {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                    e: e,
                    f: f,
                }
            }
            "translate" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.curr_byte()? == b')' {
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
            "scale" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.curr_byte()? == b')' {
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
            "rotate" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let a = s.parse_list_number()?;
                s.skip_spaces();

                if s.curr_byte()? != b')' {
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
            "skewX" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let a = s.parse_list_number()?;

                Token::SkewX {
                    angle: a,
                }
            }
            "skewY" => {
                s.skip_spaces();
                s.consume_byte(b'(')?;

                let a = s.parse_list_number()?;

                Token::SkewY {
                    angle: a,
                }
            }
            _ => {
                return Err(ErrorKind::InvalidTransform(s.gen_error_pos()).into());
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
