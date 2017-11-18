// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use std::str;

use svgparser::{
    ElementId as EId,
    AttributeId as AId,
    Tokenize,
    Error,
    ErrorPos,
};
use svgparser::svg;

macro_rules! basic_assert_eq {
    ($tokenizer:expr, $token:expr) => (
        assert_eq!($tokenizer.parse_next().unwrap(), $token)
    )
}

macro_rules! cdata_assert_eq {
    ($tokenizer:expr, $text:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Cdata(stream) => assert_eq!(stream.slice(), $text),
            _ => unreachable!(),
        }
    )
}

macro_rules! attr_assert_eq {
    ($tokenizer:expr, $name:expr, $value:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::SvgAttribute(aid, av) => {
                assert_eq!(aid, $name);
                assert_eq!(av.slice(), $value);
            },
            _ => unreachable!(),
        }
    )
}

macro_rules! xml_attr_assert_eq {
    ($tokenizer:expr, $name:expr, $value:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::XmlAttribute(name, text) => {
                assert_eq!(name, $name);
                assert_eq!(text, $value);
            },
            _ => unreachable!(),
        }
    )
}

macro_rules! text_assert_eq {
    ($tokenizer:expr, $text:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Text(stream) => assert_eq!(stream.slice(), $text),
            _ => unreachable!(),
        }
    )
}

macro_rules! entity_assert_eq {
    ($tokenizer:expr, $name:expr, $value:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Entity(name, stream) => {
                assert_eq!(name, $name);
                assert_eq!(stream.slice(), $value);
            },
            _ => unreachable!(),
        }
    )
}

