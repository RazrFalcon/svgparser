#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;
use std::str::FromStr;

use svgparser::{Color, Error};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if let Err(e) = Color::from_str(s) {
            match e {
                Error::InvalidColor(_)
                | Error::InvalidNumber(_)
                | Error::InvalidChar { .. }
                | Error::UnexpectedEndOfStream(_) => {}
                _ => panic!("{:?}", e),
            }
        }
    }
});
