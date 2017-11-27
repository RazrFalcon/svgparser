// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    FromSpan,
};
use svgparser::path::{
    Tokenizer,
    Token,
};

macro_rules! test {
    ($name:ident, $text:expr, $( $seg:expr ),*) => (
        #[test]
        fn $name() {
            let mut s = Tokenizer::from_str($text);
            $(
                assert_eq!(s.next().unwrap(), $seg);
            )*

            assert_eq!(s.next().is_none(), true);
        }
    )
}

test!(null, "", );
test!(not_a_path, "q", );
test!(not_a_move_to, "L 20 30", );
test!(stop_on_err_1, "M 10 20 L 30 40 L 50",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 }
);

test!(move_to_1, "M 10 20", Token::MoveTo { abs: true, x: 10.0, y: 20.0 });
test!(move_to_2, "m 10 20", Token::MoveTo { abs: false, x: 10.0, y: 20.0 });
test!(move_to_3, "M 10 20 30 40 50 60",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::LineTo { abs: true, x: 50.0, y: 60.0 }
);
test!(move_to_4, "M 10 20 30 40 50 60 M 70 80 90 100 110 120",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::LineTo { abs: true, x: 50.0, y: 60.0 },
    Token::MoveTo { abs: true, x: 70.0, y: 80.0 },
    Token::LineTo { abs: true, x: 90.0, y: 100.0 },
    Token::LineTo { abs: true, x: 110.0, y: 120.0 }
);

test!(arc_to_1, "M 10 20 A 5 5 30 1 1 20 20",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::EllipticalArc {
        abs: true,
        rx: 5.0, ry: 5.0,
        x_axis_rotation: 30.0,
        large_arc: true, sweep: true,
        x: 20.0, y: 20.0
    }
);

test!(arc_to_2, "M 10 20 a 5 5 30 0 0 20 20",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::EllipticalArc {
        abs: false,
        rx: 5.0, ry: 5.0,
        x_axis_rotation: 30.0,
        large_arc: false, sweep: false,
        x: 20.0, y: 20.0
    }
);

test!(arc_to_10, "M10-20A5.5.3-4 010-.1",
    Token::MoveTo { abs: true, x: 10.0, y: -20.0 },
    Token::EllipticalArc {
        abs: true,
        rx: 5.5, ry: 0.3,
        x_axis_rotation: -4.0,
        large_arc: false, sweep: true,
        x: 0.0, y: -0.1
    }
);

test!(separator_1, "M 10 20 L 5 15 C 10 20 30 40 50 60",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 5.0, y: 15.0 },
    Token::CurveTo {
        abs: true,
        x1: 10.0, y1: 20.0,
        x2: 30.0, y2: 40.0,
        x:  50.0, y:  60.0,
    }
);

test!(separator_2, "M 10, 20 L 5, 15 C 10, 20 30, 40 50, 60",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 5.0, y: 15.0 },
    Token::CurveTo {
        abs: true,
        x1: 10.0, y1: 20.0,
        x2: 30.0, y2: 40.0,
        x:  50.0, y:  60.0,
    }
);

test!(separator_3, "M 10,20 L 5,15 C 10,20 30,40 50,60",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 5.0, y: 15.0 },
    Token::CurveTo {
        abs: true,
        x1: 10.0, y1: 20.0,
        x2: 30.0, y2: 40.0,
        x:  50.0, y:  60.0,
    }
);

test!(separator_4, "M10, 20 L5, 15 C10, 20 30 40 50 60",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 5.0, y: 15.0 },
    Token::CurveTo {
        abs: true,
        x1: 10.0, y1: 20.0,
        x2: 30.0, y2: 40.0,
        x:  50.0, y:  60.0,
    }
);

test!(separator_5, "M10 20V30H40V50H60Z",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::VerticalLineTo { abs: true, y: 30.0 },
    Token::HorizontalLineTo { abs: true, x: 40.0 },
    Token::VerticalLineTo { abs: true, y: 50.0 },
    Token::HorizontalLineTo { abs: true, x: 60.0 },
    Token::ClosePath { abs: true }
);

