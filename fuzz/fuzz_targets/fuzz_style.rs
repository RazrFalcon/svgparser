#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{style, FromSpan};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Must not panic.
        let mut n = 0;
        for _ in style::Tokenizer::from_str(s) {
            n += 1;

            if n == 1000 {
                panic!("endless loop");
            }
        }
    }
});
