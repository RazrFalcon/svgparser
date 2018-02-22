// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<style>`] data.
//!
//! [`<style>`]: https://www.w3.org/TR/SVG/styling.html#StyleAttribute

use std::fmt;
use std::str;

use xmlparser::{
    Reference,
};

use error::{
    Result,
};
use {
    AttributeId,
    ErrorKind,
    FromSpan,
    Stream,
    StrSpan,
};

/// Style token.
#[derive(PartialEq)]
pub enum Token<'a> {
    /// Tuple contains attribute's name and value of an XML element.
    XmlAttribute(&'a str, &'a str),
    /// Tuple contains attribute's ID and value of an SVG element.
    SvgAttribute(AttributeId, StrSpan<'a>),
    /// Tuple contains ENTITY reference. Just a name without `&` and `;`.
    EntityRef(&'a str),
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::XmlAttribute(name, ref value) =>
                write!(f, "XmlAttribute({}, {})", name, value),
            Token::SvgAttribute(id, ref value) =>
                write!(f, "SvgAttribute({:?}, {:?})", id, value),
            Token::EntityRef(name) =>
                write!(f, "EntityRef({})", name),
        }
    }
}

/// Tokenizer for \<style\> data.
#[derive(Clone, Copy, PartialEq)]
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
}

impl<'a> FromSpan<'a> for Tokenizer<'a> {
    fn from_span(span: StrSpan<'a>) -> Self {
        Tokenizer {
            stream: Stream::from_span(span)
        }
    }
}

impl<'a> fmt::Debug for Tokenizer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleTokenizer({:?})", self.stream.span())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>>;

    /// Extracts next style object from the stream.
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    ///
    /// # Notes
    ///
    /// - By SVG spec `style` attribute can contain any style sheet language,
    ///   but we only support CSS2, which is default.
    /// - Objects with `-` prefix will be ignored since we can't write them as XML attributes.
    ///   Library will print a warning to stderr.
    /// - All comments are automatically skipped.
    fn next(&mut self) -> Option<Self::Item> {
        self.stream.skip_spaces();

        if self.stream.at_end() {
            return None;
        }

        macro_rules! try_opt2 {
            ($expr:expr) => {
                match $expr {
                    Ok(value) => value,
                    Err(e) => {
                        self.stream.jump_to_end();
                        return Some(Err(e.into()));
                    }
                }
            }
        }

        let c = try_opt2!(self.stream.curr_byte());
        if c == b'/' {
            try_opt2!(skip_comment(&mut self.stream));
            return self.next();
        } else if c == b'-' {
            try_opt2!(parse_prefix(&mut self.stream));
            return self.next();
        } else if c == b'&' {
            return Some(parse_entity_ref(&mut self.stream));
        } else if is_ident_char(c) {
            return Some(parse_attribute(&mut self.stream));
        } else {
            self.stream.jump_to_end();
            return Some(Err(ErrorKind::InvalidAttributeValue(self.stream.gen_error_pos()).into()));
        }
    }
}

fn skip_comment(stream: &mut Stream) -> Result<()> {
    stream.skip_string(b"/*")?;
    stream.skip_bytes(|_, c| c != b'*');
    stream.skip_string(b"*/")?;
    stream.skip_spaces();

    Ok(())
}

fn parse_attribute<'a>(stream: &mut Stream<'a>) -> Result<Token<'a>> {
    let name = stream.consume_bytes(|_, c| is_ident_char(c));

    if name.is_empty() {
        return Err(ErrorKind::InvalidAttributeValue(stream.gen_error_pos()).into());
    }

    stream.skip_spaces();
    stream.consume_byte(b':')?;
    stream.skip_spaces();

    let value = if stream.curr_byte()? == b'\'' {
        stream.advance(1);
        let v = stream.consume_bytes(|_, c| c != b'\'');
        stream.consume_byte(b'\'')?;
        v
    } else if stream.starts_with(b"&apos;") {
        stream.advance(6);
        let v = stream.consume_bytes(|_, c| c != b'&');
        stream.skip_string(b"&apos;")?;
        v
    } else {
        stream.consume_bytes(|_, c| c != b';')
    }.trim();

    if value.len() == 0 {
        return Err(ErrorKind::InvalidAttributeValue(stream.gen_error_pos()).into());
    }

    stream.skip_spaces();

    // ';;;' is valid style data, we need to skip it
    while stream.is_curr_byte_eq(b';') {
        stream.advance(1);
        stream.skip_spaces();
    }

    if let Some(aid) = AttributeId::from_name(name.to_str()) {
        Ok(Token::SvgAttribute(aid, value))
    } else {
        Ok(Token::XmlAttribute(name.to_str(), value.to_str()))
    }
}

fn parse_entity_ref<'a>(stream: &mut Stream<'a>) -> Result<Token<'a>> {
    if let Ok(Reference::EntityRef(name)) = stream.consume_reference() {
        Ok(Token::EntityRef(name.to_str()))
    } else {
        Err(ErrorKind::InvalidAttributeValue(stream.gen_error_pos()).into())
    }
}

fn parse_prefix<'a>(stream: &mut Stream<'a>) -> Result<()> {
    // prefixed attributes are not supported, aka '-webkit-*'

    stream.advance(1); // -
    let t = parse_attribute(stream)?;

    if let Token::XmlAttribute(name, _) = t {
        warn!("Style attribute '-{}' is skipped.", name);
    }

    Ok(())
}

fn is_ident_char(c: u8) -> bool {
    match c {
          b'0'...b'9'
        | b'A'...b'Z'
        | b'a'...b'z'
        | b'-'
        | b'_' => true,
        _ => false,
    }
}
