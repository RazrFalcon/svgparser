// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<path>`] data.
//!
//! [`<path>`]: https://www.w3.org/TR/SVG/paths.html#PathData

use super::{Stream, Error, ErrorPos};

/// Representation of path command.
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Command(u8);

// TODO: maybe move to DOM
impl Command {
    /// Constructs a new `Command` from it's name.
    ///
    /// # Examples
    /// ```
    /// use svgparser::path::Command;
    /// assert_eq!(Command::from_cmd(b'M').is_some(), true);
    /// assert_eq!(Command::from_cmd(b'n').is_some(), false);
    /// ```
    pub fn from_cmd(cmd: u8) -> Option<Command> {
        if Command::is_cmd(cmd) {
            Some(Command(cmd))
        } else {
            None
        }
    }

    // TODO: create new_* using macro, somehow.

    /// Constructs a new absolute MoveTo `Command`.
    pub fn new_move_to() -> Command {
        Command(b'M')
    }

    /// Constructs a new absolute ClosePath `Command`.
    pub fn new_close_path() -> Command {
        Command(b'Z')
    }

    /// Constructs a new absolute LineTo `Command`.
    pub fn new_line_to() -> Command {
        Command(b'L')
    }

    /// Constructs a new absolute HorizontalLineTo `Command`.
    pub fn new_hline_to() -> Command {
        Command(b'H')
    }

    /// Constructs a new absolute VerticalLineTo `Command`.
    pub fn new_vline_to() -> Command {
        Command(b'V')
    }

    /// Constructs a new absolute CurveTo `Command`.
    pub fn new_curve_to() -> Command {
        Command(b'C')
    }

    /// Constructs a new absolute SmoothLineTo `Command`.
    pub fn new_smooth_curve_to() -> Command {
        Command(b'S')
    }

    /// Constructs a new absolute QuadTo `Command`.
    pub fn new_quad_to() -> Command {
        Command(b'Q')
    }

    /// Constructs a new absolute SmoothQuadTo `Command`.
    pub fn new_smooth_quad_to() -> Command {
        Command(b'T')
    }

    /// Constructs a new absolute ArcTo `Command`.
    pub fn new_arc_to() -> Command {
        Command(b'A')
    }

    /// Returns content of command.
    pub fn data(&self) -> u8 {
        self.0
    }

    /// Returns `true` if command is MoveTo.
    pub fn is_move_to(&self) -> bool {
        self.0 == b'M' || self.0 == b'm'
    }

    /// Returns `true` if command is ClosePath.
    pub fn is_close_path(&self) -> bool {
        self.0 == b'Z' || self.0 == b'z'
    }

    /// Returns `true` if command is LineTo.
    pub fn is_line_to(&self) -> bool {
        self.0 == b'L' || self.0 == b'l'
    }

    /// Returns `true` if command is HorizontalLineTo.
    pub fn is_hline_to(&self) -> bool {
        self.0 == b'H' || self.0 == b'h'
    }

    /// Returns `true` if command is VerticalLineTo.
    pub fn is_vline_to(&self) -> bool {
        self.0 == b'V' || self.0 == b'v'
    }

    /// Returns `true` if command is CurveTo.
    pub fn is_curve_to(&self) -> bool {
        self.0 == b'C' || self.0 == b'c'
    }

    /// Returns `true` if command is SmoothLineTo.
    pub fn is_smooth_curve_to(&self) -> bool {
        self.0 == b'S' || self.0 == b's'
    }

    /// Returns `true` if command is QuadTo.
    pub fn is_quad_to(&self) -> bool {
        self.0 == b'Q' || self.0 == b'q'
    }

    /// Returns `true` if command is SmoothQuadTo.
    pub fn is_smooth_quad_to(&self) -> bool {
        self.0 == b'T' || self.0 == b't'
    }

    /// Returns `true` if command is ArcTo.
    pub fn is_arc_to(&self) -> bool {
        self.0 == b'A' || self.0 == b'a'
    }

