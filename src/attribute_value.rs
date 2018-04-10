// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

use xmlparser::{
    FromSpan,
    Reference,
    Stream,
    StrSpan,
};

use error::{
    StreamError,
    StreamResult,
};
use {
    path,
    style,
    transform,
    AspectRatio,
    AttributeId,
    Color,
    ElementId,
    Length,
    LengthList,
    NumberList,
    Points,
    StreamExt,
    ValueId,
};

/// ViewBox representation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewBox {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl ViewBox {
    /// Creates a new `ViewBox`.
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        ViewBox { x, y, w, h }
    }
}


/// The paint type fallback value in case when `FuncIRI` is not resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaintFallback {
    /// Can contain only `none` or `currentColor`.
    PredefValue(ValueId),
    /// [`<color>`] type.
    ///
    /// [`<color>`]: https://www.w3.org/TR/SVG/types.html#DataTypeColor
    Color(Color),
}

/// Representation of the SVG attribute value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeValue<'a> {
    /// [`<number>`] type.
    ///
    /// [`<number>`]: https://www.w3.org/TR/SVG/types.html#DataTypeNumber
    Number(f64),
    /// \<list-of-numbers\> type.
    NumberList(NumberList<'a>),
    /// [`<length>`] type.
    ///
    /// [`<length>`]: https://www.w3.org/TR/SVG/types.html#DataTypeLength
    Length(Length),
    /// \<list-of-lengths\> type.
    LengthList(LengthList<'a>),
    /// [`<color>`] type.
    ///
    /// [`<color>`]: https://www.w3.org/TR/SVG/types.html#DataTypeColor
    Color(Color),
    /// [`<viewBox>`] type.
    ///
    /// [`<viewBox>`]: https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute
    ViewBox(ViewBox),
    /// Representation of the [`preserveAspectRatio`] attribute.
    ///
    /// [`preserveAspectRatio`]: https://www.w3.org/TR/SVG/coords.html#PreserveAspectRatioAttribute
    AspectRatio(AspectRatio),
    /// [`<list-of-points>`] type.
    ///
    /// [`<list-of-points>`]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
    Points(Points<'a>),
    /// [`<path>`] type.
    ///
    /// [`<path>`]: https://www.w3.org/TR/SVG/paths.html#PathData
    Path(path::Tokenizer<'a>),
    /// [`<style>`] type.
    ///
    /// [`<style>`]: https://www.w3.org/TR/SVG/styling.html#StyleAttribute
    Style(style::Tokenizer<'a>),
    /// [`<transform-list>`] type.
    ///
    /// [`<transform-list>`]: https://www.w3.org/TR/SVG/types.html#DataTypeTransformList
    Transform(transform::Tokenizer<'a>),
    /// Reference to the ENTITY. Contains only `name` from `&name;`.
    EntityRef(&'a str),
    /// [`<IRI>`] type.
    ///
    /// [`<IRI>`]: https://www.w3.org/TR/SVG/types.html#DataTypeIRI
    IRI(&'a str),
    /// [`<FuncIRI>`] type.
    ///
    /// [`<FuncIRI>`]: https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI
    FuncIRI(&'a str),
    /// [`<FuncIRI>`] type with a fallback value.
    ///
    /// [`<FuncIRI>`]: https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint
    FuncIRIWithFallback(&'a str, PaintFallback),
    /// ID of the predefined value.
    PredefValue(ValueId),
    /// Unknown data.
    String(&'a str),
}


// TODO: test, somehow
impl<'a> AttributeValue<'a> {
    /// Parses `AttributeValue` from `StrSpan`.
    ///
    /// This function supports all [presentation attributes].
    ///
    /// # Errors
    ///
    /// - Most of the `StreamError`'s can occur.
    /// - Data of an unknown attribute will be parsed as `AttributeValue::String` without errors.
    ///
    /// # Notes
    ///
    /// - `enable-background` is not fully implemented.
    ///   This function will try to parse a single predefined value. Other data will be parsed as
    ///   `AttributeValue::String`.
    /// - `opacity` value will be bounded to 0..1 range.
    /// - This function didn't correct most of the numeric values.
    ///   Like `rect`'s negative size, etc.
    /// - If `prefix` is not empty and `aid` is not `Href`,
    ///   then `AttributeValue::String` will be removed.
    ///
    /// [presentation attributes]: https://www.w3.org/TR/SVG/propidx.html
    pub fn from_span(
        eid: ElementId,
        prefix: &str,
        aid: AttributeId,
        span: StrSpan<'a>,
    ) -> StreamResult<AttributeValue<'a>> {
        parse_av(eid, prefix, aid, span)
    }

    /// Parses `AttributeValue` from string.
    ///
    /// The same as [`from_frame`].
    ///
    /// [`from_frame`]: #method.from_frame
    pub fn from_str(
        eid: ElementId,
        prefix: &str,
        aid: AttributeId,
        text: &'a str,
    ) -> StreamResult<AttributeValue<'a>> {
        AttributeValue::from_span(eid, prefix, aid, StrSpan::from_str(text))
    }
}

macro_rules! parse_or {
    ($expr1:expr, $expr2:expr) => ({
        match $expr1 {
            Ok(v) => Ok(v),
            Err(_) => $expr2,
        }
    })
}

fn parse_av<'a>(
    eid: ElementId,
    prefix: &str,
    aid: AttributeId,
    span: StrSpan<'a>,
) -> StreamResult<AttributeValue<'a>> {
    use AttributeId as AId;

    // 'unicode' attribute can contain spaces
    let span = if aid != AId::Unicode { span.trim() } else { span };

    let mut stream = Stream::from_span(span);

    macro_rules! parse_predef {
        ($($cmp:pat),+) => {{
            let name = stream.slice_tail().to_str();
            match ValueId::from_name(name) {
                Some(v) => {
                    match v {
                        $(
                            $cmp => Ok(AttributeValue::PredefValue(v)),
                        )+
                        _ => Err(StreamError::InvalidPredefValue(name.into())),
                    }
                }
                None => Err(StreamError::InvalidPredefValue(name.into())),
            }
        }}
    }

    if stream.is_curr_byte_eq(b'&') {
        // TODO: attribute can contain many refs, not only one
        // TODO: advance to the end of the stream
        let r = stream.consume_reference();
        if let Ok(Reference::EntityRef(name)) = r {
            return Ok(AttributeValue::EntityRef(name.to_str()));
        }
    }

    if aid == AId::Href && prefix == "xlink" {
        return parse_iri(stream);
    }

    if !prefix.is_empty() {
        return Ok(AttributeValue::String(stream.span().to_str()));
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
                    Ok(AttributeValue::LengthList(LengthList::from_span(span)))
                }
                _ => {
                    let l = stream.parse_length()?;
                    Ok(AttributeValue::Length(l))
                }
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
            let l = stream.parse_length()?;
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
            fn get_opacity<'a>(mut s: Stream) -> StreamResult<AttributeValue<'a>> {
                let mut n = s.parse_number()?;
                n = f64_bound(0.0, n, 1.0);
                Ok(AttributeValue::Number(n))
            }

            parse_or!(parse_predef!(ValueId::Inherit), get_opacity(stream))
        }

        AId::StrokeDasharray => {
            parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::Inherit),
                Ok(AttributeValue::LengthList(LengthList::from_span(span))))
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
                => Ok(AttributeValue::String(stream.span().to_str())),
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
        }

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
                parse_func_iri(stream))
        }

        AId::Color => {
            parse_or!(parse_predef!(ValueId::Inherit),
                      parse_rgb_color(stream))
        }

          AId::LightingColor
        | AId::FloodColor
        | AId::StopColor => {
            parse_or!(parse_predef!(ValueId::Inherit, ValueId::CurrentColor),
                      parse_rgb_color(stream))
        }

          AId::StdDeviation
        | AId::BaseFrequency => {
            // TODO: this attributes can contain only one or two numbers
            Ok(AttributeValue::NumberList(NumberList::from_span(span)))
        }

        AId::Points => {
            Ok(AttributeValue::Points(Points::from_span(span)))
        }

        AId::D => {
            Ok(AttributeValue::Path(path::Tokenizer::from_span(span)))
        }

        AId::Style => {
            Ok(AttributeValue::Style(style::Tokenizer::from_span(span)))
        }

          AId::Transform
        | AttributeId::GradientTransform
        | AttributeId::PatternTransform => {
            Ok(AttributeValue::Transform(transform::Tokenizer::from_span(span)))
        }

        AId::AlignmentBaseline => {
            parse_predef!(
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
            )
        }

        AId::Display => {
            parse_predef!(
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
            )
        }

          AId::ClipRule
        | AId::FillRule => {
            parse_predef!(
                ValueId::Nonzero,
                ValueId::Evenodd,
                ValueId::Inherit
            )
        }

          AId::ClipPathUnits
        | AId::FilterUnits
        | AId::GradientUnits
        | AId::MaskContentUnits
        | AId::MaskUnits
        | AId::PatternContentUnits
        | AId::PatternUnits
        | AId::PrimitiveUnits => {
            parse_predef!(
                ValueId::UserSpaceOnUse,
                ValueId::ObjectBoundingBox
            )
        }

        AId::SpreadMethod => {
            parse_predef!(
                ValueId::Pad,
                ValueId::Reflect,
                ValueId::Repeat
            )
        }

        AId::StrokeLinecap => {
            parse_predef!(
                ValueId::Butt,
                ValueId::Round,
                ValueId::Square,
                ValueId::Inherit
            )
        }

        AId::Visibility => {
            parse_predef!(
                ValueId::Visible,
                ValueId::Hidden,
                ValueId::Collapse,
                ValueId::Inherit
            )
        }

          AId::ColorInterpolation
        | AId::ColorInterpolationFilters => {
            parse_predef!(
                ValueId::Auto,
                ValueId::SRGB,
                ValueId::LinearRGB,
                ValueId::Inherit
            )
        }

        AId::ColorRendering => {
            parse_predef!(
                ValueId::Auto,
                ValueId::OptimizeSpeed,
                ValueId::OptimizeQuality,
                ValueId::Inherit
            )
        }

        AId::DominantBaseline => {
            parse_predef!(
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
            )
        }

        AId::Direction => {
            parse_predef!(
                ValueId::Ltr,
                ValueId::Rtl,
                ValueId::Inherit
            )
        }

        AId::FontStretch => {
            parse_predef!(
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
            )
        }

        AId::FontStyle => {
            parse_predef!(
                ValueId::Normal,
                ValueId::Italic,
                ValueId::Oblique,
                ValueId::Inherit
            )
        }

        AId::FontVariant => {
            parse_predef!(
                ValueId::Normal,
                ValueId::SmallCaps,
                ValueId::Inherit
            )
        }

        AId::FontWeight => {
            parse_predef!(
                ValueId::Normal,
                ValueId::Bold,
                ValueId::Bolder,
                ValueId::Lighter,
                ValueId::N100,
                ValueId::N200,
                ValueId::N300,
                ValueId::N400,
                ValueId::N500,
                ValueId::N600,
                ValueId::N700,
                ValueId::N800,
                ValueId::N900,
                ValueId::Inherit
            )
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
                ValueId::Small,
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
            parse_predef!(
                ValueId::Auto,
                ValueId::OptimizeSpeed,
                ValueId::OptimizeQuality,
                ValueId::Inherit
            )
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
            parse_predef!(
                ValueId::Auto,
                ValueId::Visible,
                ValueId::Hidden,
                ValueId::Scroll,
                ValueId::Inherit
            )
        }

        AId::PointerEvents => {
            parse_predef!(
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
            )
        }

        AId::ShapeRendering => {
            parse_predef!(
                ValueId::Auto,
                ValueId::OptimizeSpeed,
                ValueId::CrispEdges,
                ValueId::GeometricPrecision,
                ValueId::Inherit
            )
        }

        AId::StrokeLinejoin => {
            parse_predef!(
                ValueId::Miter,
                ValueId::Round,
                ValueId::Bevel,
                ValueId::Inherit
            )
        }

        AId::TextAnchor => {
            parse_predef!(
                ValueId::Start,
                ValueId::Middle,
                ValueId::End,
                ValueId::Inherit
            )
        }

        AId::TextDecoration => {
            parse_predef!(
                ValueId::None,
                ValueId::Underline,
                ValueId::Overline,
                ValueId::LineThrough,
                ValueId::Blink,
                ValueId::Inherit
            )
        }

        AId::TextRendering => {
            parse_predef!(
                ValueId::Auto,
                ValueId::OptimizeSpeed,
                ValueId::OptimizeLegibility,
                ValueId::GeometricPrecision,
                ValueId::Inherit
            )
        }

        AId::UnicodeBidi => {
            parse_predef!(
                ValueId::Normal,
                ValueId::Embed,
                ValueId::BidiOverride,
                ValueId::Inherit
            )
        }

        AId::WritingMode => {
            parse_predef!(
                ValueId::LrTb,
                ValueId::RlTb,
                ValueId::TbRl,
                ValueId::Lr,
                ValueId::Rl,
                ValueId::Tb,
                ValueId::Inherit
            )
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
                ValueId::Inherit), Ok(AttributeValue::String(stream.span().to_str())))
        }

        AId::EnableBackground => {
            // TODO: parse 'new x y h w'

            parse_or!(parse_predef!(
                ValueId::Accumulate,
                ValueId::Inherit
            ), Ok(AttributeValue::String(stream.span().to_str())))
        }

        AId::FontFamily => {
            // TODO: complete parser

            parse_or!(parse_predef!(
                ValueId::Inherit
            ), Ok(AttributeValue::String(stream.span().to_str())))
        }

        AId::ViewBox => {
            parse_view_box(stream)
        }

        AId::PreserveAspectRatio => {
            parse_aspect_ratio(stream)
        }

        _ => Ok(AttributeValue::String(stream.span().to_str())),
    }
}

