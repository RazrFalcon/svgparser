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
    let mut tokens = svg::Tokenizer::from_str(text).tokens();

    for token in &mut tokens {
        if let svg::Token::SvgAttribute(AttributeId::D, value) = token {
            paths.push(value.slice().to_owned());
        }
    }

    paths
}

fn parse_paths(paths: &[String]) {
    for path in paths {
        let mut tokens = path::Tokenizer::from_str(path).tokens();
        for _ in &mut tokens {}
    }
}

fn long_paths(bencher: &mut Bencher) {
    let text = load_file("benches/Jupiter_diagram.svg");
    let paths = collect_paths(&text);
    bencher.iter(|| parse_paths(&paths))
}

benchmark_group!(benches, long_paths);
benchmark_main!(benches);
