// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::path::{Tokenizer, SegmentToken, Segment, SegmentData};

macro_rules! test {
    ($name:ident, $text:expr, $( $seg:expr ),*) => (
        #[test]
        fn $name() {
            let mut s = Tokenizer::from_str($text);
            $(
                assert_eq!(s.parse_next().unwrap(), SegmentToken::Segment($seg));
            )*
            assert_eq!(s.parse_next().unwrap(), SegmentToken::EndOfStream);
        }
    )
}

test!(null, "", );
test!(not_a_path, "q", );
test!(not_a_move_to, "L 20 30", );
test!(stop_on_err_1, "M 10 20 L 30 40 L 50",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } }
);

test!(move_to_1, "M 10 20", Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } });
test!(move_to_2, "m 10 20", Segment { cmd: b'm', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } });
test!(move_to_3, "M 10 20 30 40 50 60",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 50.0, y: 60.0 } }
);
test!(move_to_4, "M 10 20 30 40 50 60 M 70 80 90 100 110 120",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 50.0, y: 60.0 } },
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 70.0, y: 80.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 90.0, y: 100.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 110.0, y: 120.0 } }
);

test!(arc_to_1, "M 10 20 A 5 5 30 1 1 20 20",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'A', data: SegmentData::EllipticalArc {
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 20.0, y: 20.0
        }
    }
);

test!(arc_to_2, "M 10 20 a 5 5 30 0 0 20 20",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'a', data: SegmentData::EllipticalArc {
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: false, sweep: false,
            x: 20.0, y: 20.0
        }
    }
);

test!(arc_to_10, "M10-20A5.5.3-4 010-.1",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: -20.0 } },
    Segment { cmd: b'A', data: SegmentData::EllipticalArc {
            rx: 5.5, ry: 0.3,
            x_axis_rotation: -4.0,
            large_arc: false, sweep: true,
            x: 0.0, y: -0.1
        }
    }
);

test!(separator_1, "M 10 20 L 5 15 C 10 20 30 40 50 60",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 5.0, y: 15.0 } },
    Segment { cmd: b'C', data: SegmentData::CurveTo {
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    }
);

test!(separator_2, "M 10, 20 L 5, 15 C 10, 20 30, 40 50, 60",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 5.0, y: 15.0 } },
    Segment { cmd: b'C', data: SegmentData::CurveTo {
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    }
);

test!(separator_3, "M 10,20 L 5,15 C 10,20 30,40 50,60",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 5.0, y: 15.0 } },
    Segment { cmd: b'C', data: SegmentData::CurveTo {
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    }
);

test!(separator_4, "M10, 20 L5, 15 C10, 20 30 40 50 60",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 5.0, y: 15.0 } },
    Segment { cmd: b'C', data: SegmentData::CurveTo {
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    }
);

test!(separator_5, "M10 20V30H40V50H60Z",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'V', data: SegmentData::VerticalLineTo { y: 30.0 } },
    Segment { cmd: b'H', data: SegmentData::HorizontalLineTo { x: 40.0 } },
    Segment { cmd: b'V', data: SegmentData::VerticalLineTo { y: 50.0 } },
    Segment { cmd: b'H', data: SegmentData::HorizontalLineTo { x: 60.0 } },
    Segment { cmd: b'Z', data: SegmentData::ClosePath }
);

test!(all_segments_1, "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 S 130 140 150 160
    Q 170 180 190 200 T 210 220 A 50 50 30 1 1 230 240 Z",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'H', data: SegmentData::HorizontalLineTo { x: 50.0 } },
    Segment { cmd: b'V', data: SegmentData::VerticalLineTo { y: 60.0 } },
    Segment { cmd: b'C', data: SegmentData::CurveTo {
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        }
    },
    Segment { cmd: b'S', data: SegmentData::SmoothCurveTo {
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        }
    },
    Segment { cmd: b'Q', data: SegmentData::Quadratic {
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        }
    },
    Segment { cmd: b'T', data: SegmentData::SmoothQuadratic { x: 210.0, y: 220.0 } },
    Segment { cmd: b'A', data: SegmentData::EllipticalArc {
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        }
    },
    Segment { cmd: b'Z', data: SegmentData::ClosePath }
);

test!(all_segments_2, "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 s 130 140 150 160
    q 170 180 190 200 t 210 220 a 50 50 30 1 1 230 240 z",
    Segment { cmd: b'm', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'l', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'h', data: SegmentData::HorizontalLineTo { x: 50.0 } },
    Segment { cmd: b'v', data: SegmentData::VerticalLineTo { y: 60.0 } },
    Segment { cmd: b'c', data: SegmentData::CurveTo {
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        }
    },
    Segment { cmd: b's', data: SegmentData::SmoothCurveTo {
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        }
    },
    Segment { cmd: b'q', data: SegmentData::Quadratic {
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        }
    },
    Segment { cmd: b't', data: SegmentData::SmoothQuadratic { x: 210.0, y: 220.0 } },
    Segment { cmd: b'a', data: SegmentData::EllipticalArc {
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        }
    },
    Segment { cmd: b'z', data: SegmentData::ClosePath }
);

test!(close_path_1, "M10 20 L 30 40 ZM 100 200 L 300 400",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'Z', data: SegmentData::ClosePath },
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 100.0, y: 200.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 300.0, y: 400.0 } }
);

test!(close_path_2, "M10 20 L 30 40 zM 100 200 L 300 400",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'z', data: SegmentData::ClosePath },
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 100.0, y: 200.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 300.0, y: 400.0 } }
);

test!(close_path_3, "M10 20 L 30 40 Z Z Z",
    Segment { cmd: b'M', data: SegmentData::MoveTo { x: 10.0, y: 20.0 } },
    Segment { cmd: b'L', data: SegmentData::LineTo { x: 30.0, y: 40.0 } },
    Segment { cmd: b'Z', data: SegmentData::ClosePath },
    Segment { cmd: b'Z', data: SegmentData::ClosePath },
    Segment { cmd: b'Z', data: SegmentData::ClosePath }
);

// first token should be EndOfStream
test!(invalid_1, "M\t.", );

#[test]
fn invalid_2() {
    // ClosePath can't be followed by a number
    let mut s = Tokenizer::from_str("M0 0 Z 2");
    assert_eq!(s.parse_next().unwrap(),
        SegmentToken::Segment(Segment { cmd: b'M', data: SegmentData::MoveTo { x: 0.0, y: 0.0 } }));
    assert_eq!(s.parse_next().unwrap(),
        SegmentToken::Segment(Segment { cmd: b'Z', data: SegmentData::ClosePath }));
    assert_eq!(s.parse_next().unwrap(), SegmentToken::EndOfStream);
}