fn parse_paint_func_iri<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    if !stream.at_end() && stream.curr_byte()? == b'u' {
        stream.skip_string(b"url(#")?;
        let link = stream.consume_name()?.to_str();

        stream.consume_byte(b')')?;
        stream.skip_spaces();

        // get fallback
        if !stream.at_end() {
            let fallback = stream.slice_tail();

            let vid = match ValueId::from_name(fallback.to_str()) {
                Some(v) => {
                    match v {
                          ValueId::None
                        | ValueId::CurrentColor => Some(PaintFallback::PredefValue(v)),
                        _ => None,
                    }
                }
                None => None,
            };

            if let Some(v) = vid {
                Ok(AttributeValue::FuncIRIWithFallback(link, v))
            } else {
                let color = Color::from_span(fallback)?;
                Ok(AttributeValue::FuncIRIWithFallback(link, PaintFallback::Color(color)))
            }
        } else {
            Ok(AttributeValue::FuncIRI(link))
        }
    } else {
        Err(StreamError::NotAFuncIRI(stream.span().to_str().into()))
    }
}

fn parse_func_iri<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    if !stream.at_end() && stream.curr_byte()? == b'u' {
        stream.skip_string(b"url(#")?;
        let link = stream.consume_name()?.to_str();
        stream.consume_byte(b')')?;

        Ok(AttributeValue::FuncIRI(link))
    } else {
        Err(StreamError::NotAFuncIRI(stream.span().to_str().into()))
    }
}

