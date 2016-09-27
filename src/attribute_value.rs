// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::str;

use super::{
    AttributeId,
    ElementId,
    Error,
    Length,
    LengthList,
    NumberList,
    RgbColor,
    Stream,
    ValueId,
};

#[inline]
fn f64_bound(min: f64, val: f64, max: f64) -> f64 {
    if val > max {
        return max;
    } else if val < min {
        return min;
    }

    val
}

/// The paint type fallback value in case the FuncIRI is not resolved.
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum PaintFallback {
    /// Can contain only `none` or `currentColor`.
    PredefValue(ValueId),
    /// The \<color\> type.
    Color(RgbColor),
}

/// Representation of the SVG attribute value.
#[derive(Clone,PartialEq)]
pub enum AttributeValue<'a> {
    /// \<color\> type.
    Color(RgbColor),
    /// Reference to the ENTITY. Contains only `name` from `&name;`.
    EntityRef(&'a [u8]),
    /// \<length\> type.
    Length(Length),
    /// \<list-of-lengths\> type.
    LengthList(LengthList<'a>),
    /// \<IRI\> type.
    IRI(&'a [u8]),
    /// \<FuncIRI\> type.
    FuncIRI(&'a [u8]),
    /// \<FuncIRI\> type.
    FuncIRIWithFallback(&'a [u8], PaintFallback),
    /// \<number\> type.
    Number(f64),
    /// \<list-of-numbers\> type.
    NumberList(NumberList<'a>),
    /// ID of the predefined value.
    PredefValue(ValueId),
    /// Unknown data.
    String(&'a [u8]),
}

impl<'a> fmt::Debug for AttributeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttributeValue::Color(color) => write!(f, "Color({:?})", color),
            AttributeValue::EntityRef(name) => write!(f, "EntityRef({})", u8_to_str!(name)),
            AttributeValue::Length(len) => write!(f, "Length({:?})", len),
            AttributeValue::LengthList(list) =>
                write!(f, "LengthList({})", u8_to_str!(list.0.slice())),
            AttributeValue::IRI(name) => write!(f, "IRI({})", u8_to_str!(name)),
            AttributeValue::FuncIRI(name) => write!(f, "FuncIRI({})", u8_to_str!(name)),
            AttributeValue::FuncIRIWithFallback(name, ref fallback) =>
                write!(f, "FuncIRI({}) Fallback({:?})", u8_to_str!(name), fallback),
            AttributeValue::Number(num) => write!(f, "Number({})", num),
            AttributeValue::NumberList(list) =>
                write!(f, "NumberList({})", u8_to_str!(list.0.slice())),
            AttributeValue::PredefValue(id) => write!(f, "PredefValue({})", id.name()),
            AttributeValue::String(text) => write!(f, "String({})", u8_to_str!(text)),
        }
    }
}

macro_rules! parse_or {
    ($expr1:expr, $expr2:expr) => ({
        match $expr1 {
            Some(v) => Ok(v),
            None => $expr2,
        }
    })
}

// TODO: more attributes
// TODO: test, somehow
impl<'a> AttributeValue<'a> {
    /// Converts stream data into `AttributeValue`.
    ///
    /// This function supports all
    /// [presentation attributes](https://www.w3.org/TR/SVG/propidx.html).
    ///
    /// # Errors
    ///
    /// - Most of the `Error` types can occur.
    /// - Data of an unknown attribute will be parsed as `AttributeValue::String` without errors.
    ///
    /// # Notes
    ///
    /// - `<transform>`, `<path>` and `<style>` values should be parsed using their
    ///   own tokenizer's. This function will parse them as `AttributeValue::String`, aka ignores.
    /// - `enable-background` and `cursor` are not fully implemented.
    ///   This function will try to parse a single predefined value. Other data will be parsed as
    ///   `AttributeValue::String`.
    ///
    ///   Library will print a warning to stdout.
    /// - `viewBox` will be parsed as `AttributeValue::NumberList`.
    /// - `<opacity>` value will be bounded to 0..1 range.
    /// - This function didn't correct most of the numeric values. If value has an incorrect
    ///   data, like `viewBox='0 0 -1 -5'` (negative w/h is error), it will be parsed as is.
    pub fn from_stream(eid: ElementId, aid: AttributeId, mut stream: &mut Stream<'a>)
        -> Result<AttributeValue<'a>, Error>
    {
        use attribute::AttributeId as AId;

        let start_pos = stream.pos();

        macro_rules! parse_predef {
            ($($cmp:pat),+) => (
                match ValueId::from_name(u8_to_str!(stream.slice_tail())) {
                    Some(v) => {
                        match v {
                            $(
                                $cmp => Some(AttributeValue::PredefValue(v)),
                            )+
                            _ => None,
                        }
                    }
                    None => None,
                }
            )
        }

        macro_rules! invalid_attr {
            () => ({
                stream.set_pos_raw(start_pos);
                Err(Error::InvalidAttributeValue(stream.gen_error_pos()))
            })
        }

        macro_rules! parse_or_err {
            ($expr1:expr) => ({
                match $expr1 {
                    Some(v) => Ok(v),
                    None => invalid_attr!(),
                }
            })
        }

        // 'unicode' attribute can contain spaces
        if aid != AId::Unicode {
            stream.skip_spaces();
            stream.trim_trailing_spaces();
        }

        if !stream.at_end() && try!(stream.curr_char()) == b'&' {
            stream.advance_raw(1);
            let len = try!(stream.len_to(b';'));
            // TODO: attribute can contain many refs, not only one.
            // TODO: advance to the end of the stream
            return Ok(AttributeValue::EntityRef(stream.slice_next_raw(len)));
        }

        match aid {
              AId::X  | AId::Y
            | AId::Dx | AId::Dy => {
                // some attributes can contain different data based on the element type
                match eid {
                      ElementId::AltGlyph
                    | ElementId::Text
                    | ElementId::Tref
                    | ElementId::Tspan => {
                        Ok(AttributeValue::LengthList(LengthList(*stream)))
                    },
                    _ => {
                        let l = try!(stream.parse_length());
                        Ok(AttributeValue::Length(l))
                    },
                }
            }

              AId::X1 | AId::Y1
            | AId::X2 | AId::Y2
            | AId::R
            | AId::Rx | AId::Ry
            | AId::Cx | AId::Cy
            | AId::Fx | AId::Fy
            | AId::Offset
            | AId::Width | AId::Height => {
                let l = try!(stream.parse_length());
                Ok(AttributeValue::Length(l))
            }

              AId::StrokeDashoffset
            | AId::StrokeMiterlimit
            | AId::StrokeWidth => {
                parse_or!(parse_predef!(ValueId::Inherit), parse_length(stream))
            }

              AId::Opacity
            | AId::FillOpacity
            | AId::FloodOpacity
            | AId::StrokeOpacity
            | AId::StopOpacity => {
                // TODO: rewrite
                if try!(stream.is_char_eq(b'i')) {
                    Ok(AttributeValue::PredefValue(ValueId::Inherit))
                } else {
                    let mut n = try!(stream.parse_number());
                    n = f64_bound(0.0, n, 1.0);
                    Ok(AttributeValue::Number(n))
                }
            }

            AId::StrokeDasharray => {
                parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::Inherit
                ), Ok(AttributeValue::LengthList(LengthList(*stream))))
            }

            AId::Fill => {
                // 'fill' in animate-based elements it's another 'fill'
                // https://www.w3.org/TR/SVG/animate.html#FillAttribute
                match eid {
                      ElementId::Set
                    | ElementId::Animate
                    | ElementId::AnimateColor
                    | ElementId::AnimateMotion
                    | ElementId::AnimateTransform
                      => Ok(AttributeValue::String(stream.slice())),
                    _ => {
                        parse_or!(parse_predef!(
                            ValueId::None,
                            ValueId::CurrentColor,
                            ValueId::Inherit),
                            parse_or!(parse_paint_func_iri(stream), parse_rgb_color(stream)))
                    }
                }
            }

            AId::Stroke => {
                parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::CurrentColor,
                    ValueId::Inherit),
                    parse_or!(parse_paint_func_iri(stream), parse_rgb_color(stream)))
            },

              AId::ClipPath
            | AId::Filter
            | AId::Marker
            | AId::MarkerEnd
            | AId::MarkerMid
            | AId::MarkerStart
            | AId::Mask => {
                parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::Inherit),
                    parse_or_err!(parse_func_iri(stream)))
            }

            AId::XlinkHref => { parse_iri(stream) }

            AId::Color => {
                parse_or!(parse_predef!(ValueId::Inherit),
                    Ok(AttributeValue::Color(try!(RgbColor::from_stream(stream)))))
            }

              AId::LightingColor
            | AId::FloodColor
            | AId::StopColor => {
                parse_or!(parse_predef!(ValueId::Inherit, ValueId::CurrentColor),
                    Ok(AttributeValue::Color(try!(RgbColor::from_stream(stream)))))
            }

              AId::StdDeviation
            | AId::BaseFrequency => {
                // TODO: this attributes can contain only one or two numbers
                Ok(AttributeValue::NumberList(NumberList(*stream)))
            }

            AId::Points => {
                Ok(AttributeValue::NumberList(NumberList(*stream)))
            }

            AId::AlignmentBaseline => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::Baseline,
                    ValueId::BeforeEdge,
                    ValueId::TextBeforeEdge,
                    ValueId::Middle,
                    ValueId::Central,
                    ValueId::AfterEdge,
                    ValueId::TextAfterEdge,
                    ValueId::Ideographic,
                    ValueId::Alphabetic,
                    ValueId::Hanging,
                    ValueId::Mathematical,
                    ValueId::Inherit
                ))
            }

            AId::Display => {
                parse_or_err!(parse_predef!(
                    ValueId::Inline,
                    ValueId::Block,
                    ValueId::ListItem,
                    ValueId::RunIn,
                    ValueId::Compact,
                    ValueId::Marker,
                    ValueId::Table,
                    ValueId::InlineTable,
                    ValueId::TableRowGroup,
                    ValueId::TableHeaderGroup,
                    ValueId::TableFooterGroup,
                    ValueId::TableRow,
                    ValueId::TableColumnGroup,
                    ValueId::TableColumn,
                    ValueId::TableCell,
                    ValueId::TableCaption,
                    ValueId::None,
                    ValueId::Inherit
                ))
            }

              AId::ClipRule
            | AId::FillRule => {
                parse_or_err!(parse_predef!(
                    ValueId::Nonzero,
                    ValueId::Evenodd,
                    ValueId::Inherit
                ))
            }

              AId::ClipPathUnits
            | AId::GradientUnits => {
                parse_or_err!(parse_predef!(
                    ValueId::UserSpaceOnUse,
                    ValueId::ObjectBoundingBox
                ))
            }

            AId::SpreadMethod => {
                parse_or_err!(parse_predef!(
                    ValueId::Pad,
                    ValueId::Reflect,
                    ValueId::Repeat
                ))
            }

            AId::StrokeLinecap => {
                parse_or_err!(parse_predef!(
                    ValueId::Butt,
                    ValueId::Round,
                    ValueId::Square,
                    ValueId::Inherit
                ))
            }

            AId::Visibility => {
                parse_or_err!(parse_predef!(
                    ValueId::Visible,
                    ValueId::Hidden,
                    ValueId::Collapse,
                    ValueId::Inherit
                ))
            }

              AId::ColorInterpolation
            | AId::ColorInterpolationFilters => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::SRGB,
                    ValueId::LinearRGB,
                    ValueId::Inherit
                ))
            }

            AId::ColorRendering => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::OptimizeSpeed,
                    ValueId::OptimizeQuality,
                    ValueId::Inherit
                ))
            }

            AId::DominantBaseline => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::UseScript,
                    ValueId::NoChange,
                    ValueId::ResetSize,
                    ValueId::Ideographic,
                    ValueId::Alphabetic,
                    ValueId::Hanging,
                    ValueId::Mathematical,
                    ValueId::Central,
                    ValueId::Middle,
                    ValueId::TextAfterEdge,
                    ValueId::TextBeforeEdge,
                    ValueId::Inherit
                ))
            }

            AId::Direction => {
                parse_or_err!(parse_predef!(
                    ValueId::Ltr,
                    ValueId::Rtl,
                    ValueId::Inherit
                ))
            }

            AId::FontStretch => {
                parse_or_err!(parse_predef!(
                    ValueId::Normal,
                    ValueId::Wider,
                    ValueId::Narrower,
                    ValueId::UltraCondensed,
                    ValueId::ExtraCondensed,
                    ValueId::Condensed,
                    ValueId::SemiCondensed,
                    ValueId::SemiExpanded,
                    ValueId::Expanded,
                    ValueId::ExtraExpanded,
                    ValueId::UltraExpanded,
                    ValueId::Inherit
                ))
            }

            AId::FontStyle => {
                parse_or_err!(parse_predef!(
                    ValueId::Normal,
                    ValueId::Italic,
                    ValueId::Oblique,
                    ValueId::Inherit
                ))
            }

            AId::FontVariant => {
                parse_or_err!(parse_predef!(
                    ValueId::Normal,
                    ValueId::SmallCaps,
                    ValueId::Inherit
                ))
            }

            AId::FontWeight => {
                parse_or!(parse_predef!(
                    ValueId::Normal,
                    ValueId::Bold,
                    ValueId::Bolder,
                    ValueId::Lighter,
                    ValueId::Inherit
                ), parse_number(stream))
            }

            AId::BaselineShift => {
                parse_or!(parse_predef!(
                    ValueId::Baseline,
                    ValueId::Sub,
                    ValueId::Super,
                    ValueId::Inherit
                ), parse_length(stream))
            }

            AId::FontSize => {
                parse_or!(parse_predef!(
                    ValueId::XxSmall,
                    ValueId::XSmall,
                    ValueId::Medium,
                    ValueId::Large,
                    ValueId::XLarge,
                    ValueId::XxLarge,
                    ValueId::Larger,
                    ValueId::Smaller,
                    ValueId::Inherit
                ), parse_length(stream))
            }

            AId::FontSizeAdjust => {
                parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::Inherit
                ), parse_number(stream))
            }

            AId::ImageRendering => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::OptimizeSpeed,
                    ValueId::OptimizeQuality,
                    ValueId::Inherit
                ))
            }

            AId::Kerning => {
                parse_or!(parse_predef!(
                    ValueId::Auto,
                    ValueId::Inherit
                ), parse_length(stream))
            }

              AId::WordSpacing
            | AId::LetterSpacing => {
                parse_or!(parse_predef!(
                    ValueId::Normal,
                    ValueId::Inherit
                ), parse_length(stream))
            }

            AId::Overflow => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::Visible,
                    ValueId::Hidden,
                    ValueId::Scroll,
                    ValueId::Inherit
                ))
            }

            AId::PointerEvents => {
                parse_or_err!(parse_predef!(
                    ValueId::VisiblePainted,
                    ValueId::VisibleFill,
                    ValueId::VisibleStroke,
                    ValueId::Visible,
                    ValueId::Painted,
                    ValueId::Fill,
                    ValueId::Stroke,
                    ValueId::All,
                    ValueId::None,
                    ValueId::Inherit
                ))
            }

            AId::ShapeRendering => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::OptimizeSpeed,
                    ValueId::CrispEdges,
                    ValueId::GeometricPrecision,
                    ValueId::Inherit
                ))
            }

            AId::StrokeLinejoin => {
                parse_or_err!(parse_predef!(
                    ValueId::Miter,
                    ValueId::Round,
                    ValueId::Bevel,
                    ValueId::Inherit
                ))
            }

            AId::TextAnchor => {
                parse_or_err!(parse_predef!(
                    ValueId::Start,
                    ValueId::Middle,
                    ValueId::End,
                    ValueId::Inherit
                ))
            }

            AId::TextDecoration => {
                parse_or_err!(parse_predef!(
                    ValueId::None,
                    ValueId::Underline,
                    ValueId::Overline,
                    ValueId::LineThrough,
                    ValueId::Blink,
                    ValueId::Inherit
                ))
            }

            AId::TextRendering => {
                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::OptimizeSpeed,
                    ValueId::OptimizeLegibility,
                    ValueId::GeometricPrecision,
                    ValueId::Inherit
                ))
            }

            AId::UnicodeBidi => {
                parse_or_err!(parse_predef!(
                    ValueId::Normal,
                    ValueId::Embed,
                    ValueId::BidiOverride,
                    ValueId::Inherit
                ))
            }

            AId::WritingMode => {
                parse_or_err!(parse_predef!(
                    ValueId::LrTb,
                    ValueId::RlTb,
                    ValueId::TbRl,
                    ValueId::Lr,
                    ValueId::Rl,
                    ValueId::Tb,
                    ValueId::Inherit
                ))
            }

            AId::ColorProfile => {
                parse_or!(parse_predef!(
                    ValueId::Auto,
                    ValueId::SRGB,
                    ValueId::Inherit
                ), parse_iri(stream))
            }

            AId::GlyphOrientationVertical => {
                parse_or!(parse_predef!(
                    ValueId::Auto,
                    ValueId::Inherit), Ok(AttributeValue::String(stream.slice())))
            }

            AId::Cursor => {
                println!("Warning: The 'cursor' property is not fully supported.");

                parse_or_err!(parse_predef!(
                    ValueId::Auto,
                    ValueId::Crosshair,
                    ValueId::Default,
                    ValueId::Pointer,
                    ValueId::Move,
                    ValueId::EResize,
                    ValueId::NeResize,
                    ValueId::NwResize,
                    ValueId::NResize,
                    ValueId::SeResize,
                    ValueId::SwResize,
                    ValueId::SResize,
                    ValueId::WResize,
                    ValueId::Text,
                    ValueId::Wait,
                    ValueId::Help,
                    ValueId::Inherit
                ))
            }

            AId::EnableBackground => {
                // TODO: parse 'new x y h w'

                parse_or!(parse_predef!(
                    ValueId::Accumulate,
                    ValueId::Inherit
                ), Ok(AttributeValue::String(stream.slice())))
            }

            AId::FontFamily => {
                // TODO: complete parser

                parse_or!(parse_predef!(
                    ValueId::Inherit
                ), Ok(AttributeValue::String(stream.slice())))
            }

            AId::ViewBox => {
                Ok(AttributeValue::NumberList(NumberList(*stream)))
            }

            _ => Ok(AttributeValue::String(stream.slice())),
        }
    }
}

macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None
        }
    }
}

fn parse_paint_func_iri<'a>(stream: &mut Stream<'a>) -> Option<AttributeValue<'a>> {
    if !stream.at_end() && stream.is_char_eq_raw(b'u') {
        try_opt!(stream.advance(5).ok());
        let link_len = try_opt!(stream.len_to(b')').ok());
        let link = stream.read_raw(link_len);

        stream.advance_raw(1); // ')'
        stream.skip_spaces();

        // get fallback
        if !stream.at_end() {
            let fallback = stream.slice_tail();

            let vid = match ValueId::from_name(u8_to_str!(fallback)) {
                Some(v) => {
                    match v {
                          ValueId::None
                        | ValueId::CurrentColor => Some(PaintFallback::PredefValue(v)),
                        _ => None,
                    }
                }
                None => None,
            };

            if vid.is_some() {
                Some(AttributeValue::FuncIRIWithFallback(link, vid.unwrap()))
            } else {
                let color = try_opt!(RgbColor::from_stream(stream).ok());
                Some(AttributeValue::FuncIRIWithFallback(link, PaintFallback::Color(color)))
            }
        } else {
            Some(AttributeValue::FuncIRI(link))
        }
    } else {
        None
    }
}

fn parse_func_iri<'a>(stream: &mut Stream<'a>) -> Option<AttributeValue<'a>> {
    if !stream.at_end() && stream.is_char_eq_raw(b'u') {
        try_opt!(stream.advance(5).ok());
        let link = stream.slice_next_raw(try_opt!(stream.len_to(b')').ok()));
        Some(AttributeValue::FuncIRI(link))
    } else {
        None
    }
}

fn parse_iri<'a>(stream: &mut Stream<'a>) -> Result<AttributeValue<'a>, Error> {
    // empty xlink:href is a valid attribute
    if !stream.at_end() && stream.is_char_eq_raw(b'#') {
        // extract internal link
        try!(stream.advance(1));
        let link = stream.slice_tail();
        Ok(AttributeValue::IRI(link))
    } else {
        Ok(AttributeValue::String(stream.slice()))
    }
}

fn parse_length<'a>(stream: &mut Stream<'a>) -> Result<AttributeValue<'a>, Error> {
    let l = try!(stream.parse_length());
    Ok(AttributeValue::Length(l))
}

fn parse_number<'a>(stream: &mut Stream<'a>) -> Result<AttributeValue<'a>, Error> {
    let l = try!(stream.parse_number());
    Ok(AttributeValue::Number(l))
}

fn parse_rgb_color<'a>(stream: &mut Stream<'a>) -> Result<AttributeValue<'a>, Error> {
    let c = try!(RgbColor::from_stream(stream));
    Ok(AttributeValue::Color(c))
}
