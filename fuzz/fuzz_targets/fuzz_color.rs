#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;
use std::str::FromStr;

use svgparser::Color;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Must not panic.
        let _ = Color::from_str(s);
    }
});
