extern crate svgparser;

use std::str::FromStr;

use svgparser::{
    svg,
    path,
    style,
    transform,
    Tokenize,
    Color,
    AttributeId,
    ElementId,
    AttributeValue,
};

// This example shows how to parse different data types from string.

fn main() {
    // Parse types that implements Tokenize trait.
    parse::<svg::Tokenizer>("<svg/>");
    parse::<path::Tokenizer>("M 10 20");
    parse::<style::Tokenizer>("fill:red");
    parse::<transform::Tokenizer>("scale(1 2)");

    // or simply
    let mut t = style::Tokenizer::from_str("fill:red;stroke:blue");
    println!("{:?}", t.parse_next());

    // simple types
    println!("{:?}", Color::from_str("red"));
    println!("{:?}", AttributeValue::from_str(ElementId::Rect, AttributeId::Fill, "red"));
}

fn parse<'a, T: Tokenize<'a>>(text: &'a str) {
    let mut t = T::from_str(text);
    let token = t.parse_next();
    println!("{:?}", token);
}
