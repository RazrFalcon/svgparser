#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate svgparser;

use std::str;

use svgparser::{TextUnescape, XmlSpace};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let _ = TextUnescape::unescape(s, XmlSpace::Default).unwrap();
        let _ = TextUnescape::unescape(s, XmlSpace::Preserve).unwrap();
    }
});
