// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    AttributeId as AId,
    AttributeValue as AV,
    ElementId,
    RgbColor,
    Stream,
    Error,
    ErrorPos,
    ValueId,
};

macro_rules! test {
    ($name:ident, $aid:expr, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            let v = AV::from_stream(ElementId::Rect, $aid, &mut s).unwrap();
            assert_eq!(v, $result);
        }
    )
}

macro_rules! test_err {
    ($name:ident, $aid:expr, $text:expr, $err:expr) => (
        #[test]
        fn $name() {
            let mut s = Stream::new($text);
            let v = AV::from_stream(ElementId::Rect, $aid, &mut s);
            assert_eq!(v.unwrap_err(), $err);
        }
    )
}

test_err!(empty_1, AId::Fill, b"", Error::UnexpectedEndOfStream(ErrorPos::new(1, 1)));
test_err!(empty_2, AId::Fill, b" ", Error::UnexpectedEndOfStream(ErrorPos::new(1, 2)));
// unicode attribute can have spaces
test!(unicode_1, AId::Unicode, b" ", AV::String(b" "));

test!(paint_1, AId::Fill, b"none", AV::PredefValue(ValueId::None));
test!(paint_2, AId::Fill, b"currentColor", AV::PredefValue(ValueId::CurrentColor));
test!(paint_3, AId::Fill, b"inherit", AV::PredefValue(ValueId::Inherit));
test!(paint_4, AId::Fill, b"red", AV::Color(RgbColor::new(255, 0, 0)));
test!(paint_5, AId::Fill, b"url(#link)", AV::FuncIRI(b"link"));
test!(paint_6, AId::Fill, b"url(#link) red", AV::FuncIRI(b"link")); // TODO: must be error
// color is last type that we check during parsing <paint>, so any error will be like that
test_err!(paint_err_1, AId::Fill, b"#link", Error::InvalidColor(ErrorPos::new(1, 1)));

test!(ref_1, AId::Class, b"&ref;", AV::EntityRef(b"ref"));

// TODO: test all supported attributes
