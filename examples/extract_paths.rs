extern crate svgparser;

use std::env;
use std::fs;
use std::str;
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
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // Begin parsing.
    let mut p = svg::Tokenizer::from_str(&text);
    // Get next token.
    loop {
        // Check that it's ok.
        match p.parse_next() {
            Ok(t) => {
                // Filter 'Attribute' token.
                match t {
                    svg::Token::Attribute(name, value) => {
                        // Process only 'd' attributes.
                        if name == "d" {
                            println!("New path:");

                            let mut p = path::Tokenizer::from_frame(value);
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
