// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{Error, ErrorPos};
use svgparser::svg;

// TODO: test randomly ended files
// TODO: test parsing random parts of file

macro_rules! basic_assert_eq {
    ($tokenizer:expr, $token:expr) => (
        assert_eq!($tokenizer.parse_next().unwrap(), $token)
    )
}

macro_rules! cdata_assert_eq {
    ($tokenizer:expr, $text:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Cdata(stream) => assert_eq!(stream.slice(), $text.as_ref()),
            _ => unreachable!(),
        }
    )
}

macro_rules! attr_assert_eq {
    ($tokenizer:expr, $name:expr, $value:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Attribute(name, stream) => {
                assert_eq!(name, $name);
                assert_eq!(stream.slice(), $value.as_ref());
            },
            _ => unreachable!(),
        }
    )
}

macro_rules! text_assert_eq {
    ($tokenizer:expr, $text:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Text(stream) => assert_eq!(stream.slice(), $text.as_ref()),
            _ => unreachable!(),
        }
    )
}

macro_rules! entity_assert_eq {
    ($tokenizer:expr, $name:expr, $value:expr) => (
        match $tokenizer.parse_next().unwrap() {
            svg::Token::Entity(name, stream) => {
                assert_eq!(name, $name);
                assert_eq!(stream.slice(), $value.as_ref());
            },
            _ => unreachable!(),
        }
    )
}

