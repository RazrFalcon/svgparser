#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{Stream, StreamExt, SvgError, SvgErrorKind};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let mut stream = Stream::from_str(s);

        if let Err(e) = stream.parse_length() {
            match e {
                SvgError(SvgErrorKind::InvalidNumber(_), _) => {}
                _ => panic!("{:?}", e),
            }
        }
    }
});
