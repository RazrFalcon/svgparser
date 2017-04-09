// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    AttributeId as AId,
    AttributeValue as AV,
    PaintFallback,
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

test_err!(empty_1, AId::Fill, "", Error::UnexpectedEndOfStream(ErrorPos::new(1, 1)));
test_err!(empty_2, AId::Fill, " ", Error::UnexpectedEndOfStream(ErrorPos::new(1, 2)));
// unicode attribute can have spaces
test!(unicode_1, AId::Unicode, " ", AV::String(" "));

test!(paint_1, AId::Fill, "none", AV::PredefValue(ValueId::None));

test!(paint_2, AId::Fill, "currentColor", AV::PredefValue(ValueId::CurrentColor));

test!(paint_3, AId::Fill, "inherit", AV::PredefValue(ValueId::Inherit));

test!(paint_4, AId::Fill, "red", AV::Color(RgbColor::new(255, 0, 0)));

test!(paint_5, AId::Fill, "url(#link)", AV::FuncIRI("link"));

test!(paint_6, AId::Fill, "url(#link) red",
    AV::FuncIRIWithFallback("link", PaintFallback::Color(RgbColor::new(255, 0, 0))));

// same as above, but for `stroke`
test!(paint_7, AId::Stroke, "url(#link) red",
    AV::FuncIRIWithFallback("link", PaintFallback::Color(RgbColor::new(255, 0, 0))));

test!(paint_8, AId::Fill, "url(#link) none",
    AV::FuncIRIWithFallback("link", PaintFallback::PredefValue(ValueId::None)));

// color is last type that we check during parsing <paint>, so any error will be like that
test_err!(paint_err_1, AId::Fill, "#link", Error::InvalidColor(ErrorPos::new(1, 1)));

test!(ref_1, AId::Class, "&ref;", AV::EntityRef("ref"));

test!(eb_1, AId::EnableBackground, "new    ", AV::String("new"));

// TODO: test all supported attributes, probably via codegen.
