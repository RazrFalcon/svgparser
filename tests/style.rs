extern crate svgparser;

use svgparser::{
    style,
    xmlparser,
    AttributeId as AId,
};

use xmlparser::{
    FromSpan,
};

macro_rules! test_attr {
    ($name:ident, $text:expr, $(($aid:expr, $avalue:expr)),*) => (
        #[test]
        fn $name() {
            let mut s = style::Tokenizer::from_str($text);
            $(
                match s.next().unwrap().unwrap() {
                    style::Token::SvgAttribute(aid, value) => {
                        assert_eq!(aid, $aid);
                        assert_eq!(value.to_str(), $avalue);
                    },
                    _ => unreachable!(),
                }
            )*

            assert_eq!(s.next().is_none(), true);
        }
    )
}

test_attr!(parse_style_1, "fill:none; color:cyan; stroke-width:4.00",
    (AId::Fill, "none"),
    (AId::Color, "cyan"),
    (AId::StrokeWidth, "4.00")
);

test_attr!(parse_style_2, "fill:none;",
    (AId::Fill, "none")
);

test_attr!(parse_style_3, "font-size:24px;font-family:'Arial Bold'",
    (AId::FontSize, "24px"),
    (AId::FontFamily, "Arial Bold")
);

test_attr!(parse_style_4, "font-size:24px; /* comment */ font-style:normal;",
    (AId::FontSize, "24px"),
    (AId::FontStyle, "normal")
);

test_attr!(parse_style_5, "font-size:24px;-font-style:normal;font-stretch:normal;",
    (AId::FontSize, "24px"),
    (AId::FontStretch, "normal")
);

test_attr!(parse_style_6, "fill:none;-webkit:hi",
    (AId::Fill, "none")
);

test_attr!(parse_style_7, "font-family:&apos;Verdana&apos;",
    (AId::FontFamily, "Verdana")
);

test_attr!(parse_style_8, "  fill  :  none  ",
    (AId::Fill, "none")
);

#[test]
fn parse_style_9() {
    let mut s = style::Tokenizer::from_str("&st0; &st1;");
    assert_eq!(s.next().unwrap().unwrap(), style::Token::EntityRef("st0"));
    assert_eq!(s.next().unwrap().unwrap(), style::Token::EntityRef("st1"));
    assert_eq!(s.next().is_none(), true);
}

test_attr!(parse_style_10, "/**/", );

test_attr!(parse_style_11, "font-family:Cantarell;-inkscape-font-specification:&apos;Cantarell Bold&apos;",
    (AId::FontFamily, "Cantarell")
);

// TODO: technically incorrect, because value with spaces should be quoted
test_attr!(parse_style_12, "font-family:Neue Frutiger 65",
    (AId::FontFamily, "Neue Frutiger 65")
);

test_attr!(parse_style_13, "/*text*/fill:green/*text*/",
    (AId::Fill, "green")
);

test_attr!(parse_style_14, "  /*text*/ fill:green  /*text*/ ",
    (AId::Fill, "green")
);

#[test]
fn invalid_1() {
    let mut s = style::Tokenizer::from_str(":");
    assert_eq!(s.next().unwrap().unwrap_err().to_string(),
               "expected '/-&' not ':' at 1:1");
}

#[test]
fn invalid_2() {
    let mut s = style::Tokenizer::from_str("name:'");
    assert_eq!(s.next().unwrap().unwrap_err().to_string(),
               "unexpected end of stream");
}

#[test]
fn invalid_3() {
    let mut s = style::Tokenizer::from_str("&\x0a96M*9");
    assert_eq!(s.next().unwrap().unwrap_err().to_string(),
               "invalid name token");
}

#[test]
fn invalid_4() {
    let mut s = style::Tokenizer::from_str("/*/**/");
    assert_eq!(s.next().unwrap().is_err(), true);
}

#[test]
fn invalid_5() {
    let mut s = style::Tokenizer::from_str("&#x4B2ƿ  ;");
    assert_eq!(s.next().unwrap().unwrap_err().to_string(),
               "expected ';' not 'Æ' at 1:7");
}

#[test]
fn invalid_6() {
    let mut s = style::Tokenizer::from_str("{");
    assert_eq!(s.next().unwrap().unwrap_err().to_string(),
               "expected '/-&' not '{' at 1:1");
}
