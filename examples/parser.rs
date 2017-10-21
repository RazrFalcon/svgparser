extern crate svgparser;

use std::{env, fs, str};
use std::io::Read;

use svgparser::{
    path,
    style,
    svg,
    transform,
    AttributeId,
    AttributeValue,
    ElementId,
    Error,
    TextFrame,
    TextUnescape,
    Tokenize,
    XmlSpace,
};

fn main() {
    // Get a file path from the args.
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: extract_paths img.svg");
        return;
    }

    // Read a file into the buffer.
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // Parse an SVG.
    if let Err(e) = parse(&text) {
        println!("Error: {:?}.", e);
    }
}

// Helper macro for pretty-printing.
macro_rules! print_indent {
    ($msg:expr, $depth:expr) => ({
        write_indent($depth);
        println!($msg);
    });

    ($fmt:expr, $depth:expr, $($arg:tt)*) => ({
        write_indent($depth);
        println!($fmt, $($arg)*);
    });
}

fn write_indent(depth: usize) {
    for _ in 0..(depth * 4) {
        print!(" ");
    }
}

fn parse(text: &str) -> Result<(), Error> {
    // Control XML nodes depth.
    let mut depth = 0;

    // Remember current SVG element, because it will be
    // needed during SVG attributes parsing.
    let mut element_id = None;

    // Begin parsing.

    // Create a tokenizer iterator.
    let mut p = svg::Tokenizer::from_str(text).tokens();

    // Loop through tokens.
    for token in &mut p {
        match token {
            svg::Token::XmlElementStart(tag_name) => {
                // Emits on a non-SVG element.
                //
                // See 'SvgElementStart' for details.

                print_indent!("Non-SVG element: {}", depth, tag_name);
            }
            svg::Token::SvgElementStart(tag_id) => {
                // Emits on an SVG element.
                //
                // Note that this token represent only '<tag' part.
                // Attributes and closing tag will be emitted later.
                //
                // You can check list of all SVG elements here: codegen/spec/elements.txt

                print_indent!("SVG element: {:?}", depth, tag_id);

                element_id = Some(tag_id);
            }
            svg::Token::ElementEnd(end) => {
                parse_element_end(end, &mut depth);
                element_id = None;
            }
            svg::Token::XmlAttribute(name, value) => {
                // Emits on a non-SVG attribute.
                //
                // Note that during non-SVG element parsing all attributes will also be non-SVG.
                //
                // See 'SvgElementStart' for details.

                print_indent!("Non-SVG attribute: {} = '{}'", depth + 1, name, value);
            }
            svg::Token::SvgAttribute(aid, value) => {
                // Emits on an SVG attribute.
                //
                // Will be emitted only during SVG element parsing.
                //
                // You can check list of all SVG elements here: codegen/spec/attributes.txt

                parse_svg_attribute(element_id.unwrap(), aid, value, depth + 1)?;
            }
            svg::Token::Text(text) => {
                // 'text' contain text node content as is.
                // Basically everything between > and <.
                //
                // Token::Whitespace will not be emitted inside Token::Text.
                //
                // Use 'TextUnescape' to convert text entity references,
                // remove unneeded spaces and other.

                print_indent!("Text node: '{}'", depth + 1,
                              TextUnescape::unescape(text.slice(), XmlSpace::Default)?);
            }
            svg::Token::Cdata(cdata) => {
                // CDATA usually used inside the 'style' element and contain CSS,
                // but svgparser doesn't include CSS parser, so you have to use anyone you like.

                print_indent!("CDATA node: '{}'", depth + 1, cdata.slice());
            }
            svg::Token::Whitespace(_) => {
                // We usually don't care about whitespaces.
            }
            svg::Token::Comment(comment) => {
                println!("Comment node: '{}'", comment);
            }
            svg::Token::DtdEmpty(name) => {
                println!("An empty DTD: '{}'", name);
            }
            svg::Token::DtdStart(name) => {
                println!("DTD '{}' parsing started", name);
            }
            svg::Token::Entity(name, value) => {
                // svgparser supports only 'ENTITY'.
                // Any other DTD node will be ignored and a warning will be printed to stderr.

                println!("Entity node: '{}' = '{}'", name, value.slice());
            }
            svg::Token::DtdEnd => {
                println!("DTD parsing ended");
            }
            svg::Token::Declaration(declaration) => {
                // Currently, svgparser doesn't split declaration content,
                // so everything between '<?xml ' and '?>' will be parsed as one string.
                //
                // Any non '<?xml' declarations are not supported.

                println!("Declaration node: {}", declaration);
            }
        }
    }

    p.error()?;

    Ok(())
}

