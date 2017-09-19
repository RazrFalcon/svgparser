use std::fmt;
use std::marker::PhantomData;

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

    /// Constructs a new `Tokenizer` from a string.
    fn from_str(text: &'a str) -> Self {
        Self::from_frame(TextFrame::from_str(text))
    }

    /// Constructs a new `Tokenizer` from `TextFrame`.
    fn from_frame(text: TextFrame<'a>) -> Self;

    /// Returns a `Tokens` iterator.
    ///
    /// # Examples
    ///
    /// Minimal:
    ///
    /// ```
    /// use svgparser::{transform, Tokenize};
    ///
    /// let p = transform::Tokenizer::from_str("scale(2 3) invalid").tokens();
    /// let ts: Vec<transform::Token> = p.into_iter().collect();
    /// assert_eq!(ts.len(), 1);
    /// ```
    ///
    /// With an error check:
    ///
    /// ```
    /// use svgparser::{transform, Tokenize, Error, ErrorPos};
    ///
    /// let p = &mut transform::Tokenizer::from_str("scale(2 3) invalid").tokens();
    /// assert_eq!(p.into_iter().count(), 1);
    /// assert_eq!(p.error(), Err(Error::InvalidTransform(ErrorPos::new(1, 12))));
    /// assert_eq!(p.into_iter().count(), 0);
    /// ```
    ///
    /// With a panic on error:
    ///
    /// ```should_panic
    /// use svgparser::{transform, Tokenize};
    ///
    /// let mut p = transform::Tokenizer::from_str("scale(2 3) invalid").tokens();
    /// p.set_allow_panic(true);
    /// let _: Vec<transform::Token> = p.into_iter().collect(); // Will panic.
    /// ```
    fn tokens(self) -> Tokens<'a, Self> {
        Tokens::new(self)
    }

    /// Parses a next token.
    ///
    /// This is a low level way to process an SVG data.
    /// Prefer [`tokens()`] method/iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgparser::{transform, Tokenize, Error};
    ///
    /// let mut p = transform::Tokenizer::from_str("scale(2 3)");
    /// loop {
    ///     match p.parse_next() {
    ///         Ok(t) => println!("{:?}", t),
    ///         Err(Error::EndOfStream) => break, // Tokenizer finished.
    ///         Err(e) => panic!(e),
    ///     }
    /// }
    /// ```
    ///
    /// [`tokens()`]: #method.tokens
    fn parse_next(&mut self) -> Result<Self::Token, Error>;
}

/// An iterator over tokenizer tokens.
///
/// It's a high level way of iterating over tokens.
/// It handles `Error::EndOfStream` processing and stores occurred error
/// for a later use.
///
/// This iterator is *lazy*.
pub struct Tokens<'a, T>
    where T: Tokenize<'a> + 'a
{
    tokenizer: T,
    err: Result<(), Error>,
    is_finished: bool,
    allow_panic: bool,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Tokens<'a, T>
    where T: Tokenize<'a>
{
    fn new(t: T) -> Tokens<'a, T> {
        Tokens {
            tokenizer: t,
            err: Ok(()),
            is_finished: false,
            allow_panic: false,
            marker: PhantomData,
        }
    }

    /// Allows iterator to panic.
    ///
    /// If set to `true`, iterator will panic on error.
    /// Otherwise, occurred error will be stored an will
    /// be available via `error()` method.
    pub fn set_allow_panic(&mut self, flag: bool) {
        self.allow_panic = flag;
    }

    /// Returns an error if there is one.
    pub fn error(&self) -> Result<(), Error> {
        self.err
    }

    /// Returns `true` if an iterator reached it's end.
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}

impl<'a, T> Iterator for Tokens<'a, T>
    where T: Tokenize<'a>
{
    type Item = T::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished || self.err.is_err() {
            return None;
        }

        let token = self.tokenizer.parse_next();
        match token {
            Ok(token) => Some(token),
            Err(Error::EndOfStream) => {
                self.is_finished = true;
                None
            }
            Err(e) => {
                if self.allow_panic {
                    panic!("{:?}", e);
                } else {
                    self.err = Err(e);
                }
                None
            }
        }
    }
}
