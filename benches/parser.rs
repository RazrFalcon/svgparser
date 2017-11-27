#[macro_use]
extern crate bencher;
extern crate svgparser;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

use svgparser::{svg, FromSpan};

fn load_file(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn parse(text: &str) {
    for token in svg::Tokenizer::from_str(text) {
        let _ = token.unwrap();
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
