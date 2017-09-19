// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for parsing SVG structure.

use std::fmt;
use std::str;

use {
    AttributeId,
    ElementId,
    Error,
    Stream,
    TextFrame,
    Tokenize,
};

/// SVG token.
#[derive(PartialEq)]
pub enum Token<'a> {
    /// The token contains tag name of an XML element.
    XmlElementStart(&'a str),
    /// The token contains tag name of an SVG element.
    SvgElementStart(ElementId),
    /// The token contains a type of enclosing tag.
    ElementEnd(ElementEnd<'a>),
    /// The token contains an XML attribute name and value.
    ///
    /// Can appear in XML and SVG elements.
    XmlAttribute(&'a str, &'a str),
    /// The token contains an SVG attribute name and value.
    ///
    /// Can appear only in SVG elements.
    SvgAttribute(AttributeId, TextFrame<'a>),
    /// The token contains text between elements including whitespaces.
    ///
    /// Basically everything between `>` and `<`.
    Text(TextFrame<'a>),
    /// The token contains CDATA data without `<![CDATA[` and `]]>`.
    Cdata(TextFrame<'a>),
    /// The token represents whitespaces between elements.
    ///
    /// It will contain only ` \n\t\r` characters.
    ///
    /// If there is a text between elements - `Whitespace` will not be emitted at all.
    Whitespace(&'a str),
    /// The token contains comment data without `<!--` and `-->`.
    Comment(&'a str),
    /// The token contains a title of empty DOCTYPE.
    DtdEmpty(&'a str),
    /// The token contains a title of DOCTYPE.
    DtdStart(&'a str),
    /// The token contains name and value of ENTITY.
    Entity(&'a str, TextFrame<'a>),
    /// The token indicates DOCTYPE end.
    DtdEnd,
    /// The token contains declaration data without `<?` and `?>`.
    Declaration(&'a str),
}

// TODO: remove
impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::XmlElementStart(s) =>
                write!(f, "XmlElementStart({})", s),
            Token::SvgElementStart(s) =>
                write!(f, "SvgElementStart({:?})", s),
            Token::ElementEnd(ref e) => {
                let c = match *e {
                    ElementEnd::Open => ">",
                    ElementEnd::CloseXml(_) => "</",
                    ElementEnd::CloseSvg(_) => "</",
                    ElementEnd::Empty => "/>",
                };
                write!(f, "ElementEnd({})", c)
            }
            Token::XmlAttribute(k, ref v) =>
                write!(f, "XmlAttribute({}, {:?})", k, v),
            Token::SvgAttribute(k, ref v) =>
                write!(f, "SvgAttribute({:?}, {:?})", k, v),
            Token::Text(ref s) =>
                write!(f, "Text({:?})", s),
            Token::Cdata(ref s) =>
                write!(f, "CDATA({:?})", s),
            Token::Whitespace(s) =>
                write!(f, "Whitespace({})", s),
            Token::Comment(s) =>
                write!(f, "Comment({})", s),
            Token::DtdEmpty(s) =>
                write!(f, "DtdEmpty({})", s),
            Token::DtdStart(s) =>
                write!(f, "DtdStart({})", s),
            Token::Entity(k, ref v) =>
                write!(f, "ENTITY({}, {:?})", k, v),
            Token::DtdEnd =>
                write!(f, "DtdEnd"),
            Token::Declaration(s) =>
                write!(f, "Declaration({})", s),
        }
    }
}

/// `ElementEnd` token.
#[derive(Debug, PartialEq, Clone)]
pub enum ElementEnd<'a> {
    /// Indicates `>`
    Open,
    /// Indicates `</name>` of an XML element
    CloseXml(&'a str),
    /// Indicates `</name>` of an SVG element
    CloseSvg(ElementId),
    /// Indicates `/>`
    Empty,
}

enum State {
    AtStart,
    Unknown,
    Dtd,
    Attributes,
    Finished,
}

/// Tokenizer for SVG structure.
pub struct Tokenizer<'a> {
    stream: Stream<'a>,
    state: State,
    depth: u32,
    curr_elem: Option<ElementId>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    type Token = Token<'a>;

    fn from_frame(text: TextFrame<'a>) -> Tokenizer {
        Tokenizer {
            stream: Stream::from_frame(text),
            state: State::AtStart,
            depth: 0,
            curr_elem: None,
        }
    }

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
    fn parse_next(&mut self) -> Result<Token<'a>, Error> {
        match self.state {
            State::Unknown => {
                if self.stream.at_end() {
                    self.state = State::Finished;
                    return Err(Error::EndOfStream);
                }

                if self.stream.starts_with(b"<?") {
                    self.parse_declaration()
                } else if self.stream.starts_with(b"<!--") {
                    self.parse_comment()
                } else if self.stream.starts_with(b"<![") {
                    self.parse_cdata()
                } else if self.stream.starts_with(b"<!DOCTYPE") {
                    self.parse_dtd()
                } else if self.stream.starts_with(b"</") {
                    self.stream.advance(2)?; // </
                    let tag_name = self.stream.read_to(b'>')?;
                    self.stream.advance(1)?; // >

                    if self.depth == 0 {
                        // Error will occur on the next closing tag after invalid,
                        // because we only checking depth and not a closing tag names.
                        return Err(Error::UnexpectedClosingTag(self.stream.gen_error_pos()));
                    }

                    self.depth -= 1;
                    self.curr_elem = None;

                    let end = match ElementId::from_name(tag_name) {
                        Some(eid) => ElementEnd::CloseSvg(eid),
                        None => ElementEnd::CloseXml(tag_name),
                    };

                    Ok(Token::ElementEnd(end))
                } else if self.stream.is_char_eq_unchecked(b'<') {
                    self.depth += 1;
                    self.parse_element()
                } else if self.depth > 0 {
                    let start = self.stream.pos();
                    self.stream.skip_spaces();

                    if self.stream.is_char_eq(b'<')? {
                        let text = self.stream.slice_region_unchecked(start, self.stream.pos());
                        Ok(Token::Whitespace(text))
                    } else {
                        let b = self.stream.pos() - start;
                        self.stream.back(b)?;
                        let end = self.stream.pos() + self.stream.len_to(b'<')?;
                        let text_frame = self.stream.slice_frame_unchecked(self.stream.pos(), end);
                        self.stream.advance_unchecked(text_frame.len());

                        Ok(Token::Text(text_frame))
                    }
                } else if self.stream.is_space()? {
                    // ignore spaces outside the root element
                    assert_eq!(self.depth, 0);
                    self.stream.skip_spaces();
                    self.parse_next()
                } else {
                    Err(Error::InvalidSvgToken(self.stream.gen_error_pos()))
                }
            }
            State::Dtd => {
                self.parse_entity()
            }
            State::Attributes => {
                self.parse_attribute()
            }
            State::AtStart => {
                if self.stream.at_end() {
                    self.state = State::Finished;
                    return Err(Error::EndOfStream);
                }

                // skip byte order
                if self.stream.is_char_eq(0xEF)? {
                    self.stream.advance(3)?; // EF BB BF
                }

                self.state = State::Unknown;
                self.parse_next()
            }
            State::Finished => {
                Err(Error::EndOfStream)
            }
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn parse_declaration(&mut self) -> Result<Token<'a>, Error> {
        debug_assert!(self.stream.starts_with(b"<?"));

        // we are parsing the Declaration, not the Processing Instruction
        if !self.stream.starts_with(b"<?xml ") {
            return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
        }
        self.stream.advance_unchecked(6); // '<?xml '

        // TODO: parse attributes

        // TODO: ? can be inside the string
        let l = self.stream.len_to(b'?')?;
        let s = self.stream.read_unchecked(l);

        self.stream.consume_char(b'?')?;
        self.stream.consume_char(b'>')?;

        Ok(Token::Declaration(s))
    }

    fn parse_comment(&mut self) -> Result<Token<'a>, Error> {
        self.stream.advance(4)?; // skip <!--
        let start_pos = self.stream.pos();

        // read all until -->
        loop {
            let len = self.stream.len_to(b'>')?;

            // length should be at least 2 to prevent '-' chars overlap
            // like this: '<!-->'
            if len < 2 {
                return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
            }

            self.stream.advance_unchecked(len);
            if self.stream.char_at(-1)? == b'-' && self.stream.char_at(-2)? == b'-' {
                break;
            }
            self.stream.advance(1)?;
        }

        // save data between <!-- and -->
        let end_pos = self.stream.pos() - 2;
        let s = self.stream.slice_region_unchecked(start_pos, end_pos);
        self.stream.advance(1)?;

        Ok(Token::Comment(s))
    }

    fn parse_cdata(&mut self) -> Result<Token<'a>, Error> {
        self.stream.advance(9)?; // skip <![CDATA[
        let start_pos = self.stream.pos();

        loop {
            self.stream.jump_to(b']')?;
            if self.stream.starts_with(b"]]>") {
                break;
            }
            self.stream.advance(1)?;
        }

        // go back to CDATA start to properly init substream.
        let end = self.stream.pos();
        self.stream.set_pos_unchecked(start_pos);

        let text_frame = self.stream.slice_frame_unchecked(self.stream.pos(), end);

        // go to end of CDATA again
        self.stream.set_pos_unchecked(end);
        self.stream.advance(3)?;

        Ok(Token::Cdata(text_frame))
    }

    fn parse_dtd(&mut self) -> Result<Token<'a>, Error> {
        // if first occurred char is '[' - than DTD has content
        // if first occurred char is '>' - than DTD is empty

        debug_assert!(self.stream.starts_with(b"<!DOCTYPE"));

        self.stream.advance_unchecked(9); // '<!DOCTYPE'
        self.stream.consume_char(b' ')?;
        let start = self.stream.pos();

        let l = self.stream.slice_tail()
            .as_bytes()
            .into_iter()
            .position(|x| *x == b'[' || *x == b'>');

        match l {
            Some(l) => self.stream.advance(l)?,
            None => return Err(self.stream.gen_end_of_stream_error()),
        }

        if start == self.stream.pos() {
            return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
        }

        if self.stream.is_char_eq(b'>')? {
            // empty DOCTYPE
            let text = self.stream.slice_region_unchecked(start, self.stream.pos());
            self.stream.advance(1)?;
            Ok(Token::DtdEmpty(text))
        } else {
            // [
            self.state = State::Dtd;

            // skip space at the end
            let text = self.stream.slice_region_unchecked(start, self.stream.pos() - 1);
            self.stream.advance(1)?; // [
            self.stream.skip_spaces();

            Ok(Token::DtdStart(text))
        }
    }

    fn parse_entity(&mut self) -> Result<Token<'a>, Error> {
        if self.stream.starts_with(b"<!ENTITY") {
            self.stream.advance(9)?; // '<!ENTITY '

            let key = self.stream.read_to(b' ')?;

            self.stream.skip_spaces();
            self.stream.consume_char(b'"')?;

            let value_len = self.stream.len_to(b'"')?;

            let text_frame = self.stream.slice_frame_unchecked(
                self.stream.pos(),
                self.stream.pos() + value_len
            );

            self.stream.advance_unchecked(value_len);

            self.stream.consume_char(b'"')?;
            self.stream.skip_spaces();
            self.stream.consume_char(b'>')?;
            self.stream.skip_spaces();

            Ok(Token::Entity(key, text_frame))
        } else if self.stream.starts_with(b"]>") {
            self.stream.advance(2)?; // ]>
            self.state = State::Unknown;

            Ok(Token::DtdEnd)
        } else {
            // skip unsupported elements

            let l = self.stream.len_to(b'>')? + 1;
            warnln!(
                "Unsupported DOCTYPE object: '{}'.",
                self.stream.slice_next_unchecked(l)
            );
            self.stream.advance_unchecked(l);

            self.stream.skip_spaces();
            self.parse_next()
        }
    }

    fn parse_element(&mut self) -> Result<Token<'a>, Error> {
        debug_assert!(self.stream.is_char_eq_unchecked(b'<'));
        self.stream.advance(1)?; // <

        let start_pos = self.stream.pos();

        // consume a tag name
        while !self.stream.at_end() && self.stream.is_ident_unchecked() {
            self.stream.advance(1)?;
        }

        // check that current char is a valid one:
        // '<tagname '
        // '<tagname/'
        // '<tagname>'
        if !self.stream.at_end() {
            if     !self.stream.is_space_unchecked()
                && !self.stream.is_char_eq_unchecked(b'/')
                && !self.stream.is_char_eq_unchecked(b'>')
            {
                return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
            }
        } else {
            // stream can't end here
            return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
        }

        // check that element has tag name
        if start_pos == self.stream.pos() {
            return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
        }

        // TODO: implement read_back(start_pos)
        let tag_name = self.stream.slice_region_unchecked(start_pos, self.stream.pos());
        self.stream.skip_spaces();
        self.state = State::Attributes;

        let token = match ElementId::from_name(tag_name) {
            Some(eid) => {
                self.curr_elem = Some(eid);
                Token::SvgElementStart(eid)
            }
            None => {
                self.curr_elem = None;
                Token::XmlElementStart(tag_name)
            }
        };

        Ok(token)
    }

    fn parse_attribute(&mut self) -> Result<Token<'a>, Error> {
        if self.stream.is_char_eq(b'/')? {
            self.depth -= 1;
            self.stream.advance(2)?;
            self.state = State::Unknown;
            self.curr_elem = None;
            return Ok(Token::ElementEnd(ElementEnd::Empty));
        }

        if self.stream.is_char_eq(b'>')? {
            self.stream.advance_unchecked(1);
            self.state = State::Unknown;
            self.curr_elem = None;
            return Ok(Token::ElementEnd(ElementEnd::Open));
        }

        self.stream.skip_spaces();

        let name = {
            let start = self.stream.pos();
            // consume an attribute name
            while !self.stream.at_end() && self.stream.is_ident_unchecked() {
                self.stream.advance(1)?;
            }

            let len = self.stream.pos() - start;
            if len == 0 {
                return Err(Error::InvalidSvgToken(self.stream.gen_error_pos()));
            }

            self.stream.slice_region_unchecked(start, start + len)
        };

        self.stream.skip_spaces();

        self.stream.consume_char(b'=')?;
        self.stream.skip_spaces();

        if !(self.stream.is_char_eq(b'"')? || self.stream.is_char_eq(b'\'')?) {
            return Err(Error::InvalidChar {
                current: self.stream.curr_char_unchecked() as char,
                expected: '"',
                pos: self.stream.gen_error_pos(),
            });
        }

        let quote = self.stream.curr_char()?;
        self.stream.advance(1)?; // quote

        let end = self.stream.pos() + self.stream.len_to(quote)?;
        let text_frame = self.stream.slice_frame_unchecked(self.stream.pos(), end);

        self.stream.advance_unchecked(text_frame.len());
        self.stream.advance(1)?; // quote

        self.stream.skip_spaces();

        if let Some(_) = self.curr_elem {
            if let Some(aid) = AttributeId::from_name(name) {
                return Ok(Token::SvgAttribute(aid, text_frame));
            }
        }

        Ok(Token::XmlAttribute(name, text_frame.slice()))
    }
}
