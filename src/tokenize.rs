use std::fmt;

use {TextFrame, Error};

/// A general tokenizer interface.
pub trait Tokenize<'a> {
    /// Token type.
    type Token: fmt::Debug;

    /// Constructs a new `Tokenizer` from string.
    fn from_str(text: &'a str) -> Self;
    /// Constructs a new `Tokenizer` from `TextFrame`.
    fn from_frame(text: TextFrame<'a>) -> Self;
    /// Parses a next token.
    fn parse_next(&mut self) -> Result<Self::Token, Error>;
}
