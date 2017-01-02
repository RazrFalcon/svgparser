// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::Stream;
use svgparser::style;

macro_rules! test_attr {
    ($name:ident, $text:expr, $(($aname:expr, $avalue:expr)),*) => (
        #[test]
        fn $name() {
            let stream = Stream::new($text);
            let mut s = style::Tokenizer::new(stream);
            $(
                match s.parse_next().unwrap() {
                    style::Token::Attribute(name, stream) => {
                        assert_eq!(name, $aname.as_ref());
                        assert_eq!(stream.slice(), $avalue.as_ref());
                    },
                    _ => unreachable!(),
                }
            )*
            assert_eq!(s.parse_next().unwrap(), style::Token::EndOfStream);
        }
    )
}

test_attr!(parse_style_1, b"fill:none; color:cyan; stroke-width:4.00",
    (b"fill", b"none"),
    (b"color", b"cyan"),
    (b"stroke-width", b"4.00")
);

test_attr!(parse_style_2, b"fill:none;",
    (b"fill", b"none")
);

test_attr!(parse_style_3, b"font-size:24px;font-family:'Arial Bold'",
    (b"font-size", b"24px"),
    (b"font-family", b"Arial Bold")
);

test_attr!(parse_style_4, b"font-size:24px; /* comment */ font-style:normal;",
    (b"font-size", b"24px"),
    (b"font-style", b"normal")
);

test_attr!(parse_style_5, b"font-size:24px;-font-style:normal;font-stretch:normal;",
    (b"font-size", b"24px"),
    (b"font-stretch", b"normal")
);

test_attr!(parse_style_6, b"fill:none;-webkit:hi",
    (b"fill", b"none")
);

test_attr!(parse_style_7, b"font-family:&apos;Verdana&apos;",
    (b"font-family", b"Verdana")
);

#[test]
fn parse_style_8() {
    let stream = Stream::new(b"&st0; &st1;");
    let mut s = style::Tokenizer::new(stream);
    assert_eq!(s.parse_next().unwrap(), style::Token::EntityRef(b"st0"));
    assert_eq!(s.parse_next().unwrap(), style::Token::EntityRef(b"st1"));
    assert_eq!(s.parse_next().unwrap(), style::Token::EndOfStream);
}
