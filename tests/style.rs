// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::style;
use svgparser::{Stream, Error, ErrorPos};

use std::str;

macro_rules! test_attr {
    ($name:ident, $text:expr, $(($aname:expr, $avalue:expr)),*) => (
        #[test]
        fn $name() {
            let stream = Stream::new($text);
            let mut s = style::Tokenizer::new(stream);
            $(
                match s.parse_next().unwrap() {
                    style::Token::Attribute(name, stream) => {
                        assert_eq!(name, $aname);
                        assert_eq!(stream.slice(), $avalue);
                    },
                    _ => unreachable!(),
                }
            )*
            assert_eq!(s.parse_next().unwrap(), style::Token::EndOfStream);
        }
    )
}

test_attr!(parse_style_1, "fill:none; color:cyan; stroke-width:4.00",
    ("fill", "none"),
    ("color", "cyan"),
    ("stroke-width", "4.00")
);

test_attr!(parse_style_2, "fill:none;",
    ("fill", "none")
);

test_attr!(parse_style_3, "font-size:24px;font-family:'Arial Bold'",
    ("font-size", "24px"),
    ("font-family", "Arial Bold")
);

test_attr!(parse_style_4, "font-size:24px; /* comment */ font-style:normal;",
    ("font-size", "24px"),
    ("font-style", "normal")
);

test_attr!(parse_style_5, "font-size:24px;-font-style:normal;font-stretch:normal;",
    ("font-size", "24px"),
    ("font-stretch", "normal")
);

test_attr!(parse_style_6, "fill:none;-webkit:hi",
    ("fill", "none")
);

test_attr!(parse_style_7, "font-family:&apos;Verdana&apos;",
    ("font-family", "Verdana")
);

test_attr!(parse_style_8, "  fill  :  none  ",
    ("fill", "none")
);

#[test]
fn parse_style_9() {
    let stream = Stream::new("&st0; &st1;");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().unwrap(), style::Token::EntityRef("st0"));
    assert_eq!(s.parse_next().unwrap(), style::Token::EntityRef("st1"));
    assert_eq!(s.parse_next().unwrap(), style::Token::EndOfStream);
}

test_attr!(parse_style_10, "/**/",
);

#[test]
fn invalid_1() {
    let stream = Stream::new(":");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().err().unwrap(), Error::InvalidAttributeValue(ErrorPos::new(1,1)));
}

#[test]
fn invalid_2() {
    let stream = Stream::new("name:'");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().err().unwrap(), Error::InvalidAttributeValue(ErrorPos::new(1,7)));
}

#[test]
fn invalid_3() {
    let stream = Stream::new("&\x0a96M*9");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().err().unwrap(), Error::InvalidAttributeValue(ErrorPos::new(1,2)));
}

#[test]
fn invalid_4() {
    let stream = Stream::new("/*/**/");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().is_err(), true);
}
