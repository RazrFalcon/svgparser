#[macro_use]
extern crate bencher;
extern crate svgparser;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

use svgparser::{svg, Tokenize};

fn load_file(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn parse(text: &str) {
    let mut p = svg::Tokenizer::from_str(text);
    loop {
        match p.parse_next() {
            Ok(t) => {
                match t {
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

fn skull(bencher: &mut Bencher) {
    let text = load_file("benches/horns_scary.svg");
    bencher.iter(|| parse(&text))
}

fn jupiter(bencher: &mut Bencher) {
    let text = load_file("benches/Jupiter_diagram.svg");
    bencher.iter(|| parse(&text))
}

fn logo(bencher: &mut Bencher) {
    let text = load_file("benches/SVG_logo.svg");
    bencher.iter(|| parse(&text))
}

benchmark_group!(benches, skull, jupiter, logo);
benchmark_main!(benches);
