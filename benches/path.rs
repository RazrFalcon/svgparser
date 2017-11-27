#[macro_use]
extern crate bencher;
extern crate svgparser;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

use svgparser::{svg, path, AttributeId, FromSpan};

fn load_file(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn collect_paths(text: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for token in svg::Tokenizer::from_str(text) {
        if let svg::Token::Attribute(name, value) = token.unwrap() {
            if let svg::Name::Svg(AttributeId::D) = name {
                paths.push(value.to_str().to_owned());
            }
        }
    }

    paths
}

fn parse_paths(paths: &[String]) {
    for path in paths {
        for _ in path::Tokenizer::from_str(path) {}
    }
}

fn long_paths(bencher: &mut Bencher) {
    let text = load_file("benches/Jupiter_diagram.svg");
    let paths = collect_paths(&text);
    bencher.iter(|| parse_paths(&paths))
}

benchmark_group!(benches, long_paths);
benchmark_main!(benches);
