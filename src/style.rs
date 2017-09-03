// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing [`<style>`] data.
//!
//! [`<style>`]: https://www.w3.org/TR/SVG/styling.html#StyleAttribute

use std::fmt;
use std::str;

use {Tokenize, Stream, TextFrame, AttributeId, Error};

/// Style token.
#[derive(PartialEq)]
pub enum Token<'a> {
    /// Tuple contains attribute's name and value of an XML element.
    XmlAttribute(&'a str, &'a str),
    /// Tuple contains attribute's ID and value of an SVG element.
    SvgAttribute(AttributeId, TextFrame<'a>),
    /// Tuple contains ENTITY reference. Just a name without `&` and `;`.
    EntityRef(&'a str),
    /// The end of the stream.
    EndOfStream,
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
            Token::EndOfStream =>
                write!(f, "EndOfStream"),
        }
    }
}

/// Tokenizer for \<style\> data.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
}

// TODO: create InvalidStyle instead of InvalidAttributeValue

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    type Token = Token<'a>;

    fn from_str(text: &'a str) -> Tokenizer<'a> {
        Tokenizer { stream: Stream::from_str(text) }
    }

    fn from_frame(frame: TextFrame<'a>) -> Tokenizer<'a> {
        Tokenizer { stream: Stream::from_frame(frame) }
    }

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
    fn parse_next(&mut self) -> Result<Token<'a>, Error> {
        self.stream.skip_spaces();

        if self.stream.at_end() {
            return Ok(Token::EndOfStream);
        }

        let c = self.stream.curr_char_raw();
        if c == b'/' {
            skip_comment(&mut self.stream)?;
            return self.parse_next();
        } else if c == b'-' {
            parse_prefix(&mut self.stream)?;
            return self.parse_next();
        } else if c == b'&' {
            return parse_entity_ref(&mut self.stream);
        } else if is_valid_ident_char(c) {
            return parse_attribute(&mut self.stream);
        } else {
            return Err(Error::InvalidAttributeValue(self.stream.gen_error_pos()));
        }
    }
}

fn skip_comment(stream: &mut Stream) -> Result<(), Error> {
    stream.advance(2)?; // skip /*
    stream.jump_to(b'*')?;
    stream.advance(2)?; // skip */
    stream.skip_spaces();

    Ok(())
}

fn parse_attribute<'a>(stream: &mut Stream<'a>) -> Result<Token<'a>, Error> {
    // consume an attribute name
    let name = {
        let start_pos = stream.pos();
        while !stream.at_end() {
            let c = stream.curr_char_raw();
            if is_valid_ident_char(c) {
                stream.advance_raw(1);
            } else {
                break;
            }
        }

        stream.slice_region_raw(start_pos, stream.pos())
    };

    if name.is_empty() {
        return Err(Error::InvalidAttributeValue(stream.gen_error_pos()));
    }

    stream.skip_spaces();
    stream.consume_char(b':')?;
    stream.skip_spaces();

    let end_char;
    if stream.is_char_eq(b'\'')? {
        // skip start quote
        stream.advance(1)?;
        end_char = b';';
    } else if stream.is_char_eq(b'&')? {
        // skip escaped start quote aka '&apos;'
        if stream.starts_with(b"&apos;") {
            stream.advance_raw(6);
            end_char = b'&';
        } else {
            return Err(Error::InvalidAttributeValue(stream.gen_error_pos()));
        }
    } else {
        end_char = b';';
    }

    let mut value_len = stream.len_to_or_end(end_char);

    if value_len == 0 {
        return Err(Error::InvalidAttributeValue(stream.gen_error_pos()));
    }

    // TODO: stream can be at the end
    // skip end quote
    if stream.char_at(value_len as isize - 1)? == b'\'' {
        value_len -= 1;
    }

    // TODO: use is_space
    while stream.char_at(value_len as isize - 1)? == b' ' {
        value_len -= 1;
    }

    let text_frame = stream.slice_frame_raw(stream.pos(), stream.pos() + value_len);

    stream.advance_raw(value_len);
    stream.skip_spaces();

    if !stream.at_end() {
        if stream.is_char_eq_raw(b'\'') {
            stream.advance_raw(1);
        } else if stream.is_char_eq_raw(b'&') {
            if stream.starts_with(b"&apos;") {
                stream.advance_raw(6);
            } else {
                return Err(Error::InvalidAttributeValue(stream.gen_error_pos()));
            }
        }
    }

    // ';;;' is valid style data, we need to skip it
    while !stream.at_end() && stream.is_char_eq_raw(b';') {
        stream.advance_raw(1);
        stream.skip_spaces();
    }

    if let Some(aid) = AttributeId::from_name(name) {
        return Ok(Token::SvgAttribute(aid, text_frame));
    }

    Ok(Token::XmlAttribute(name, text_frame.slice()))
}

fn parse_entity_ref<'a>(stream: &mut Stream<'a>) -> Result<Token<'a>, Error> {
    // extract 'text' from '&text;'
    stream.advance_raw(1); // &

    let mut len = stream.len_to_space_or_end(); // ;
    if len == 0 {
        return Err(Error::InvalidAttributeValue(stream.gen_error_pos()));
    }
    len -= 1;

    let name = stream.read_raw(len);
    stream.consume_char(b';')?;

    Ok(Token::EntityRef(name))
}

fn parse_prefix<'a>(stream: &mut Stream<'a>) -> Result<(), Error> {
    // prefixed attributes are not supported, aka '-webkit-*'

    stream.advance_raw(1); // -
    let t = parse_attribute(stream)?;

    if let Token::XmlAttribute(name, _) = t {
        warnln!("Style attribute '-{}' is skipped.", name);
    }

    Ok(())
}

fn is_valid_ident_char(c: u8) -> bool {
    match c {
          b'0'...b'9'
        | b'A'...b'Z'
        | b'a'...b'z'
        | b'-'
        | b'_' => true,
        _ => false,
    }
}
