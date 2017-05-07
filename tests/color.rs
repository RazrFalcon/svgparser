// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Color, Error, ErrorPos};

macro_rules! test_parse {
    ($name:ident, $text:expr, $color:expr) => {
        #[test]
        fn $name() {
            assert_eq!(Color::from_str($text).unwrap(), $color);
        }
    };
}

test_parse!(
    rrggbb,
    "#ff0000",
    Color::new(255, 0, 0)
);

test_parse!(
    rrggbb_upper,
    "#FF0000",
    Color::new(255, 0, 0)
);

test_parse!(
    rgb_hex,
    "#f00",
    Color::new(255, 0, 0)
);

test_parse!(
    rrggbb_spaced,
    "  #ff0000  ",
    Color::new(255, 0, 0)
);

test_parse!(
    rgb_numeric,
    "rgb(254, 203, 231)",
    Color::new(254, 203, 231)
);

test_parse!(
    rgb_numeric_spaced,
    " rgb( 77 , 77 , 77 ) ",
    Color::new(77, 77, 77)
);

test_parse!(
    rgb_percentage,
    "rgb(50%, 50%, 50%)",
    Color::new(127, 127, 127)
);

test_parse!(
    rgb_percentage_overflow,
    "rgb(140%, -10%, 130%)",
    Color::new(255, 0, 255)
);

test_parse!(
    rgb_percentage_float,
    "rgb(33.333%,46.666%,93.333%)",
    Color::new(85, 119, 238)
);

test_parse!(
    name_red,
    "red",
    Color::new(255, 0, 0)
);

test_parse!(
    name_red_spaced,
    " red ",
    Color::new(255, 0, 0)
);

test_parse!(
    name_cornflowerblue,
    "cornflowerblue",
    Color::new(100, 149, 237)
);

macro_rules! test_error {
    ($name:ident, $text:expr, $err:expr) => {
        #[test]
        fn $name() {
            assert_eq!(Color::from_str($text).err().unwrap(), $err);
        }
    };
}

test_error!(
    case_insensitive_parsing_not_supported_1,
    "RGB(50, 50, 50)",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_2,
    "Red",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_3,
    "RED",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    case_insensitive_parsing_not_supported_4,
    "text",
    Error::InvalidColor(ErrorPos::new(1, 1))
);

test_error!(
    icc_color_not_supported_1,
    "#CD853F icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
    Error::InvalidColor(ErrorPos::new(1, 9))
);

test_error!(
    icc_color_not_supported_2,
    "red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
    Error::InvalidColor(ErrorPos::new(1, 5))
);

test_error!(
    invalid_input_1,
    "rgb(-0\x0d",
    Error::UnexpectedEndOfStream(ErrorPos::new(1, 8))
);
