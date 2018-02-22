// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::str;

use xmlparser::{
    Reference,
};

use error::{
    Result,
};
use {
    AttributeId,
    Color,
    ElementId,
    ErrorKind,
    Length,
    LengthList,
    NumberList,
    Stream,
    StreamExt,
    StrSpan,
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
#[derive(Clone, PartialEq)]
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
    /// [`<viewBox>`].
    ///
    /// [`<viewBox>`]: https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute
    ViewBox(ViewBox),
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

impl<'a> fmt::Debug for AttributeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttributeValue::Color(color) =>
                write!(f, "Color({:?})", color),
            AttributeValue::ViewBox(vb) =>
                write!(f, "ViewBox({:?})", vb),
            AttributeValue::EntityRef(name) =>
                write!(f, "EntityRef({})", name),
            AttributeValue::Length(len) =>
                write!(f, "Length({:?})", len),
            AttributeValue::LengthList(list) =>
                write!(f, "LengthList({})", list.data()),
            AttributeValue::IRI(name) =>
                write!(f, "IRI({})", name),
            AttributeValue::FuncIRI(name) =>
                write!(f, "FuncIRI({})", name),
            AttributeValue::FuncIRIWithFallback(name, ref fallback) =>
                write!(f, "FuncIRI({}) Fallback({:?})", name, fallback),
            AttributeValue::Number(num) =>
                write!(f, "Number({})", num),
            AttributeValue::NumberList(list) =>
                write!(f, "NumberList({})", list.data()),
            AttributeValue::PredefValue(id) =>
                write!(f, "PredefValue({})", id.name()),
            AttributeValue::String(text) =>
                write!(f, "String({})", text),
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
    /// Parses `AttributeValue` from [`StrSpan`].
    ///
    /// This function supports all [presentation attributes].
    ///
    /// # Errors
    ///
    /// - Most of the `Error`'s can occur.
    /// - Data of an unknown attribute will be parsed as `AttributeValue::String` without errors.
    ///
    /// # Notes
    ///
    /// - `transform`, path's `d`, `points` and `style` attributes should be parsed using their
    ///   own tokenizer's. This function will parse them as `AttributeValue::String`, aka ignores.
    /// - `enable-background` and `cursor` are not fully implemented.
    ///   This function will try to parse a single predefined value. Other data will be parsed as
    ///   `AttributeValue::String`.
    ///
    ///   Library will print a warning to stderr.
    /// - `opacity` value will be bounded to 0..1 range.
    /// - This function didn't correct most of the numeric values.
    ///   Like `rect`'s negative size, etc.
    ///
    /// [`StrSpan`]: struct.StrSpan.html
    /// [presentation attributes]: https://www.w3.org/TR/SVG/propidx.html
    pub fn from_span(
        eid: ElementId,
        aid: AttributeId,
        span: StrSpan<'a>,
    ) -> Result<AttributeValue<'a>> {
        use AttributeId as AId;

        // 'unicode' attribute can contain spaces
        let span = if aid != AId::Unicode { span.trim() } else { span };

        let mut stream = Stream::from_span(span);

        let start_pos = stream.pos();

        macro_rules! parse_predef {
            ($($cmp:pat),+) => (
                match ValueId::from_name(stream.slice_tail().to_str()) {
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
                Err(ErrorKind::InvalidAttributeValue(stream.gen_error_pos_from(start_pos)).into())
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

        if stream.is_curr_byte_eq(b'&') {
            // TODO: attribute can contain many refs, not only one
            // TODO: advance to the end of the stream
            let r = stream.consume_reference();
            if let Ok(Reference::EntityRef(name)) = r {
                return Ok(AttributeValue::EntityRef(name.to_str()));
            }
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
                fn get_opacity<'a>(mut s: Stream) -> Result<AttributeValue<'a>> {
                    let mut n = s.parse_number()?;
                    n = f64_bound(0.0, n, 1.0);
                    Ok(AttributeValue::Number(n))
                }

                parse_or!(parse_predef!(
                    ValueId::Inherit
                ), get_opacity(stream))
            }

            AId::StrokeDasharray => {
                parse_or!(parse_predef!(
                    ValueId::None,
                    ValueId::Inherit
                ), Ok(AttributeValue::LengthList(LengthList::from_span(span))))
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
                    parse_or_err!(parse_func_iri(stream)))
            }

            AId::XlinkHref => { parse_iri(stream) }

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
            | AId::FilterUnits
            | AId::GradientUnits
            | AId::MaskContentUnits
            | AId::MaskUnits
            | AId::PatternContentUnits
            | AId::PatternUnits
            | AId::PrimitiveUnits => {
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
                parse_or_err!(parse_predef!(
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
                ))
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
                    ValueId::Inherit), Ok(AttributeValue::String(stream.span().to_str())))
            }

            AId::Cursor => {
                warn!("The 'cursor' property is not fully supported");

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
                ), Ok(AttributeValue::String(stream.span().to_str())))
            }

            AId::FontFamily => {
                // TODO: complete parser

                parse_or!(parse_predef!(
                    ValueId::Inherit
                ), Ok(AttributeValue::String(stream.span().to_str())))
            }

            AId::ViewBox => {
                parse_or_err!(parse_view_box(stream))
            }

            _ => Ok(AttributeValue::String(stream.span().to_str())),
        }
    }

    /// Parses `AttributeValue` from string.
    ///
    /// The same as [`from_frame`].
    ///
    /// [`from_frame`]: #method.from_frame
    pub fn from_str(
        eid: ElementId,
        aid: AttributeId,
        text: &'a str,
    ) -> Result<AttributeValue<'a>>
    {
        AttributeValue::from_span(eid, aid, StrSpan::from_str(text))
    }
}

