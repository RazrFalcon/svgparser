// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Stream, RgbColor, Error, ErrorPos};

macro_rules! test_parse {
    ($name:ident, $text:expr, $color:expr) => {
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            assert_eq!(RgbColor::from_stream(&mut s).unwrap(), $color);
        }
    };
}

test_parse!(
    rrggbb,
    b"#ff0000",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    rrggbb_upper,
    b"#FF0000",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    rgb_hex,
    b"#f00",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    rrggbb_spaced,
    b"  #ff0000  ",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    rgb_numeric,
    b"rgb(254, 203, 231)",
    RgbColor::new(254, 203, 231)
);

test_parse!(
    rgb_numeric_spaced,
    b" rgb( 77 , 77 , 77 ) ",
    RgbColor::new(77, 77, 77)
);

test_parse!(
    rgb_percentage,
    b"rgb(50%, 50%, 50%)",
    RgbColor::new(127, 127, 127)
);

test_parse!(
    rgb_percentage_overflow,
    b"rgb(140%, -10%, 130%)",
    RgbColor::new(255, 0, 255)
);

test_parse!(
    rgb_percentage_float,
    b"rgb(33.333%,46.666%,93.333%)",
    RgbColor::new(85, 119, 238)
);

test_parse!(
    name_red,
    b"red",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    name_red_spaced,
    b" red ",
    RgbColor::new(255, 0, 0)
);

test_parse!(
    name_cornflowerblue,
    b"cornflowerblue",
    RgbColor::new(100, 149, 237)
);

macro_rules! test_error {
    ($name:ident, $text:expr, $err:expr) => {
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            assert_eq!(RgbColor::from_stream(&mut s).err().unwrap(), $err);
        }
    };
}

test_error!(
    case_insensitive_parsing_not_supported_1,
    b"RGB(50, 50, 50)",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_2,
    b"Red",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_3,
    b"RED",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_4,
    b"text",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    icc_color_not_supported_1,
    b"#CD853F icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
    Error::InvalidColor(ErrorPos::new(1, 9))
);

test_error!(
    icc_color_not_supported_2,
    b"red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
    Error::InvalidColor(ErrorPos::new(1, 5))
);

test_error!(
    invalid_input_1,
    b"rgb(-0\x0d",
    Error::UnexpectedEndOfStream(ErrorPos::new(1, 8))
);
