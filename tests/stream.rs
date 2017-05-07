// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Stream, Length, LengthUnit, Error, ErrorPos};

macro_rules! test_number {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::from_str($text);
            assert_eq!(s.parse_number().unwrap(), $result);
        }
    )
}

test_number!(number_1,  "0", 0.0);
test_number!(number_2,  "1", 1.0);
test_number!(number_3,  "-1", -1.0);
test_number!(number_4,  " -1 ", -1.0);
test_number!(number_5,  "  1  ", 1.0);
test_number!(number_6,  ".4", 0.4);
test_number!(number_7,  "-.4", -0.4);
test_number!(number_8,  "-.4text", -0.4);
test_number!(number_9,  "-.01 text", -0.01);
test_number!(number_10, "-.01 4", -0.01);
test_number!(number_11, ".0000000000008", 0.0000000000008);
test_number!(number_12, "1000000000000", 1000000000000.0);
test_number!(number_13, "123456.123456", 123456.123456);
test_number!(number_14, "+10", 10.0);
test_number!(number_15, "1e2", 100.0);
test_number!(number_16, "1e+2", 100.0);
test_number!(number_17, "1E2", 100.0);
test_number!(number_18, "1e-2", 0.01);
test_number!(number_19, "1ex", 1.0);
test_number!(number_20, "1em", 1.0);
test_number!(number_21, "12345678901234567890", 12345678901234567000.0);
test_number!(number_22, "0.", 0.0);
test_number!(number_23, "1.3e-2", 0.013);

// ---

macro_rules! test_number_err {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::from_str($text);
            assert_eq!(s.parse_number().err().unwrap(), $result);
        }
    )
}

test_number_err!(number_err_1, "q",    Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_2, "",     Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_3, "-",    Error::UnexpectedEndOfStream(ErrorPos::new(1,2)));
test_number_err!(number_err_4, "+",    Error::UnexpectedEndOfStream(ErrorPos::new(1,2)));
test_number_err!(number_err_5, "-q",   Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_6, ".",    Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_7, "99999999e99999999",  Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_8, "-99999999e99999999", Error::InvalidNumber(ErrorPos::new(1,1)));

// ---

macro_rules! test_length {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::from_str($text);
            assert_eq!(s.parse_length().unwrap(), $result);
        }
    )
}

test_length!(length_1,  "1",   Length::new(1.0, LengthUnit::None));
test_length!(length_2,  "1em", Length::new(1.0, LengthUnit::Em));
test_length!(length_3,  "1ex", Length::new(1.0, LengthUnit::Ex));
test_length!(length_4,  "1px", Length::new(1.0, LengthUnit::Px));
test_length!(length_5,  "1in", Length::new(1.0, LengthUnit::In));
test_length!(length_6,  "1cm", Length::new(1.0, LengthUnit::Cm));
test_length!(length_7,  "1mm", Length::new(1.0, LengthUnit::Mm));
test_length!(length_8,  "1pt", Length::new(1.0, LengthUnit::Pt));
test_length!(length_9,  "1pc", Length::new(1.0, LengthUnit::Pc));
test_length!(length_10, "1%",  Length::new(1.0, LengthUnit::Percent));
test_length!(length_11, "1,",  Length::new(1.0, LengthUnit::None));
test_length!(length_12, "1 ,", Length::new(1.0, LengthUnit::None));
test_length!(length_13, "1 1", Length::new(1.0, LengthUnit::None));
test_length!(length_14, "1e0", Length::new(1.0, LengthUnit::None));
test_length!(length_15, "1.0e0", Length::new(1.0, LengthUnit::None));
test_length!(length_16, "1.0e0em", Length::new(1.0, LengthUnit::Em));

#[test]
fn length_err_1() {
    let mut s = Stream::from_str("1q");
    assert_eq!(s.parse_length().unwrap(), Length::new(1.0, LengthUnit::None));
    assert_eq!(s.parse_length().err().unwrap(), Error::InvalidNumber(ErrorPos::new(1,2)));
}

// ---

#[test]
fn integer_1() {
    let mut s = Stream::from_str("10");
    assert_eq!(s.parse_integer().unwrap(), 10);
}

#[test]
fn integer_err_1() {
    // error because of overflow
    let mut s = Stream::from_str("10000000000000");
    assert_eq!(s.parse_integer().err().unwrap(), Error::InvalidNumber(ErrorPos::new(1,1)));
}