fn parse_paint_func_iri<'a>(mut stream: Stream<'a>) -> Option<AttributeValue<'a>> {
    if !stream.at_end() && try_opt!(stream.curr_byte().ok()) == b'u' {
        try_opt!(stream.skip_string(b"url(#").ok());
        let link = try_opt!(stream.consume_name().ok()).to_str();

        try_opt!(stream.consume_byte(b')').ok());
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
                Some(AttributeValue::FuncIRIWithFallback(link, v))
            } else {
                let color = try_opt!(Color::from_span(fallback).ok());
                Some(AttributeValue::FuncIRIWithFallback(link, PaintFallback::Color(color)))
            }
        } else {
            Some(AttributeValue::FuncIRI(link))
        }
    } else {
        None
    }
}

fn parse_func_iri<'a>(mut stream: Stream<'a>) -> Option<AttributeValue<'a>> {
    if !stream.at_end() && try_opt!(stream.curr_byte().ok()) == b'u' {
        try_opt!(stream.skip_string(b"url(#").ok());
        let link = try_opt!(stream.consume_name().ok()).to_str();
        try_opt!(stream.consume_byte(b')').ok());

        Some(AttributeValue::FuncIRI(link))
    } else {
        None
    }
}

fn parse_iri<'a>(mut stream: Stream<'a>) -> Result<AttributeValue<'a>> {
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

fn parse_length<'a>(mut stream: Stream<'a>) -> Result<AttributeValue<'a>> {
    let l = stream.parse_length()?;
    Ok(AttributeValue::Length(l))
}

fn parse_number<'a>(mut stream: Stream<'a>) -> Result<AttributeValue<'a>> {
    let l = stream.parse_number()?;
    Ok(AttributeValue::Number(l))
}

fn parse_rgb_color<'a>(stream: Stream<'a>) -> Result<AttributeValue<'a>> {
    let c = Color::from_span(stream.span())?;
    Ok(AttributeValue::Color(c))
}

fn parse_view_box<'a>(mut stream: Stream<'a>) -> Option<AttributeValue<'a>> {
    let x = try_opt!(stream.parse_list_number().ok());
    let y = try_opt!(stream.parse_list_number().ok());
    let w = try_opt!(stream.parse_list_number().ok());
    let h = try_opt!(stream.parse_list_number().ok());

    if w <= 0.0 || h <= 0.0 {
        return None;
    }

    Some(AttributeValue::ViewBox(ViewBox { x, y, w, h }))
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
