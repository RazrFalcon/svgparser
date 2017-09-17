#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{transform, Tokenize, Error};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let mut p = transform::Tokenizer::from_str(s);
        loop {
            match p.parse_next() {
                Ok(token) => {
                    if token == transform::Token::EndOfStream {
                        break;
                    }
                }
                Err(e) => {
                    match e {
                        Error::UnexpectedEndOfStream(_)
                        | Error::InvalidTransform(_) => {}
                        _ => panic!("{:?}", e),
                    }
                }
            }
        }
    }
});
