// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Stream, Error};
use svgparser::path;

macro_rules! test {
    ($name:ident, $text:expr, $( $seg:expr ),*) => (
        #[test]
        fn $name() {
            let stream = Stream::new($text);
            let mut s = path::Tokenizer::new(stream);
            $(
                assert_eq!(s.parse_next().unwrap(), $seg);
            )*
            assert_eq!(s.parse_next().is_err(), true);
        }
    )
}

test!(null, b"", );
test!(not_a_path, b"q", );
test!(not_a_move_to, b"L 20 30", );
test!(stop_on_err_1, b"M 10 20 L 30 40 L 50",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0)
);

#[test]
fn stop_on_err_2() {
    let stream = Stream::new(b"M 10 20 L 30 40 L 50");
    let mut s = path::Tokenizer::new(stream);
    let mut vec = Vec::new();
    loop {
        match s.parse_next() {
            Ok(seg) => vec.push(seg),
            Err(e) => {
                match e {
                    Error::EndOfStream => break,
                    _ => panic!("{:?}", e),
                }
            },
        }
    }

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], path::Segment::new_move_to(10.0, 20.0));
    assert_eq!(vec[1], path::Segment::new_line_to(30.0, 40.0));
}

test!(move_to_1, b"M 10 20", path::Segment::new_move_to(10.0, 20.0));
test!(move_to_2, b"m 10 20", path::Segment::new_move_to(10.0, 20.0).to_relative());
test!(move_to_3, b"M 10 20 30 40 50 60",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0),
    path::Segment::new_line_to(50.0, 60.0)
);

test!(move_to_4, b"M 10 20 30 40 50 60 M 70 80 90 100 110 120",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0),
    path::Segment::new_line_to(50.0, 60.0),
    path::Segment::new_move_to(70.0, 80.0),
    path::Segment::new_line_to(90.0, 100.0),
    path::Segment::new_line_to(110.0, 120.0)
);

test!(arc_to_1, b"M 10 20 A 5 5 30 1 1 20 20",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_arc_to(5.0, 5.0, 30.0, true, true, 20.0, 20.0)
);

test!(arc_to_2, b"M 10 20 a 5 5 30 0 0 20 20",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_arc_to(5.0, 5.0, 30.0, false, false, 20.0, 20.0).to_relative()
);

test!(arc_to_10, b"M10-20A5.5.3-4 110-.1",
    path::Segment::new_move_to(10.0, -20.0),
    path::Segment::new_arc_to(5.5, 0.3, -4.0, true, true, 0.0, -0.1)
);

test!(separator_1, b"M 10 20 L 5 15 C 10 20 30 40 50 60",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(5.0, 15.0),
    path::Segment::new_curve_to(10.0, 20.0, 30.0, 40.0, 50.0, 60.0)
);

test!(separator_2, b"M 10, 20 L 5, 15 C 10, 20 30, 40 50, 60",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(5.0, 15.0),
    path::Segment::new_curve_to(10.0, 20.0, 30.0, 40.0, 50.0, 60.0)
);

test!(separator_3, b"M 10,20 L 5,15 C 10,20 30,40 50,60",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(5.0, 15.0),
    path::Segment::new_curve_to(10.0, 20.0, 30.0, 40.0, 50.0, 60.0)
);

test!(separator_4, b"M10, 20 L5, 15 C10, 20 30 40 50 60",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(5.0, 15.0),
    path::Segment::new_curve_to(10.0, 20.0, 30.0, 40.0, 50.0, 60.0)
);

test!(separator_5, b"M10 20V30H40V50H60Z",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_vline_to(30.0),
    path::Segment::new_hline_to(40.0),
    path::Segment::new_vline_to(50.0),
    path::Segment::new_hline_to(60.0),
    path::Segment::new_close_path()
);

test!(all_segments_1, b"M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 S 130 140 150 160
    Q 170 180 190 200 T 210 220 A 50 50 30 1 1 230 240 Z",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0),
    path::Segment::new_hline_to(50.0),
    path::Segment::new_vline_to(60.0),
    path::Segment::new_curve_to(70.0, 80.0, 90.0, 100.0, 110.0, 120.0),
    path::Segment::new_smooth_curve_to(130.0, 140.0, 150.0, 160.0),
    path::Segment::new_quad_to(170.0, 180.0, 190.0, 200.0),
    path::Segment::new_smooth_quad_to(210.0, 220.0),
    path::Segment::new_arc_to(50.0, 50.0, 30.0, true, true, 230.0, 240.0),
    path::Segment::new_close_path()
);

test!(all_segments_2, b"m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 s 130 140 150 160
    q 170 180 190 200 t 210 220 a 50 50 30 1 1 230 240 z",
    path::Segment::new_move_to(10.0, 20.0).to_relative(),
    path::Segment::new_line_to(30.0, 40.0).to_relative(),
    path::Segment::new_hline_to(50.0).to_relative(),
    path::Segment::new_vline_to(60.0).to_relative(),
    path::Segment::new_curve_to(70.0, 80.0, 90.0, 100.0, 110.0, 120.0).to_relative(),
    path::Segment::new_smooth_curve_to(130.0, 140.0, 150.0, 160.0).to_relative(),
    path::Segment::new_quad_to(170.0, 180.0, 190.0, 200.0).to_relative(),
    path::Segment::new_smooth_quad_to(210.0, 220.0).to_relative(),
    path::Segment::new_arc_to(50.0, 50.0, 30.0, true, true, 230.0, 240.0).to_relative(),
    path::Segment::new_close_path().to_relative()
);

test!(close_path_1, b"M10 20 L 30 40 ZM 100 200 L 300 400",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0),
    path::Segment::new_close_path(),
    path::Segment::new_move_to(100.0, 200.0),
    path::Segment::new_line_to(300.0, 400.0)
);

test!(close_path_2, b"M10 20 L 30 40 zM 100 200 L 300 400",
    path::Segment::new_move_to(10.0, 20.0),
    path::Segment::new_line_to(30.0, 40.0),
    path::Segment::new_close_path().to_relative(),
    path::Segment::new_move_to(100.0, 200.0),
    path::Segment::new_line_to(300.0, 400.0)
);

// TODO: process error of 'm 10 20 z L 10 20'