#[test]
fn parse_empty_1() {
    let mut p = svg::Tokenizer::from_str("");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_empty_2() {
    let mut p = svg::Tokenizer::from_str(" \t\n \t ");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_declaration_1() {
    let mut p = svg::Tokenizer::from_str("<?xml text?>");
    basic_assert_eq!(p, svg::Token::Declaration("text"));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_declaration_2() {
    let mut p = svg::Tokenizer::from_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>");
    basic_assert_eq!(p,
               svg::Token::Declaration("version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\""));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_pi_1() {
    let mut p = svg::Tokenizer::from_str("<?xpacket begin='﻿' id=''?>");
    basic_assert_eq!(p,
               svg::Token::ProcessingInstruction("xpacket", Some("begin='﻿' id=''")));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_pi_2() {
    let mut p = svg::Tokenizer::from_str("<?xpacket ?>");
    basic_assert_eq!(p,
               svg::Token::ProcessingInstruction("xpacket", None));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_comment_1() {
    let mut p = svg::Tokenizer::from_str("<!-- comment -->");
    basic_assert_eq!(p, svg::Token::Comment(" comment "));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_comment_2() {
    let mut p = svg::Tokenizer::from_str("<!-- <tag/> -->");
    basic_assert_eq!(p, svg::Token::Comment(" <tag/> "));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_comment_3() {
    let mut p = svg::Tokenizer::from_str("<!-- qwe1 --><!-- qwe2 -->");
    basic_assert_eq!(p, svg::Token::Comment(" qwe1 "));
    basic_assert_eq!(p, svg::Token::Comment(" qwe2 "));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_comment_4() {
    let mut p = svg::Tokenizer::from_str("<!---->");
    basic_assert_eq!(p, svg::Token::Comment(""));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_cdata_1() {
    let mut p = svg::Tokenizer::from_str("<![CDATA[content]]>");
    cdata_assert_eq!(p, "content");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_cdata_2() {
    let mut p = svg::Tokenizer::from_str("<![CDATA[<message>text</message>]]>");
    cdata_assert_eq!(p, "<message>text</message>");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_malformed_xml_inside_cdata() {
    let mut p = svg::Tokenizer::from_str("<![CDATA[</this is malformed!</malformed</malformed & worse>]]>");
    cdata_assert_eq!(p, "</this is malformed!</malformed</malformed & worse>");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_multiply_cdata() {
    let mut p = svg::Tokenizer::from_str("<![CDATA[1]]><![CDATA[2]]>");
    cdata_assert_eq!(p, "1");
    cdata_assert_eq!(p, "2");
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_cdata_inside_elem_1() {
    let mut p = svg::Tokenizer::from_str("<style><![CDATA[data]]></style>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Style));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    cdata_assert_eq!(p, "data");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Style)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_cdata_inside_elem_2() {
    let mut p = svg::Tokenizer::from_str("<style> \t<![CDATA[data]]>\n</style>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Style));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::Whitespace(" \t"));
    cdata_assert_eq!(p, "data");
    basic_assert_eq!(p, svg::Token::Whitespace("\n"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Style)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_entity_1() {
    let mut p = svg::Tokenizer::from_str("<!DOCTYPE note SYSTEM \"Note.dtd\">");
    basic_assert_eq!(p, svg::Token::DtdEmpty("note SYSTEM \"Note.dtd\""));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_entity_2() {
    let mut p = svg::Tokenizer::from_str(
        "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"
             \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\" [
               <!-- an internal subset can be embedded here -->
          ]>");
    basic_assert_eq!(p,
        svg::Token::DtdStart("html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"
             \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\""));
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_entity_3() {
    let mut p = svg::Tokenizer::from_str(
    "<!DOCTYPE svg PUBLIC [
        <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
      ]>");

    basic_assert_eq!(p, svg::Token::DtdStart("svg PUBLIC"));
    entity_assert_eq!(p, "ns_extend", "http://ns.adobe.com/Extensibility/1.0/");
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_entity_4() {
    let mut p = svg::Tokenizer::from_str(
    "<!DOCTYPE svg PUBLIC [
        <!ELEMENT sgml ANY>
        <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
        <!NOTATION example1SVG-rdf SYSTEM \"example1.svg.rdf\">
        <!ATTLIST img data ENTITY #IMPLIED>
      ]>");

    basic_assert_eq!(p, svg::Token::DtdStart("svg PUBLIC"));
    entity_assert_eq!(p, "ns_extend", "http://ns.adobe.com/Extensibility/1.0/");
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_elem_1() {
    let mut p = svg::Tokenizer::from_str("<svg/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_elem_2() {
    let mut p = svg::Tokenizer::from_str("<svg></svg>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Svg)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_elem_3() {
    let mut p = svg::Tokenizer::from_str("<svg><rect/></svg>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Rect));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Svg)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_1() {
    let mut p = svg::Tokenizer::from_str("<svg version=\"1.0\"/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_2() {
    let mut p = svg::Tokenizer::from_str("<svg version='1.0'/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_3() {
    let mut p = svg::Tokenizer::from_str("<svg font=\"'Verdana'\"/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Font, "'Verdana'");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_4() {
    let mut p = svg::Tokenizer::from_str("<svg version=\"1.0\" color=\"red\"/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0");
    attr_assert_eq!(p, AId::Color, "red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_5() {
    let mut p = svg::Tokenizer::from_str("<svg xmlns=\"http://www.w3.org/2000/svg\"/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Xmlns, "http://www.w3.org/2000/svg");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_6() {
    let mut p = svg::Tokenizer::from_str("<svg version=\"1.0\" color='red'/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0");
    attr_assert_eq!(p, AId::Color, "red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_7() {
    // I don't know how much correct is this.
    let mut p = svg::Tokenizer::from_str("<svg version=\"1.0' color='red\"/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0' color='red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_8() {
    // '=' can be surrounded by spaces
    let mut p = svg::Tokenizer::from_str("<svg version  =  '1.0'/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Version, "1.0");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_9() {
    let mut p = svg::Tokenizer::from_str("<svg stroke-width='1.0' x1='1.0' xlink:href='#link'/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::StrokeWidth, "1.0");
    attr_assert_eq!(p, AId::X1, "1.0");
    attr_assert_eq!(p, AId::XlinkHref, "#link");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_attributes_10() {
    // mixed attributes
    let mut p = svg::Tokenizer::from_str("<svg color='red' mycolor='red'/>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    attr_assert_eq!(p, AId::Color, "red");
    xml_attr_assert_eq!(p, "mycolor", "red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_text_1() {
    let mut p = svg::Tokenizer::from_str("<p>text</p>");
    basic_assert_eq!(p, svg::Token::XmlElementStart("p"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, "text");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseXml("p")));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_text_2() {
    let mut p = svg::Tokenizer::from_str("<p> text </p>");
    basic_assert_eq!(p, svg::Token::XmlElementStart("p"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, " text ");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseXml("p")));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_text_3() {
    let mut p = svg::Tokenizer::from_str("<text><tspan>q1<tspan>q2</tspan>q3</tspan></text>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Text));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Tspan));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, "q1");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Tspan));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, "q2");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Tspan)));
    text_assert_eq!(p, "q3");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Tspan)));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Text)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn parse_whitespace_1() {
    let mut p = svg::Tokenizer::from_str("<text> </text>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Text));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::Whitespace(" "));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::Text)));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

// whitespaces outside element are ignored
#[test]
fn parse_whitespace_2() {
    let mut p = svg::Tokenizer::from_str(" <text/> ");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Text));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

#[test]
fn skip_byte_order_1() {
    let mut s = Vec::new();
    s.push(0xEF);
    s.push(0xBB);
    s.push(0xBF);

    let t = str::from_utf8(&s).unwrap();

    let mut p = svg::Tokenizer::from_str(t);
    assert_eq!(p.parse_next().unwrap_err(), Error::EndOfStream);
}

// The idea of this tests is to prove that any error correctly reported as svgparser::error,
// not as Rust bound checking error. Because bound checking can be disabled and it will lead
// to accessing invalid data. Basically a security error.
// Also, predefined error easy to resolve.

#[test]
fn stream_end_on_element_1() {
    let mut p = svg::Tokenizer::from_str("q");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 1)));
}

#[test]
fn stream_end_on_element_2() {
    let mut p = svg::Tokenizer::from_str("text");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 1)));
}

#[test]
fn stream_end_on_element_4() {
    let mut p = svg::Tokenizer::from_str("<");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 2)));
}

#[test]
fn stream_end_on_element_5() {
    let mut p = svg::Tokenizer::from_str("<svg><");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 7)));
}

#[test]
fn stream_end_on_element_6() {
    let mut p = svg::Tokenizer::from_str("< svg");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 2)));
}

#[test]
fn stream_end_on_element_7() {
    let mut p = svg::Tokenizer::from_str("<svg");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 5)));
}

#[test]
fn stream_end_on_attribute_1() {
    let mut p = svg::Tokenizer::from_str("<svg x");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 7)));
}

#[test]
fn stream_end_on_attribute_2() {
    let mut p = svg::Tokenizer::from_str("<svg x=");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 8)));
}

#[test]
fn stream_end_on_attribute_3() {
    let mut p = svg::Tokenizer::from_str("<svg x=\"");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 9)));
}

#[test]
fn invalid_structure_1() {
    let mut p = svg::Tokenizer::from_str("<svg><g/><rect/></g></svg>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::G));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Rect));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::G)));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedClosingTag(ErrorPos::new(1, 27)));
}

#[test]
fn invalid_structure_2() {
    let mut p = svg::Tokenizer::from_str("<svg></g></svg>");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::Svg));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::CloseSvg(EId::G)));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedClosingTag(ErrorPos::new(1, 16)));
}

#[test]
fn invalid_structure_3() {
    let mut p = svg::Tokenizer::from_str("\n<<<!W<\x03>");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(2, 2)));
}

#[test]
fn invalid_structure_4() {
    let mut p = svg::Tokenizer::from_str("<?></g");
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 3)));
}

#[test]
fn invalid_structure_5() {
    let mut p = svg::Tokenizer::from_str("<!DOCTYPE [");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 11)));
}

#[test]
fn invalid_structure_6() {
    let mut p = svg::Tokenizer::from_str("<!DOCTYPEE");
    assert_eq!(p.parse_next().err().unwrap(),
               Error::InvalidChar{ current: 'E', expected: ' ', pos: ErrorPos::new(1, 10) });
}

#[test]
fn invalid_structure_7() {
    let mut p = svg::Tokenizer::from_str("<a\r)-=!DO)-<!E");
    basic_assert_eq!(p, svg::Token::SvgElementStart(EId::A));
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 4)));
}

#[test]
fn invalid_structure_8() {
    let mut p = svg::Tokenizer::from_str("<!-->");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 5)));
}