    /// Checks that command is absolute.
    ///
    /// # Examples
    /// ```
    /// use svgparser::path::Command;
    /// assert_eq!(Command::new_move_to().is_absolute(), true);
    /// assert_eq!(Command::new_move_to().to_relative().is_absolute(), false);
    /// ```
    pub fn is_absolute(&self) -> bool {
        match self.0 {
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

    /// Checks that command is relative.
    ///
    /// # Examples
    /// ```
    /// use svgparser::path::Command;
    /// assert_eq!(Command::new_move_to().is_relative(), false);
    /// assert_eq!(Command::new_move_to().to_relative().is_relative(), true);
    /// ```
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Converts command into absolute.
    ///
    /// # Examples
    /// ```
    /// use svgparser::path::Command;
    /// assert_eq!(Command::from_cmd(b'm').unwrap().to_absolute().is_absolute(), true);
    /// ```
    pub fn to_absolute(&self) -> Command {
        let c = match self.0 {
            b'm' => b'M',
            b'z' => b'Z',
            b'l' => b'L',
            b'h' => b'H',
            b'v' => b'V',
            b'c' => b'C',
            b's' => b'S',
            b'q' => b'Q',
            b't' => b'T',
            b'a' => b'A',
            _ => self.0,
        };
        Command(c)
    }

    /// Converts command into relative.
    ///
    /// # Examples
    /// ```
    /// use svgparser::path::Command;
    /// assert_eq!(Command::from_cmd(b'M').unwrap().to_relative().is_relative(), true);
    /// ```
    pub fn to_relative(&self) -> Command {
        let c = match self.0 {
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
            _ => self.0,
        };
        Command(c)
    }

    fn is_cmd(c: u8) -> bool {
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
}

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
    cmd: Command,
    data: SegmentData,
}

impl Segment {
    /// Constructs a new MoveTo `Segment`.
    pub fn new_move_to(x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_move_to(),
            data: SegmentData::MoveTo { x: x, y: y },
        }
    }

    /// Constructs a new ClosePath `Segment`.
    pub fn new_close_path() -> Segment {
        Segment {
            cmd: Command::new_close_path(),
            data: SegmentData::ClosePath,
        }
    }

    /// Constructs a new LineTo `Segment`.
    pub fn new_line_to(x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_line_to(),
            data: SegmentData::LineTo { x: x, y: y },
        }
    }

    /// Constructs a new HorizontalLineTo `Segment`.
    pub fn new_hline_to(x: f64) -> Segment {
        Segment {
            cmd: Command::new_hline_to(),
            data: SegmentData::HorizontalLineTo { x: x },
        }
    }

    /// Constructs a new VerticalLineTo `Segment`.
    pub fn new_vline_to(y: f64) -> Segment {
        Segment {
            cmd: Command::new_vline_to(),
            data: SegmentData::VerticalLineTo { y: y },
        }
    }

    /// Constructs a new CurveTo `Segment`.
    pub fn new_curve_to(x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_curve_to(),
            data: SegmentData::CurveTo {
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y2,
                x: x,
                y: y,
            },
        }
    }

    /// Constructs a new SmoothCurveTo `Segment`.
    pub fn new_smooth_curve_to(x2: f64, y2: f64, x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_smooth_curve_to(),
            data: SegmentData::SmoothCurveTo {
                x2: x2,
                y2: y2,
                x: x,
                y: y,
            },
        }
    }

    /// Constructs a new QuadTo `Segment`.
    pub fn new_quad_to(x1: f64, y1: f64, x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_quad_to(),
            data: SegmentData::Quadratic {
                x1: x1,
                y1: y1,
                x: x,
                y: y,
            },
        }
    }

    /// Constructs a new SmoothQuadTo `Segment`.
    pub fn new_smooth_quad_to(x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_smooth_quad_to(),
            data: SegmentData::SmoothQuadratic {
                x: x,
                y: y,
            },
        }
    }

    /// Constructs a new ArcTo `Segment`.
    pub fn new_arc_to(rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool,
                  x: f64, y: f64) -> Segment {
        Segment {
            cmd: Command::new_arc_to(),
            data: SegmentData::EllipticalArc {
                rx: rx,
                ry: ry,
                x_axis_rotation: x_axis_rotation,
                large_arc: large_arc,
                sweep: sweep,
                x: x,
                y: y,
            },
        }
    }

    /// Returns segment's command.
    pub fn cmd(&self) -> &Command {
        &self.cmd
    }

    /// Returns segment's data.
    pub fn data(&self) -> &SegmentData {
        &self.data
    }

    /// Returns relative copy of segment.
    pub fn to_relative(mut self) -> Segment {
        self.cmd = self.cmd.to_relative();
        self
    }

    /// Returns absolute copy of segment.
    pub fn to_absolute(mut self) -> Segment {
        self.cmd = self.cmd.to_absolute();
        self
    }
}

/// Tokenizer for \<path\> data.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<Command>,
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

        if !has_prev_cmd && !Command::is_cmd(first_char) {
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
        let cmd: Command;
        if Command::is_cmd(first_char) {
            is_implicit_move_to = false;

            cmd = Command::from_cmd(first_char).unwrap();
            s.advance_raw(1);
        } else if is_digit(first_char) && has_prev_cmd {
            let prev_cmd = self.prev_cmd.unwrap();

            if prev_cmd.data() == b'M' || prev_cmd.data() == b'm' {
                // 'If a moveto is followed by multiple pairs of coordinates, the subsequent
                // pairs are treated as implicit lineto commands.'
                // So we parse them as lineto.
                is_implicit_move_to = true;
                cmd = if prev_cmd.is_absolute() {
                    Command::from_cmd(b'L').unwrap()
                } else {
                    Command::from_cmd(b'l').unwrap()
                };
            } else {
                is_implicit_move_to = false;
                cmd = prev_cmd;
            }
        } else {
            data_error!();
        }

        let cmdl = cmd.to_relative();

        let data = match cmdl.data() {
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

        if cmdl.data() == b'z' {
            self.prev_cmd = None;
        } else if is_implicit_move_to {
            self.prev_cmd = if cmd.is_absolute() {
                Some(Command::from_cmd(b'M').unwrap())
            } else {
                Some(Command::from_cmd(b'm').unwrap())
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
