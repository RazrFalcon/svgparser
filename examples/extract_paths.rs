// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use std::env;
use std::fs;
use std::io::Read;

use svgparser::svg;
use svgparser::path;

fn main() {
    // Get a file path from the args.
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: extract_paths img.svg");
        return;
    }

    // Read a file to the buffer.
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut v = Vec::new();
    file.read_to_end(&mut v).unwrap();

    // Begin parsing.
    let mut p = svg::Tokenizer::new(&v);
    // Get next token.
    loop {
        // Check that it's ok.
        match p.parse_next() {
            Ok(t) => {
                // Filter 'Attribute' token.
                match t {
                    svg::Token::Attribute(name, value) => {
                        // Process only 'd' attributes.
                        if name == b"d" {
                            println!("New path:");

                            let mut p = path::Tokenizer::new(value);
                            loop {
                                match p.parse_next() {
                                    Ok(segment_token) => {
                                        match segment_token {
                                            path::SegmentToken::Segment(segment) =>
                                                println!("  {:?}", segment),
                                            path::SegmentToken::EndOfStream =>
                                                break,
                                        }
                                    }
                                    Err(e) => {
                                        // By SVG spec, invalid data occurred in the path should
                                        // stop parsing of this path, but not the whole document.
                                        // So we just show a warning and continue parsing.
                                        println!("Warning: {:?}.", e);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    svg::Token::EndOfStream => break,
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {:?}.", e);
                return;
            }
        }
    }
}
