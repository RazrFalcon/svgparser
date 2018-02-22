// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    AttributeId as AId,
    AttributeValue as AV,
    ChainedErrorExt,
    Color,
    ElementId,
    PaintFallback,
    ValueId,
    ViewBox,
};

macro_rules! test {
    ($name:ident, $aid:expr, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let v = AV::from_str(ElementId::Rect, $aid, $text).unwrap();
            assert_eq!(v, $result);
        }
    )
}

macro_rules! test_err {
    ($name:ident, $aid:expr, $text:expr, $err:expr) => (
        #[test]
        fn $name() {
            let v = AV::from_str(ElementId::Rect, $aid, $text);
            assert_eq!(v.unwrap_err().full_chain(), $err);
        }
    )
}

test_err!(empty_1, AId::Fill, "", "Error: unexpected end of stream");
test_err!(empty_2, AId::Fill, " ", "Error: unexpected end of stream");
// unicode attribute can have spaces
test!(unicode_1, AId::Unicode, " ", AV::String(" "));

test!(paint_1, AId::Fill, "none", AV::PredefValue(ValueId::None));

test!(paint_2, AId::Fill, "currentColor", AV::PredefValue(ValueId::CurrentColor));

test!(paint_3, AId::Fill, "inherit", AV::PredefValue(ValueId::Inherit));

test!(paint_4, AId::Fill, "red", AV::Color(Color::new(255, 0, 0)));

test!(paint_5, AId::Fill, "url(#link)", AV::FuncIRI("link"));

test!(paint_6, AId::Fill, "url(#link) red",
    AV::FuncIRIWithFallback("link", PaintFallback::Color(Color::new(255, 0, 0))));

// same as above, but for `stroke`
test!(paint_7, AId::Stroke, "url(#link) red",
    AV::FuncIRIWithFallback("link", PaintFallback::Color(Color::new(255, 0, 0))));

test!(paint_8, AId::Fill, "url(#link) none",
    AV::FuncIRIWithFallback("link", PaintFallback::PredefValue(ValueId::None)));

test!(ref_1, AId::Class, "&ref;", AV::EntityRef("ref"));

test!(eb_1, AId::EnableBackground, "new    ", AV::String("new"));

test!(vb_1, AId::ViewBox, "10 20 30 40",
    AV::ViewBox(ViewBox { x: 10.0, y: 20.0, w: 30.0, h: 40.0 }));

test!(vb_2, AId::ViewBox, "10.1 20.2 30.3 40.4",
    AV::ViewBox(ViewBox { x: 10.1, y: 20.2, w: 30.3, h: 40.4 }));

test!(vb_3, AId::ViewBox, "10 20 30 40 invalid data",
    AV::ViewBox(ViewBox { x: 10.0, y: 20.0, w: 30.0, h: 40.0 }));

test!(vb_4, AId::ViewBox, "-10 -20 30 40",
    AV::ViewBox(ViewBox { x: -10.0, y: -20.0, w: 30.0, h: 40.0 }));

// color is last type that we check during parsing <paint>, so any error will be like that
test_err!(paint_err_1, AId::Fill, "#link", "Error: invalid color at 1:1");

test_err!(vb_err_1, AId::ViewBox, "qwe", "Error: invalid attribute value at 1:1");
test_err!(vb_err_2, AId::ViewBox, "10 20 30 0", "Error: invalid attribute value at 1:1");
test_err!(vb_err_3, AId::ViewBox, "10 20 0 40", "Error: invalid attribute value at 1:1");
test_err!(vb_err_4, AId::ViewBox, "10 20 0 0", "Error: invalid attribute value at 1:1");
test_err!(vb_err_5, AId::ViewBox, "10 20 -30 0", "Error: invalid attribute value at 1:1");
test_err!(vb_err_6, AId::ViewBox, "10 20 30 -40", "Error: invalid attribute value at 1:1");
test_err!(vb_err_7, AId::ViewBox, "10 20 -30 -40", "Error: invalid attribute value at 1:1");

// TODO: test all supported attributes, probably via codegen.
