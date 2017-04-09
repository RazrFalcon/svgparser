#[macro_use]
extern crate bencher;
extern crate svgparser;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

use svgparser::svg;

fn skull(bencher: &mut Bencher) {
    let path = env::current_dir().unwrap().join("benches/horns_scary.svg");
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    bencher.iter(||{
        let mut p = svg::Tokenizer::new(&text);
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
    })
}

fn jupiter(bencher: &mut Bencher) {
    let path = env::current_dir().unwrap().join("benches/Jupiter_diagram.svg");
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    bencher.iter(||{
        let mut p = svg::Tokenizer::new(&text);
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
    })
}

fn logo(bencher: &mut Bencher) {
    let path = env::current_dir().unwrap().join("benches/SVG_logo.svg");
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    bencher.iter(||{
        let mut p = svg::Tokenizer::new(&text);
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
    })
}

benchmark_group!(benches, skull, jupiter, logo);
benchmark_main!(benches);
