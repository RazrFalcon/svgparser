// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module for parsing [`<path>`] data.
//!
//! [`<path>`]: https://www.w3.org/TR/SVG/paths.html#PathData

use std::fmt;

use xmlparser::{
    FromSpan,
    Stream,
    StrSpan,
};

use {
    StreamExt,
};

/// Path's segment token.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    MoveTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    LineTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    HorizontalLineTo {
        abs: bool,
        x: f64,
    },
    VerticalLineTo {
        abs: bool,
        y: f64,
    },
    CurveTo {
        abs: bool,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    SmoothCurveTo {
        abs: bool,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        abs: bool,
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    SmoothQuadratic {
        abs: bool,
        x: f64,
        y: f64,
    },
    EllipticalArc {
        abs: bool,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    },
    ClosePath {
        abs: bool,
    },
}

/// Tokenizer for the \<path\> data.
#[derive(Clone, Copy, PartialEq)]
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> FromSpan<'a> for Tokenizer<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Tokenizer {
            stream: Stream::from_span(span),
            prev_cmd: None,
        }
    }
}

impl<'a> fmt::Debug for Tokenizer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathTokenizer({:?})", self.stream.span())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    /// Extracts next path data segment from the stream.
    ///
    /// # Errors
    ///
    /// - By the SVG spec any invalid data inside path data should stop parsing of this data,
    ///   but not the whole document.
    ///
    ///   This function will return `Error::EndOfStream` on any kind of error
    ///   and print a warning to stderr.
    ///
    ///   In other words, you will retrieve as much data as possible.
    ///
    ///   Example: `M 10 20 L 30 40 #!@$1 L 50 60` -> `M 10 20 L 30 40`
    ///
    /// # Notes
    ///
    /// - We do not support implicit commands, so all commands will be converted to explicit one.
    ///   It mostly affects implicit MoveTo, which will be converted, according to the spec,
    ///   into explicit LineTo.
    ///
    ///   Example: `M 10 20 30 40 50 60` -> `M 10 20 L 30 40 L 50 60`
    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return None;
        }

        macro_rules! data_error {
            () => ({
                warn!(
                    "Invalid path data at {}. The remaining data is ignored.",
                    s.gen_error_pos()
                );
                s.jump_to_end();
                return None;
            })
        }

        macro_rules! try_num {
            ($expr:expr) => (
                match $expr {
                    Ok(v) => v,
                    Err(_) => data_error!(),
                }
            )
        }

        macro_rules! parse_num {
            () => ( try_num!(s.parse_list_number()); )
        }

        let has_prev_cmd = self.prev_cmd.is_some();
        let first_char = s.curr_byte().unwrap(); // TODO: remove unwrap

        if !has_prev_cmd && !is_cmd(first_char) {
            warn!("'{}' is not a command. \
                   The remaining data is ignored.", first_char as char);
            s.jump_to_end();
            return None;
        }

        if !has_prev_cmd {
            match first_char {
                b'M' | b'm' => {}
                _ => {
                    warn!("First segment must be MoveTo. \
                           The remaining data is ignored.");
                    s.jump_to_end();
                    return None;
                }
            }
        }

        // TODO: simplify
        let is_implicit_move_to;
        let cmd: u8;
        if is_cmd(first_char) {
            is_implicit_move_to = false;
            cmd = first_char;
            s.advance(1);
        } else if is_digit(first_char) && has_prev_cmd {
            // unwrap is safe, because we checked 'has_prev_cmd'
            let prev_cmd = self.prev_cmd.unwrap();

            if prev_cmd == b'Z' || prev_cmd == b'z' {
                warn!("ClosePath cannot be followed by a number. \
                       The remaining data is ignored.");
                s.jump_to_end();
                return None;
            }

            if prev_cmd == b'M' || prev_cmd == b'm' {
                // 'If a moveto is followed by multiple pairs of coordinates,
                // the subsequent pairs are treated as implicit lineto commands.'
                // So we parse them as LineTo.
                is_implicit_move_to = true;
                cmd = if is_absolute(prev_cmd) { b'L' } else { b'l' };
            } else {
                is_implicit_move_to = false;
                cmd = prev_cmd;
            }
        } else {
            data_error!();
        }

        let cmdl = to_relative(cmd);
        let absolute = is_absolute(cmd);
        let token = match cmdl {
            b'm' => {
                Token::MoveTo {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'l' => {
                Token::LineTo {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'h' => {
                Token::HorizontalLineTo {
                    abs: absolute,
                    x: parse_num!(),
                }
            }
            b'v' => {
                Token::VerticalLineTo {
                    abs: absolute,
                    y: parse_num!(),
                }
            }
            b'c' => {
                Token::CurveTo {
                    abs: absolute,
                    x1: parse_num!(),
                    y1: parse_num!(),
                    x2: parse_num!(),
                    y2: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b's' => {
                Token::SmoothCurveTo {
                    abs: absolute,
                    x2: parse_num!(),
                    y2: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b'q' => {
                Token::Quadratic {
                    abs: absolute,
                    x1: parse_num!(),
                    y1: parse_num!(),
                    x:  parse_num!(),
                    y:  parse_num!(),
                }
            }
            b't' => {
                Token::SmoothQuadratic {
                    abs: absolute,
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'a' => {
                Token::EllipticalArc {
                    abs: absolute,
                    rx: parse_num!(),
                    ry: parse_num!(),
                    x_axis_rotation: parse_num!(),
                    large_arc: try_opt!(parse_flag(s)),
                    sweep: try_opt!(parse_flag(s)),
                    x: parse_num!(),
                    y: parse_num!(),
                }
            }
            b'z' => {
                Token::ClosePath {
                    abs: absolute,
                }
            }
            _ => unreachable!(),
        };

        self.prev_cmd = Some(
            if is_implicit_move_to {
                if is_absolute(cmd) { b'M' } else { b'm' }
            } else {
                cmd
            }
        );

        Some(token)
    }
}

/// Returns `true` if the selected char is the command.
pub fn is_cmd(c: u8) -> bool {
    match c {
          b'M' | b'm'
        | b'Z' | b'z'
        | b'L' | b'l'
        | b'H' | b'h'
        | b'V' | b'v'
        | b'C' | b'c'
        | b'S' | b's'
        | b'Q' | b'q'
        | b'T' | b't'
        | b'A' | b'a' => true,
        _ => false,
    }
}

/// Returns `true` if the selected char is the absolute command.
pub fn is_absolute(c: u8) -> bool {
    debug_assert!(is_cmd(c));
    match c {
          b'M'
        | b'Z'
        | b'L'
        | b'H'
        | b'V'
        | b'C'
        | b'S'
        | b'Q'
        | b'T'
        | b'A' => true,
        _ => false,
    }
}

/// Converts the selected command char into the relative command char.
pub fn to_relative(c: u8) -> u8 {
    debug_assert!(is_cmd(c));
    match c {
        b'M' => b'm',
        b'Z' => b'z',
        b'L' => b'l',
        b'H' => b'h',
        b'V' => b'v',
        b'C' => b'c',
        b'S' => b's',
        b'Q' => b'q',
        b'T' => b't',
        b'A' => b'a',
        _ => c,
    }
}

fn is_digit(c: u8) -> bool {
    match c {
        b'0'...b'9' | b'.' | b'-' | b'+' => true,
        _ => false,
    }
}

// By the SVG spec 'large-arc' and 'sweep' must contain only one char
// and can be written without any separators, aka: 10 20 30 01 10 20.
fn parse_flag(s: &mut Stream) -> Option<bool> {
    s.skip_spaces();
    let c = try_opt!(s.curr_byte().ok());
    match c {
        b'0' | b'1' => {
            s.advance(1);
            if try_opt!(s.curr_byte().ok()) == b',' {
                s.advance(1);
            }
            s.skip_spaces();

            Some(c == b'1')
        }
        _ => {
            // error type is not relevant since it will be ignored
            None
        }
    }
}
