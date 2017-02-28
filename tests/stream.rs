// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Stream, Length, LengthUnit, Error, ErrorPos};

macro_rules! test_number {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            assert_eq!(s.parse_number().unwrap(), $result);
        }
    )
}

test_number!(number_1,  b"0", 0.0);
test_number!(number_2,  b"1", 1.0);
test_number!(number_3,  b"-1", -1.0);
test_number!(number_4,  b" -1 ", -1.0);
test_number!(number_5,  b"  1  ", 1.0);
test_number!(number_6,  b".4", 0.4);
test_number!(number_7,  b"-.4", -0.4);
test_number!(number_8,  b"-.4text", -0.4);
test_number!(number_9,  b"-.01 text", -0.01);
test_number!(number_10, b"-.01 4", -0.01);
test_number!(number_11, b".0000000000008", 0.0000000000008);
test_number!(number_12, b"1000000000000", 1000000000000.0);
test_number!(number_13, b"123456.123456", 123456.123456);
test_number!(number_14, b"+10", 10.0);
test_number!(number_15, b"1e2", 100.0);
test_number!(number_16, b"1e+2", 100.0);
test_number!(number_17, b"1E2", 100.0);
test_number!(number_18, b"1e-2", 0.01);
test_number!(number_19, b"1ex", 1.0);
test_number!(number_20, b"1em", 1.0);
test_number!(number_21, b"12345678901234567890", 12345678901234567000.0);
test_number!(number_22, b"0.", 0.0);
test_number!(number_23, b"1.3e-2", 0.013);

// ---

macro_rules! test_number_err {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            assert_eq!(s.parse_number().err().unwrap(), $result);
        }
    )
}

test_number_err!(number_err_1, b"q",    Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_2, b"",     Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_3, b"-",    Error::UnexpectedEndOfStream(ErrorPos::new(1,2)));
test_number_err!(number_err_4, b"+",    Error::UnexpectedEndOfStream(ErrorPos::new(1,2)));
test_number_err!(number_err_5, b"-q",   Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_6, b".",    Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_7, b"99999999e99999999",  Error::InvalidNumber(ErrorPos::new(1,1)));
test_number_err!(number_err_8, b"-99999999e99999999", Error::InvalidNumber(ErrorPos::new(1,1)));

// ---

macro_rules! test_length {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            assert_eq!(s.parse_length().unwrap(), $result);
        }
    )
}

test_length!(length_1,  b"1",   Length::new(1.0, LengthUnit::None));
test_length!(length_2,  b"1em", Length::new(1.0, LengthUnit::Em));
test_length!(length_3,  b"1ex", Length::new(1.0, LengthUnit::Ex));
test_length!(length_4,  b"1px", Length::new(1.0, LengthUnit::Px));
test_length!(length_5,  b"1in", Length::new(1.0, LengthUnit::In));
test_length!(length_6,  b"1cm", Length::new(1.0, LengthUnit::Cm));
test_length!(length_7,  b"1mm", Length::new(1.0, LengthUnit::Mm));
test_length!(length_8,  b"1pt", Length::new(1.0, LengthUnit::Pt));
test_length!(length_9,  b"1pc", Length::new(1.0, LengthUnit::Pc));
test_length!(length_10, b"1%",  Length::new(1.0, LengthUnit::Percent));
test_length!(length_11, b"1,",  Length::new(1.0, LengthUnit::None));
test_length!(length_12, b"1 ,", Length::new(1.0, LengthUnit::None));
test_length!(length_13, b"1 1", Length::new(1.0, LengthUnit::None));
test_length!(length_14, b"1e0", Length::new(1.0, LengthUnit::None));
test_length!(length_15, b"1.0e0", Length::new(1.0, LengthUnit::None));
test_length!(length_16, b"1.0e0em", Length::new(1.0, LengthUnit::Em));

#[test]
fn length_err_1() {
    let mut s = Stream::new(b"1q");
    assert_eq!(s.parse_length().err().unwrap(), Error::InvalidLength(ErrorPos::new(1,2)));
}

// ---

#[test]
fn integer_1() {
    let mut s = Stream::new(b"10");
    assert_eq!(s.parse_integer().unwrap(), 10);
}

#[test]
fn integer_err_1() {
    // error because of overflow
    let mut s = Stream::new(b"10000000000000");
    assert_eq!(s.parse_integer().err().unwrap(), Error::InvalidNumber(ErrorPos::new(1,1)));
}
