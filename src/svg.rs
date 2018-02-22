// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module for parsing SVG structure.

use std::str;

use xmlparser;

use error::{
    Result,
};
use {
    AttributeId,
    ElementId,
    StrSpan,
    FromSpan,
};


/// Name.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Name<'a, T> {
    /// An XML name.
    Xml(&'a str),
    /// An SVG id.
    Svg(T),
}

/// Tag name.
pub type TagName<'a> = Name<'a, ElementId>;
/// Attribute name.
pub type AttrName<'a> = Name<'a, AttributeId>;

impl<'a> From<StrSpan<'a>> for TagName<'a> {
    fn from(v: StrSpan<'a>) -> Self {
        let v = v.to_str();
        match ElementId::from_name(v) {
            Some(id) => Name::Svg(id),
            None => Name::Xml(v),
        }
    }
}

impl<'a> From<StrSpan<'a>> for AttrName<'a> {
    fn from(v: StrSpan<'a>) -> Self {
        let v = v.to_str();
        match AttributeId::from_name(v) {
            Some(id) => Name::Svg(id),
            None => Name::Xml(v),
        }
    }
}


/// SVG token.
#[derive(Debug)]
pub enum Token<'a> {
    /// XML declaration token.
    ///
    /// Example: `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>` ->
    /// `("1.0", "UTF-8", "yes")`
    Declaration(&'a str, Option<&'a str>, Option<&'a str>),
    /// XML processing instruction token.
    ///
    /// Example: `<?target content?>` -> `("target", "content")`
    ProcessingInstruction(&'a str, Option<&'a str>),
    /// The ENTITY token.
    ///
    /// Can appear only inside the DTD.
    /// We emit only entities with [EntityValue](https://www.w3.org/TR/xml/#NT-EntityValue).
    /// [ExternalID](https://www.w3.org/TR/xml/#NT-ExternalID) skipped.
    ///
    /// Example: `<!ENTITY ns_extend "http://test.com">` -> `("ns_extend", "http://test.com")`
    EntityDeclaration(&'a str, StrSpan<'a>),
    /// Comment token.
    ///
    /// Example: `<!-- text -->` -> `" text "`
    Comment(&'a str),
    /// Element start token.
    ///
    /// Example:
    ///
    /// `<nonsvg ...` -> `"nonsvg"`
    ///
    /// `<svg ...` -> `ElementId::SVG`
    ElementStart(TagName<'a>),
    /// Element end token.
    ///
    /// See `ElementEnd` doc for example.
    ElementEnd(ElementEnd<'a>),
    /// Attribute token.
    ///
    /// Example:
    ///
    /// `name="value"` -> `("name", "value")`
    ///
    /// `fill="red"` -> `(AttributeId::Fill, "red")`
    Attribute(AttrName<'a>, StrSpan<'a>),
    /// Text token.
    ///
    /// Contains text between elements including whitespaces.
    /// Basically everything between `>` and `<`.
    ///
    /// Contains text as is. Use `TextUnescape` to unescape it.
    ///
    /// Example: `<text>text</text>` -> `"text"`
    Text(StrSpan<'a>),
    /// CDATA token.
    ///
    /// Example: `<![CDATA[text]]>` -> `"text"`
    Cdata(StrSpan<'a>),
    /// Whitespaces token.
    ///
    /// It will contain only whitespace characters like `\n \t\r`
    /// and escaped version of them, like `&#x20;`.
    ///
    /// If there is a text between elements - `Whitespace` will not be emitted at all.
    ///
    /// Example: `<rect/>\n<rect/>` -> `"\n"`
    Whitespaces(&'a str),
}


/// `ElementEnd` token.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementEnd<'a> {
    /// Indicates `>`
    Open,
    /// Indicates `</name>`
    Close(TagName<'a>),
    /// Indicates `/>`
    Empty,
}


/// Tokenizer for SVG structure.
#[allow(missing_debug_implementations)]
pub struct Tokenizer<'a> {
    parser: xmlparser::Tokenizer<'a>,
}

impl<'a> FromSpan<'a> for Tokenizer<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Tokenizer {
            parser: xmlparser::Tokenizer::from_span(span),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>>;

    /// Extracts next SVG node from the stream.
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - Only ENTITY objects are extracted from DOCTYPE. Library will print a warning to stderr.
    /// - The parser doesn't check an input encoding, assuming that it's UTF-8.
    ///   You should evaluate it by yourself or you will get `Error::Utf8Error`.
    fn next(&mut self) -> Option<Self::Item> {
        let token = match try_opt!(self.parser.next()) {
            Ok(t) => t,
            Err(e) => return Some(Err(e.into()))
        };

        let t = match token {
            xmlparser::Token::ElementStart(name) => {
                Ok(Token::ElementStart(name.into()))
            }
            xmlparser::Token::ElementEnd(end) => {
                let svg_end = match end {
                    xmlparser::ElementEnd::Open => {
                        ElementEnd::Open
                    }
                    xmlparser::ElementEnd::Close(name) => {
                        ElementEnd::Close(name.into())
                    }
                    xmlparser::ElementEnd::Empty => {
                        ElementEnd::Empty
                    }
                };

                Ok(Token::ElementEnd(svg_end))
            }
            xmlparser::Token::Attribute(name, value) => {
                Ok(Token::Attribute(name.into(), value))
            }
            xmlparser::Token::Text(text) => {
                Ok(Token::Text(text))
            }
            xmlparser::Token::Whitespaces(text) => {
                Ok(Token::Whitespaces(text.to_str()))
            }
            xmlparser::Token::Cdata(text) => {
                Ok(Token::Cdata(text))
            }
            xmlparser::Token::Comment(text) => {
                Ok(Token::Comment(text.to_str()))
            }
            xmlparser::Token::EntityDeclaration(name, def) => {
                match def {
                    xmlparser::EntityDefinition::EntityValue(value) => {
                        Ok(Token::EntityDeclaration(name.to_str(), value))
                    }
                    _ => {
                        return self.next();
                    }
                }
            }
            xmlparser::Token::Declaration(version, encoding, standalone) => {
                Ok(Token::Declaration(
                    version.to_str(),
                    encoding.map(|s| s.to_str()),
                    standalone.map(|s| s.to_str())
                ))
            }
            xmlparser::Token::ProcessingInstruction(target, content) => {
                Ok(Token::ProcessingInstruction(
                    target.to_str(),
                    content.map(|s| s.to_str())
                ))
            }
              xmlparser::Token::DtdStart(_, _)
            | xmlparser::Token::EmptyDtd(_, _)
            | xmlparser::Token::DtdEnd => {
                return self.next();
            }
        };

        Some(t)
    }
}
