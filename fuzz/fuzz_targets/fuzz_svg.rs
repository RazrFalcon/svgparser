#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{svg, Tokenize};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        for _ in svg::Tokenizer::from_str(s).tokens() {}
    }
});
