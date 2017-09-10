use std::fmt;

use {
    Error,
    TextFrame,
};

/// A general tokenizer interface.
pub trait Tokenize<'a>
    where Self: Sized
{
    /// Token type.
    type Token: fmt::Debug;

    /// Constructs a new `Tokenizer` from string.
    fn from_str(text: &'a str) -> Self {
        Self::from_frame(TextFrame::from_str(text))
    }

    /// Constructs a new `Tokenizer` from `TextFrame`.
    fn from_frame(text: TextFrame<'a>) -> Self;

    /// Parses a next token.
    fn parse_next(&mut self) -> Result<Self::Token, Error>;
}