fn parse_svg_attribute(eid: ElementId, aid: AttributeId, value: TextFrame, depth: usize)
    -> Result<(), Error>
{
    // SVG attributes parsing should be done 'manually'.
    // svgparser doesn't parse attributes by default because it can be
    // very expensive (in a case of paths).
    // So you can decide for yourself what to do with attributes.

    match aid {
        AttributeId::D => {
            print_indent!("SVG path:", depth);

            // By the SVG spec, any invalid data occurred in the path should
            // stop parsing of this path, but not the whole document.
            //
            // This function will return `Error::EndOfStream` on any kind of error
            // and will print a warning to stderr.
            // So there is no point in error checking.
            let mut p = path::Tokenizer::from_frame(value).tokens();
            for segment in &mut p {
                print_indent!("{:?}", depth + 1, segment)
            }
        }
        AttributeId::Style => {
            print_indent!("SVG style:", depth);

            let mut p = style::Tokenizer::from_frame(value).tokens();
            for token in &mut p {
                match token {
                    style::Token::XmlAttribute(name, value) => {
                        print_indent!("Non-SVG attribute: {} = '{}'", depth + 1, name, value);
                    }
                    style::Token::SvgAttribute(aid, value) => {
                        parse_svg_attribute(eid, aid, value, depth + 1)?;
                    }
                    style::Token::EntityRef(name) => {
                        print_indent!("Entity reference: {}", depth + 1, name)
                    }
                }
            }

            p.error()?;
        }
          AttributeId::Transform
        | AttributeId::GradientTransform
        | AttributeId::PatternTransform => {
            print_indent!("SVG transform:", depth);

            let mut p = transform::Tokenizer::from_frame(value).tokens();
            for ts in &mut p {
                print_indent!("{:?}", depth + 1, ts)
            }

            p.error()?;
        }
        _ => {
            // We need ElementId for attribute parsing.
            // See 'from_frame' documentation for details.
            let av = AttributeValue::from_frame(eid, aid, value)?;
            print_indent!("SVG attribute: {:?} = {:?}", depth, aid, av);
        }
    }

    // Note that 'class' attribute should be parsed manually if needed.

    Ok(())
}

fn parse_element_end(end: svg::ElementEnd, depth: &mut usize) {
    match end {
        svg::ElementEnd::Open => {
            // <tag> <-- that case
            //   <tag/>
            // </tag>
            print_indent!("Element has children:", *depth + 1);

            *depth += 1;
        }
        svg::ElementEnd::CloseXml(tag_name) => {
            *depth -= 1;

            // <tag>
            //   <tag/>
            // </tag> <-- that case
            print_indent!("Non-SVG element '{}' closed", *depth, tag_name);
        }
        svg::ElementEnd::CloseSvg(tag_id) => {
            *depth -= 1;

            // <tag>
            //   <tag/>
            // </tag> <-- that case
            print_indent!("SVG element '{:?}' closed", *depth, tag_id);
        }
        svg::ElementEnd::Empty => {
            // <tag>
            //   <tag/> <-- that case
            // </tag>
            print_indent!("Element without children closed", *depth);
        }
    }
}
