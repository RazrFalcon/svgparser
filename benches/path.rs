#[macro_use]
extern crate bencher;
extern crate svgparser;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

use svgparser::{svg, path, AttributeId, Tokenize};

fn load_file(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn collect_paths(text: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut p = svg::Tokenizer::from_str(text);
    loop {
        match p.parse_next() {
            Ok(t) => {
                match t {
                    svg::Token::EndOfStream => break,
                    svg::Token::SvgAttribute(aid, value) => {
                        if aid == AttributeId::D {
                            paths.push(value.slice().to_owned());
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                panic!("Error: {:?}.", e);
            }
        }
    }

    paths
}

fn parse_paths(paths: &[String]) {
    for path in paths {
        let mut p = path::Tokenizer::from_str(path);
        loop {
            match p.parse_next() {
                Ok(token) => {
                    if token == path::Token::EndOfStream {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

fn long_paths(bencher: &mut Bencher) {
    let text = load_file("benches/Jupiter_diagram.svg");
    let paths = collect_paths(&text);
    bencher.iter(|| parse_paths(&paths))
}

benchmark_group!(benches, long_paths);
benchmark_main!(benches);
