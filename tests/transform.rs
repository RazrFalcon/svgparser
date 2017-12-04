// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    FromSpan,
    ChainedErrorExt,
};
use svgparser::transform::{
    Tokenizer,
    Token,
};

macro_rules! test {
    ($name:ident, $text:expr, $($value:expr),*) => (
        #[test]
        fn $name() {
            let mut ts = Tokenizer::from_str($text);
            $(
                assert_eq!(ts.next().unwrap().unwrap(), $value);
            )*

            assert_eq!(ts.next().is_none(), true);
        }
    )
}

test!(matrix_1, "matrix(1 0 0 1 10 20)",
    Token::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(matrix_2, "matrix(1, 0, 0, 1, 10, 20)",
    Token::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(matrix_3, " matrix ( 1, 0, 0, 1, 10, 20 )",
    Token::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 10.0, f: 20.0 }
);

test!(translate_1, "translate(10 20)",
    Token::Translate { tx: 10.0, ty: 20.0 }
);

test!(translate_2, "translate(10)",
    Token::Translate { tx: 10.0, ty: 0.0 }
);

test!(scale_1, "scale(2 3)",
    Token::Scale { sx: 2.0, sy: 3.0 }
);

test!(scale_2, "scale(2)",
    Token::Scale { sx: 2.0, sy: 2.0 }
);

test!(rotate_1, "rotate(30)",
    Token::Rotate { angle: 30.0 }
);

test!(rotate_2, "rotate(30 10 20)",
    Token::Translate { tx: 10.0, ty: 20.0 },
    Token::Rotate { angle: 30.0 },
    Token::Translate { tx: -10.0, ty: -20.0 }
);

test!(skew_y_1, "skewX(30)",
    Token::SkewX { angle: 30.0 }
);

test!(skew_x_1, "skewY(30)",
    Token::SkewY { angle: 30.0 }
);

test!(ts_list_1, "translate(-10,-20) scale(2) rotate(45) translate(5,10)",
    Token::Translate { tx: -10.0, ty: -20.0 },
    Token::Scale { sx: 2.0, sy: 2.0 },
    Token::Rotate { angle: 45.0 },
    Token::Translate { tx: 5.0, ty: 10.0 }
);

test!(ts_list_2, "translate(10,20), scale(2) ,  rotate(45),",
    Token::Translate { tx: 10.0, ty: 20.0 },
    Token::Scale { sx: 2.0, sy: 2.0 },
    Token::Rotate { angle: 45.0 }
);

#[test]
fn error_1() {
    let mut ts = Tokenizer::from_str("text");
    assert_eq!(ts.next().unwrap().unwrap_err().full_chain(),
               "Error: unexpected end of stream");
}

#[test]
fn error_2() {
    let mut ts = Tokenizer::from_str("scale(2) text");
    let _ = ts.next().unwrap();
    assert_eq!(ts.next().unwrap().unwrap_err().full_chain(),
               "Error: unexpected end of stream");
}

#[test]
fn error_3() {
    let mut ts = Tokenizer::from_str(" ");
    assert_eq!(ts.next().is_none(), true);
}

#[test]
fn error_4() {
    let mut ts = Tokenizer::from_str("???G");
    assert_eq!(ts.next().unwrap().unwrap_err().full_chain(),
               "Error: invalid name token");
}
