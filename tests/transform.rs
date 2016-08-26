// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Stream, Error, ErrorPos};
use svgparser::transform;
use svgparser::transform::Transform;

#[test]
fn empty_1() {
    let s = Stream::new(b" ");
    let mut ts = transform::Tokenizer::new(s);
    assert_eq!(ts.next().is_none(), true);
}

macro_rules! test {
    ($name:ident, $text:expr, $($value:expr),*) => (
        #[test]
        fn $name() {
            let s = Stream::new($text);
            let mut ts = transform::Tokenizer::new(s);
            $(
                assert_eq!(ts.parse_next().unwrap(), $value);
            )*
            assert_eq!(ts.parse_next().err().unwrap(), Error::EndOfStream);
        }
    )
}

test!(matrix_1, b"matrix(1 0 0 1 10 20)",
    Transform::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(matrix_2, b"matrix(1, 0, 0, 1, 10, 20)",
    Transform::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(matrix_3, b" matrix ( 1, 0, 0, 1, 10, 20 )",
    Transform::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(translate_1, b"translate(10 20)",
    Transform::Translate { tx: 10.0, ty: 20.0 }
);

test!(translate_2, b"translate(10)",
    Transform::Translate { tx: 10.0, ty: 0.0 }
);

test!(scale_1, b"scale(2 3)",
    Transform::Scale { sx: 2.0, sy: 3.0 }
);

test!(scale_2, b"scale(2)",
    Transform::Scale { sx: 2.0, sy: 2.0 }
);

test!(rotate_1, b"rotate(30)",
    Transform::Rotate { angle: 30.0 }
);

test!(rotate_2, b"rotate(30 10 20)",
    Transform::Translate { tx: 10.0, ty: 20.0 },
    Transform::Rotate { angle: 30.0 },
    Transform::Translate { tx: -10.0, ty: -20.0 }
);

test!(skew_y_1, b"skewX(30)",
    Transform::SkewX { angle: 30.0 }
);

test!(skew_x_1, b"skewY(30)",
    Transform::SkewY { angle: 30.0 }
);

test!(ts_list_1, b"translate(-10,-20) scale(2) rotate(45) translate(5,10)",
    Transform::Translate { tx: -10.0, ty: -20.0 },
    Transform::Scale { sx: 2.0, sy: 2.0 },
    Transform::Rotate { angle: 45.0 },
    Transform::Translate { tx: 5.0, ty: 10.0 }
);

#[test]
fn error_1() {
    let s = Stream::new(b"text");
    let mut ts = transform::Tokenizer::new(s);
    assert_eq!(ts.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1,1)));
}

#[test]
fn error_2() {
    let s = Stream::new(b"scale(2) text");
    let mut ts = transform::Tokenizer::new(s);
    ts.parse_next().unwrap();
    assert_eq!(ts.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1,10)));
}