#[test]
fn parse_should_fail_1() {
    let mut p = svg::Tokenizer::new(b"");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_should_fail_2() {
    let mut p = svg::Tokenizer::new(b" \t\n \t ");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_declaration_1() {
    let mut p = svg::Tokenizer::new(b"<?xml text?>");
    basic_assert_eq!(p, svg::Token::Declaration(b"text"));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_declaration_2() {
    let mut p = svg::Tokenizer::new(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>");
    basic_assert_eq!(p,
               svg::Token::Declaration(b"version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\""));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_comment_1() {
    let mut p = svg::Tokenizer::new(b"<!-- comment -->");
    basic_assert_eq!(p, svg::Token::Comment(b" comment "));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_comment_2() {
    let mut p = svg::Tokenizer::new(b"<!-- <tag/> -->");
    basic_assert_eq!(p, svg::Token::Comment(b" <tag/> "));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_comment_3() {
    let mut p = svg::Tokenizer::new(b"<!-- qwe1 --><!-- qwe2 -->");
    basic_assert_eq!(p, svg::Token::Comment(b" qwe1 "));
    basic_assert_eq!(p, svg::Token::Comment(b" qwe2 "));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_cdata_1() {
    let mut p = svg::Tokenizer::new(b"<![CDATA[content]]>");
    cdata_assert_eq!(p, b"content");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_cdata_2() {
    let mut p = svg::Tokenizer::new(b"<![CDATA[<message>text</message>]]>");
    cdata_assert_eq!(p, b"<message>text</message>");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_malformed_xml_inside_cdata() {
    let mut p = svg::Tokenizer::new(b"<![CDATA[</this is malformed!</malformed</malformed & worse>]]>");
    cdata_assert_eq!(p, b"</this is malformed!</malformed</malformed & worse>");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_multiply_cdata() {
    let mut p = svg::Tokenizer::new(b"<![CDATA[1]]><![CDATA[2]]>");
    cdata_assert_eq!(p, b"1");
    cdata_assert_eq!(p, b"2");
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_cdata_inside_elem_1() {
    let mut p = svg::Tokenizer::new(b"<style><![CDATA[data]]></style>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"style"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    cdata_assert_eq!(p, b"data");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"style")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_cdata_inside_elem_2() {
    let mut p = svg::Tokenizer::new(b"<style> \t<![CDATA[data]]>\n</style>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"style"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::Whitespace(b" \t"));
    cdata_assert_eq!(p, b"data");
    basic_assert_eq!(p, svg::Token::Whitespace(b"\n"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"style")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_entity_1() {
    let mut p = svg::Tokenizer::new(b"<!DOCTYPE note SYSTEM \"Note.dtd\">");
    basic_assert_eq!(p, svg::Token::DtdEmpty(b"note SYSTEM \"Note.dtd\""));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_entity_2() {
    let mut p = svg::Tokenizer::new(
        b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"
             \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\" [
               <!-- an internal subset can be embedded here -->
          ]>");
    basic_assert_eq!(p,
        svg::Token::DtdStart(b"html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"
             \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\""));
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_entity_3() {
    let mut p = svg::Tokenizer::new(
    b"<!DOCTYPE svg PUBLIC [
        <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
      ]>");

    basic_assert_eq!(p, svg::Token::DtdStart(b"svg PUBLIC"));
    entity_assert_eq!(p, b"ns_extend", b"http://ns.adobe.com/Extensibility/1.0/");
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_entity_4() {
    let mut p = svg::Tokenizer::new(
    b"<!DOCTYPE svg PUBLIC [
        <!ELEMENT sgml ANY>
        <!ENTITY ns_extend \"http://ns.adobe.com/Extensibility/1.0/\">
        <!NOTATION example1SVG-rdf SYSTEM \"example1.svg.rdf\">
        <!ATTLIST img data ENTITY #IMPLIED>
      ]>");

    basic_assert_eq!(p, svg::Token::DtdStart(b"svg PUBLIC"));
    entity_assert_eq!(p, b"ns_extend", b"http://ns.adobe.com/Extensibility/1.0/");
    basic_assert_eq!(p, svg::Token::DtdEnd);
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_elem_1() {
    let mut p = svg::Tokenizer::new(b"<svg/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_elem_2() {
    let mut p = svg::Tokenizer::new(b"<svg></svg>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"svg")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_elem_3() {
    let mut p = svg::Tokenizer::new(b"<svg><rect/></svg>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::ElementStart(b"rect"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"svg")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_1() {
    let mut p = svg::Tokenizer::new(b"<svg version=\"1.0\"/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"version", b"1.0");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_2() {
    let mut p = svg::Tokenizer::new(b"<svg version='1.0'/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"version", b"1.0");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_3() {
    let mut p = svg::Tokenizer::new(b"<svg font=\"'Verdana'\"/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"font", b"'Verdana'");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_4() {
    let mut p = svg::Tokenizer::new(b"<svg version=\"1.0\" color=\"red\"/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"version", b"1.0");
    attr_assert_eq!(p, b"color", b"red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_5() {
    let mut p = svg::Tokenizer::new(b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"xmlns", b"http://www.w3.org/2000/svg");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_6() {
    let mut p = svg::Tokenizer::new(b"<svg version=\"1.0\" color='red'/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"version", b"1.0");
    attr_assert_eq!(p, b"color", b"red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_attributes_7() {
    // I don't know how much correct is this
    let mut p = svg::Tokenizer::new(b"<svg version=\"1.0' color='red\"/>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    attr_assert_eq!(p, b"version", b"1.0' color='red");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_text_1() {
    let mut p = svg::Tokenizer::new(b"<p>text</p>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"p"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, b"text");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"p")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_text_2() {
    let mut p = svg::Tokenizer::new(b"<p> text </p>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"p"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, b" text ");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"p")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_text_3() {
    let mut p = svg::Tokenizer::new(b"<text><tspan>q1<tspan>q2</tspan>q3</tspan></text>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"text"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::ElementStart(b"tspan"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, b"q1");
    basic_assert_eq!(p, svg::Token::ElementStart(b"tspan"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    text_assert_eq!(p, b"q2");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"tspan")));
    text_assert_eq!(p, b"q3");
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"tspan")));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"text")));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn parse_whitespace_1() {
    let mut p = svg::Tokenizer::new(b"<text> </text>");
    basic_assert_eq!(p, svg::Token::ElementStart(b"text"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    basic_assert_eq!(p, svg::Token::Whitespace(b" "));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Close(b"text")));
    assert_eq!(p.next().is_none(), true);
}

// whitespaces outside element are ignored
#[test]
fn parse_whitespace_2() {
    let mut p = svg::Tokenizer::new(b" <text/> ");
    basic_assert_eq!(p, svg::Token::ElementStart(b"text"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Empty));
    assert_eq!(p.next().is_none(), true);
}

#[test]
fn skip_byte_order_1() {
    let mut s = Vec::new();
    s.push(0xEF);
    s.push(0xBB);
    s.push(0xBF);
    let mut p = svg::Tokenizer::new(&s);
    assert_eq!(p.next().is_none(), true);
}

// The idea of this tests is to prove that any error correctly reported as svgparser::error,
// not as Rust bound checking error. Because bound checking can be disabled and it will lead
// to accessing invalid data. Basically a security error.
// Also, predefined error easy to resolve.

#[test]
fn stream_end_on_element_1() {
    let mut p = svg::Tokenizer::new(b"q");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 1)));
}

#[test]
fn stream_end_on_element_2() {
    let mut p = svg::Tokenizer::new(b"text");
    assert_eq!(p.parse_next().err().unwrap(), Error::InvalidSvgToken(ErrorPos::new(1, 1)));
}

#[test]
fn stream_end_on_element_4() {
    let mut p = svg::Tokenizer::new(b"<");
    assert_eq!(p.parse_next().err().unwrap(), Error::ElementWithoutTagName(ErrorPos::new(1, 2)));
}

#[test]
fn stream_end_on_element_5() {
    let mut p = svg::Tokenizer::new(b"<svg><");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    basic_assert_eq!(p, svg::Token::ElementEnd(svg::ElementEnd::Open));
    assert_eq!(p.parse_next().err().unwrap(), Error::ElementWithoutTagName(ErrorPos::new(1, 7)));
}

#[test]
fn stream_end_on_element_6() {
    let mut p = svg::Tokenizer::new(b"< svg");
    assert_eq!(p.parse_next().err().unwrap(), Error::ElementWithoutTagName(ErrorPos::new(1, 2)));
}

#[test]
fn stream_end_on_element_7() {
    let mut p = svg::Tokenizer::new(b"<svg");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 5)));
}

#[test]
fn stream_end_on_attribute_1() {
    let mut p = svg::Tokenizer::new(b"<svg x");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 6)));
}

#[test]
fn stream_end_on_attribute_2() {
    let mut p = svg::Tokenizer::new(b"<svg x=");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 8)));
}

#[test]
fn stream_end_on_attribute_3() {
    let mut p = svg::Tokenizer::new(b"<svg x=\"");
    basic_assert_eq!(p, svg::Token::ElementStart(b"svg"));
    assert_eq!(p.parse_next().err().unwrap(), Error::UnexpectedEndOfStream(ErrorPos::new(1, 9)));
}