test!(all_segments_1, "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 S 130 140 150 160
    Q 170 180 190 200 T 210 220 A 50 50 30 1 1 230 240 Z",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::HorizontalLineTo { abs: true, x: 50.0 },
    Token::VerticalLineTo { abs: true, y: 60.0 },
    Token::CurveTo {
        abs: true,
        x1:  70.0, y1:  80.0,
        x2:  90.0, y2: 100.0,
        x:  110.0, y:  120.0,
    },
    Token::SmoothCurveTo {
        abs: true,
        x2: 130.0, y2: 140.0,
        x:  150.0, y:  160.0,
    },
    Token::Quadratic {
        abs: true,
        x1: 170.0, y1: 180.0,
        x:  190.0, y:  200.0,
    },
    Token::SmoothQuadratic { abs: true, x: 210.0, y: 220.0 },
    Token::EllipticalArc {
        abs: true,
        rx: 50.0, ry: 50.0,
        x_axis_rotation: 30.0,
        large_arc: true, sweep: true,
        x: 230.0, y: 240.0
    },
    Token::ClosePath { abs: true }
);

test!(all_segments_2, "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 s 130 140 150 160
    q 170 180 190 200 t 210 220 a 50 50 30 1 1 230 240 z",
    Token::MoveTo { abs: false, x: 10.0, y: 20.0 },
    Token::LineTo { abs: false, x: 30.0, y: 40.0 },
    Token::HorizontalLineTo { abs: false, x: 50.0 },
    Token::VerticalLineTo { abs: false, y: 60.0 },
    Token::CurveTo {
        abs: false,
        x1:  70.0, y1:  80.0,
        x2:  90.0, y2: 100.0,
        x:  110.0, y:  120.0,
    },
    Token::SmoothCurveTo {
        abs: false,
        x2: 130.0, y2: 140.0,
        x:  150.0, y:  160.0,
    },
    Token::Quadratic {
        abs: false,
        x1: 170.0, y1: 180.0,
        x:  190.0, y:  200.0,
    },
    Token::SmoothQuadratic { abs: false, x: 210.0, y: 220.0 },
    Token::EllipticalArc {
        abs: false,
        rx: 50.0, ry: 50.0,
        x_axis_rotation: 30.0,
        large_arc: true, sweep: true,
        x: 230.0, y: 240.0
    },
    Token::ClosePath { abs: false }
);

test!(close_path_1, "M10 20 L 30 40 ZM 100 200 L 300 400",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::ClosePath { abs: true },
    Token::MoveTo { abs: true, x: 100.0, y: 200.0 },
    Token::LineTo { abs: true, x: 300.0, y: 400.0 }
);

test!(close_path_2, "M10 20 L 30 40 zM 100 200 L 300 400",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::ClosePath { abs: false },
    Token::MoveTo { abs: true, x: 100.0, y: 200.0 },
    Token::LineTo { abs: true, x: 300.0, y: 400.0 }
);

test!(close_path_3, "M10 20 L 30 40 Z Z Z",
    Token::MoveTo { abs: true, x: 10.0, y: 20.0 },
    Token::LineTo { abs: true, x: 30.0, y: 40.0 },
    Token::ClosePath { abs: true },
    Token::ClosePath { abs: true },
    Token::ClosePath { abs: true }
);

// first token should be EndOfStream
test!(invalid_1, "M\t.", );

#[test]
fn invalid_2() {
    // ClosePath can't be followed by a number
    let mut s = Tokenizer::from_str("M0 0 Z 2");
    assert_eq!(s.next().unwrap(), Token::MoveTo { abs: true, x: 0.0, y: 0.0 });
    assert_eq!(s.next().unwrap(), Token::ClosePath { abs: true });
    assert_eq!(s.next(), None);
}
