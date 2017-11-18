#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{path, Tokenize};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let mut p = path::Tokenizer::from_str(s).tokens();
        for _ in &mut p {}
        let _ = p.error().unwrap();
    }
});