fn parse_iri<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    // empty xlink:href is a valid attribute
    if !stream.at_end() && stream.curr_byte()? == b'#' {
        // extract internal link
        stream.advance(1);
        let link = stream.slice_tail();
        Ok(AttributeValue::IRI(link.to_str()))
    } else {
        Ok(AttributeValue::String(stream.span().to_str()))
    }
}

fn parse_length<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    let l = stream.parse_length()?;
    Ok(AttributeValue::Length(l))
}

fn parse_number<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    let l = stream.parse_number()?;
    Ok(AttributeValue::Number(l))
}

fn parse_rgb_color<'a>(stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    let c = Color::from_span(stream.span())?;
    Ok(AttributeValue::Color(c))
}

fn parse_aspect_ratio<'a>(stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    let r = AspectRatio::from_span(stream.span())?;
    Ok(AttributeValue::AspectRatio(r))
}

fn parse_view_box<'a>(mut stream: Stream<'a>) -> StreamResult<AttributeValue<'a>> {
    let x = stream.parse_list_number()?;
    let y = stream.parse_list_number()?;
    let w = stream.parse_list_number()?;
    let h = stream.parse_list_number()?;

    if w <= 0.0 || h <= 0.0 {
        return Err(StreamError::InvalidViewbox);
    }

    Ok(AttributeValue::ViewBox(ViewBox::new(x, y, w, h)))
}

#[inline]
fn f64_bound(min: f64, val: f64, max: f64) -> f64 {
    if val > max {
        return max;
    } else if val < min {
        return min;
    }

    val
}
