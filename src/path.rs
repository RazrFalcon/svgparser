// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<path>`] data.
//!
//! [`<path>`]: https://www.w3.org/TR/SVG/paths.html#PathData

use super::{Stream, Error, ErrorPos};

#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(missing_docs)]
pub enum SegmentData {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    HorizontalLineTo {
        x: f64,
    },
    VerticalLineTo {
        y: f64,
    },
    CurveTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    SmoothCurveTo {
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    SmoothQuadratic {
        x: f64,
        y: f64,
    },
    EllipticalArc {
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    },
    ClosePath,
}

/// Representation of path segment.
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Segment {
    /// Command type.
    pub cmd: u8,
    /// Points.
    pub data: SegmentData,
}

/// Tokenizer for \<path\> data.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> Tokenizer<'a> {
    /// Constructs a new `Tokenizer`.
    pub fn new(stream: Stream<'a>) -> Tokenizer<'a> {
        Tokenizer {
            stream: stream,
            prev_cmd: None,
        }
    }

    /// Extracts next path data segment from stream.
    ///
    /// # Errors
    ///
    /// - `Error::EndOfStream` indicates end of parsing, not error.
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - By SVG spec any invalid data inside path data should stop parsing of this data,
    ///   but not the whole document.
    ///
    ///   This function will return `Error::EndOfStream` on any kind of error.
    ///   Library will print warning to stdout.
    ///
    ///   In other words - you will get as much data as possible.
    ///
    ///   Example: `M 10 20 L 30 40 #!@$1 L 50 60` -> `M 10 20 L 30 40`
    ///
    /// - We don't support implicit commands, so all commands will be converted to explicit one.
    ///   It's mostly affects implicit MoveTo, which will be converted, according to spec,
    ///   into explicit LineTo.
    ///
    ///   Example: `M 10 20 30 40 50 60` -> `M 10 20 L 30 40 L 50 60`
    pub fn parse_next(&mut self) -> Result<Segment, Error> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return Err(Error::EndOfStream);
        }

        macro_rules! data_error {
            () => ({
                println!("Warning: invalid path data at {:?}. \
                          The remaining data is ignored.", s.gen_error_pos());
                return Err(Error::EndOfStream);
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
            println!("Warning: '{}' not a command. \
                      The remaining data is ignored.", first_char as char);
            return Err(Error::EndOfStream);
        }

        if !has_prev_cmd {
            match first_char {
                b'M' | b'm' => {}
                _ => {
                    println!("Warning: first segment must be MoveTo. \
                              The remaining data is ignored.");
                    return Err(Error::EndOfStream);
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
            let prev_cmd = self.prev_cmd.unwrap();

            if prev_cmd == b'M' || prev_cmd == b'm' {
                // 'If a moveto is followed by multiple pairs of coordinates, the subsequent
                // pairs are treated as implicit lineto commands.'
                // So we parse them as lineto.
                is_implicit_move_to = true;
                cmd = if is_absolute(prev_cmd) {
                    b'L'
                } else {
                    b'l'
                };
            } else {
                is_implicit_move_to = false;
                cmd = prev_cmd;
            }
        } else {
            data_error!();
        }

        let cmdl = to_relative(cmd);

        let data = match cmdl {
            b'm' => {
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());

                SegmentData::MoveTo { x: x, y: y }
            }
            b'l' => {
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());
                SegmentData::LineTo { x: x, y: y }
            }
            b'h' => {
                let x = try_num!(s.parse_list_number());
                SegmentData::HorizontalLineTo { x: x }
            }
            b'v' => {
                let y = try_num!(s.parse_list_number());
                SegmentData::VerticalLineTo { y: y }
            }
            b'c' => {
                let x1 = try_num!(s.parse_list_number());
                let y1 = try_num!(s.parse_list_number());
                let x2 = try_num!(s.parse_list_number());
                let y2 = try_num!(s.parse_list_number());
                let x  = try_num!(s.parse_list_number());
                let y  = try_num!(s.parse_list_number());
                SegmentData::CurveTo {
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
                SegmentData::SmoothCurveTo {
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
                SegmentData::Quadratic {
                    x1: x1,
                    y1: y1,
                    x: x,
                    y: y,
                }
            }
            b't' => {
                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());
                SegmentData::SmoothQuadratic { x: x, y: y }
            }
            b'a' => {
                let rx = try_num!(s.parse_list_number());
                let ry = try_num!(s.parse_list_number());
                let angle = try_num!(s.parse_list_number());

                // By SVG spec 'large-arc' and 'sweep' must contain only one char,
                // and can be written without any separators, aka: 10 20 30 01 10 20.
                let la = try_num!(parse_flag(s));
                let sweep = try_num!(parse_flag(s));

                let x = try_num!(s.parse_list_number());
                let y = try_num!(s.parse_list_number());

                SegmentData::EllipticalArc {
                    rx: rx,
                    ry: ry,
                    x_axis_rotation: angle,
                    large_arc: la,
                    sweep: sweep,
                    x: x,
                    y: y,
                }
            }
            b'z' => SegmentData::ClosePath,
            _ => unreachable!(),
        };

        if cmdl == b'z' {
            self.prev_cmd = None;
        } else if is_implicit_move_to {
            self.prev_cmd = if is_absolute(cmd) {
                Some(b'M')
            } else {
                Some(b'm')
            };
        } else {
            self.prev_cmd = Some(cmd);
        }

        Ok(Segment {
            cmd: cmd,
            data: data,
        })
    }
}

/// Returns `true` if the selected char is the command.
pub fn is_cmd(c: u8) -> bool {
    match c {
        b'M' | b'm' => true,
        b'Z' | b'z' => true,
        b'L' | b'l' => true,
        b'H' | b'h' => true,
        b'V' | b'v' => true,
        b'C' | b'c' => true,
        b'S' | b's' => true,
        b'Q' | b'q' => true,
        b'T' | b't' => true,
        b'A' | b'a' => true,
        _ => false,
    }
}

/// Returns `true` if the selected char is the absolute command.
pub fn is_absolute(c: u8) -> bool {
    debug_assert!(is_cmd(c));
    match c {
        b'M' => true,
        b'Z' => true,
        b'L' => true,
        b'H' => true,
        b'V' => true,
        b'C' => true,
        b'S' => true,
        b'Q' => true,
        b'T' => true,
        b'A' => true,
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
        b'0'...b'9' => true,
        b'.' | b'-' | b'+' => true,
        _ => false,
    }
}

fn parse_flag(s: &mut Stream) -> Result<bool, Error> {
    s.skip_spaces();
    let c = try!(s.curr_char());
    match c {
        b'0' | b'1' => {
            try!(s.advance(1));
            if try!(s.is_char_eq(b',')) {
                try!(s.advance(1));
            }
            s.skip_spaces();

            Ok(c == b'1')
        }
        _ => {
            // error type is not relevant, since it will be ignored
            Err(Error::UnexpectedEndOfStream(ErrorPos::new(0,0)))
        }
    }
}

impl_iter_for_tokenizer!(Segment);
