#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{style, Tokenize, Error};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let mut p = style::Tokenizer::from_str(s);
        loop {
            match p.parse_next() {
                Ok(token) => {
                    if token == style::Token::EndOfStream {
                        break;
                    }
                }
                Err(e) => {
                    match e {
                        Error::UnexpectedEndOfStream(_)
                        | Error::InvalidAttributeValue(_) => {}
                        _ => panic!("{:?}", e),
                    }
                }
            }
        }
    }
});
