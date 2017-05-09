// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<path>`] data.
//!
//! [`<path>`]: https://www.w3.org/TR/SVG/paths.html#PathData

use {Tokenize, Stream, TextFrame, Error, ErrorPos};

/// Path's segment token.
#[allow(missing_docs)]
#[derive(Copy,Clone,Debug,PartialEq)]
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
    /// The end of the stream.
    EndOfStream,
}

/// Tokenizer for the \<path\> data.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    type Token = Token;

    fn from_frame(frame: TextFrame<'a>) -> Tokenizer<'a> {
        Tokenizer {
            stream: Stream::from_frame(frame),
            prev_cmd: None,
        }
    }

    fn from_str(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            stream: Stream::from_str(text),
            prev_cmd: None,
        }
    }

    /// Extracts next path data segment from the stream.
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - By SVG spec any invalid data inside path data should stop the parsing of this data,
    ///   but not the whole document.
    ///
    ///   This function will return `EndOfStream` on any kind of error.
    ///   Library will print a warning to stderr.
    ///
    ///   In other words - you will get as much data as possible.
    ///
    ///   Example: `M 10 20 L 30 40 #!@$1 L 50 60` -> `M 10 20 L 30 40`
    ///
    /// - We don't support implicit commands, so all commands will be converted to explicit one.
    ///   It mostly affects implicit MoveTo, which will be converted, according to spec,
    ///   into explicit LineTo.
    ///
    ///   Example: `M 10 20 30 40 50 60` -> `M 10 20 L 30 40 L 50 60`
    fn parse_next(&mut self) -> Result<Token, Error> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return Ok(Token::EndOfStream);
        }

        macro_rules! data_error {
            () => ({
                warnln!("Invalid path data at {}. The remaining data is ignored.",
                         s.gen_error_pos());
                return Ok(Token::EndOfStream);
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

        let has_prev_cmd = self.prev_cmd.is_some();
        let first_char = s.curr_char_raw();

        if !has_prev_cmd && !is_cmd(first_char) {
            warnln!("'{}' is not a command. \
                     The remaining data is ignored.", first_char as char);
            return Ok(Token::EndOfStream);
        }

        if !has_prev_cmd {
            match first_char {
                b'M' | b'm' => {}
                _ => {
                    warnln!("Warning: First segment must be MoveTo. \
                             The remaining data is ignored.");
                    return Ok(Token::EndOfStream);
                }
            }
        }

        let is_implicit_move_to;
        let cmd: u8;
        if is_cmd(first_char) {
            is_implicit_move_to = false;

            cmd = first_char;
            s.advance_raw(1);
        } else if is_digit(first_char) && has_prev_cmd {
            // unwrap is safe, because we checked 'has_prev_cmd'
            let prev_cmd = self.prev_cmd.unwrap();

            // ClosePath can't be followed by a number
            if prev_cmd == b'Z' || prev_cmd == b'z' {
                data_error!();
            }

            if prev_cmd == b'M' || prev_cmd == b'm' {
                // 'If a moveto is followed by multiple pairs of coordinates, the subsequent
                // pairs are treated as implicit lineto commands.'
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
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());

                Token::MoveTo { abs: absolute, x: x, y: y }
            }
            b'l' => {
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());
                Token::LineTo { abs: absolute, x: x, y: y }
            }
            b'h' => {
                let x = try_num!(s.parse_list_number());
                Token::HorizontalLineTo { abs: absolute, x: x }
            }
            b'v' => {
                let y = try_num!(s.parse_list_number());
                Token::VerticalLineTo { abs: absolute, y: y }
            }
            b'c' => {
                let x1 = try_num!(s.parse_list_number());
                let y1 = try_num!(s.parse_list_number());
                let x2 = try_num!(s.parse_list_number());
                let y2 = try_num!(s.parse_list_number());
                let x  = try_num!(s.parse_list_number());
                let y  = try_num!(s.parse_list_number());
                Token::CurveTo {
                    abs: absolute,
                    x1: x1,
                    y1: y1,
                    x2: x2,
                    y2: y2,
                    x: x,
                    y: y,
                }
            }
            b's' => {
                let x2 = try_num!(s.parse_list_number());
                let y2 = try_num!(s.parse_list_number());
                let x  = try_num!(s.parse_list_number());
                let y  = try_num!(s.parse_list_number());
                Token::SmoothCurveTo {
                    abs: absolute,
                    x2: x2,
                    y2: y2,
                    x: x,
                    y: y,
                }
            }
            b'q' => {
                let x1 = try_num!(s.parse_list_number());
                let y1 = try_num!(s.parse_list_number());
                let x  = try_num!(s.parse_list_number());
                let y  = try_num!(s.parse_list_number());
                Token::Quadratic {
                    abs: absolute,
                    x1: x1,
                    y1: y1,
                    x: x,
                    y: y,
                }
            }
            b't' => {
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());
                Token::SmoothQuadratic { abs: absolute, x: x, y: y }
            }
            b'a' => {
                let rx = try_num!(s.parse_list_number());
                let ry = try_num!(s.parse_list_number());
                let angle = try_num!(s.parse_list_number());

                // By SVG spec 'large-arc' and 'sweep' must contain only one char
                // and can be written without any separators, aka: 10 20 30 01 10 20.
                let la = try_num!(parse_flag(s));
                let sweep = try_num!(parse_flag(s));

                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());

                Token::EllipticalArc {
                    abs: absolute,
                    rx: rx,
                    ry: ry,
                    x_axis_rotation: angle,
                    large_arc: la,
                    sweep: sweep,
                    x: x,
                    y: y,
                }
            }
            b'z' => Token::ClosePath { abs: absolute },
            _ => unreachable!(),
        };

        if is_implicit_move_to {
            self.prev_cmd = if is_absolute(cmd) {
                Some(b'M')
            } else {
                Some(b'm')
            };
        } else {
            self.prev_cmd = Some(cmd);
        }

        Ok(token)
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

fn parse_flag(s: &mut Stream) -> Result<bool, Error> {
    s.skip_spaces();
    let c = s.curr_char()?;
    match c {
        b'0' | b'1' => {
            s.advance(1)?;
            if s.is_char_eq(b',')? {
                s.advance(1)?;
            }
            s.skip_spaces();

            Ok(c == b'1')
        }
        _ => {
            // error type is not relevant since it will be ignored
            Err(Error::UnexpectedEndOfStream(ErrorPos::new(0, 0)))
        }
    }
}
