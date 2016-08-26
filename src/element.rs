// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file is autogenerated. Do not edit it!

use std::fmt;

/// List of all SVG elements.
#[derive(Copy,Clone,PartialEq)]
#[allow(missing_docs)]
pub enum ElementId {
    A,
    AltGlyph,
    AltGlyphDef,
    AltGlyphItem,
    Animate,
    AnimateColor,
    AnimateMotion,
    AnimateTransform,
    Circle,
    ClipPath,
    ColorProfile,
    Cursor,
    Defs,
    Desc,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    FlowPara,
    FlowRegion,
    FlowRoot,
    FlowSpan,
    Font,
    FontFace,
    FontFaceFormat,
    FontFaceName,
    FontFaceSrc,
    FontFaceUri,
    ForeignObject,
    G,
    Glyph,
    GlyphRef,
    Hkern,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Metadata,
    MissingGlyph,
    Mpath,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Script,
    Set,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Title,
    Tref,
    Tspan,
    Use,
    View,
    Vkern,
}

static ELEMENTS: ::phf::Map<&'static str, ElementId> = ::phf::Map {
    key: 8958141709656110593,
    disps: ::phf::Slice::Static(&[
        (5, 23),
        (0, 52),
        (0, 2),
        (8, 68),
        (1, 0),
        (0, 3),
        (9, 44),
        (0, 1),
        (1, 23),
        (9, 57),
        (0, 7),
        (6, 28),
        (0, 20),
        (0, 58),
        (47, 63),
        (5, 0),
        (9, 17),
    ]),
    entries: ::phf::Slice::Static(&[
        ("a", ElementId::A),
        ("feComponentTransfer", ElementId::FeComponentTransfer),
        ("title", ElementId::Title),
        ("g", ElementId::G),
        ("desc", ElementId::Desc),
        ("tspan", ElementId::Tspan),
        ("image", ElementId::Image),
        ("font-face", ElementId::FontFace),
        ("script", ElementId::Script),
        ("mask", ElementId::Mask),
        ("clipPath", ElementId::ClipPath),
        ("font-face-format", ElementId::FontFaceFormat),
        ("cursor", ElementId::Cursor),
        ("animateMotion", ElementId::AnimateMotion),
        ("feTurbulence", ElementId::FeTurbulence),
        ("feMorphology", ElementId::FeMorphology),
        ("font-face-uri", ElementId::FontFaceUri),
        ("feDistantLight", ElementId::FeDistantLight),
        ("altGlyph", ElementId::AltGlyph),
        ("animateColor", ElementId::AnimateColor),
        ("svg", ElementId::Svg),
        ("polyline", ElementId::Polyline),
        ("text", ElementId::Text),
        ("altGlyphDef", ElementId::AltGlyphDef),
        ("feOffset", ElementId::FeOffset),
        ("flowSpan", ElementId::FlowSpan),
        ("flowPara", ElementId::FlowPara),
        ("filter", ElementId::Filter),
        ("missing-glyph", ElementId::MissingGlyph),
        ("feFuncB", ElementId::FeFuncB),
        ("fePointLight", ElementId::FePointLight),
        ("feConvolveMatrix", ElementId::FeConvolveMatrix),
        ("flowRegion", ElementId::FlowRegion),
        ("altGlyphItem", ElementId::AltGlyphItem),
        ("foreignObject", ElementId::ForeignObject),
        ("feSpotLight", ElementId::FeSpotLight),
        ("feFlood", ElementId::FeFlood),
        ("feFuncG", ElementId::FeFuncG),
        ("feGaussianBlur", ElementId::FeGaussianBlur),
        ("feMergeNode", ElementId::FeMergeNode),
        ("use", ElementId::Use),
        ("feMerge", ElementId::FeMerge),
        ("pattern", ElementId::Pattern),
        ("tref", ElementId::Tref),
        ("animateTransform", ElementId::AnimateTransform),
        ("textPath", ElementId::TextPath),
        ("feColorMatrix", ElementId::FeColorMatrix),
        ("feImage", ElementId::FeImage),
        ("line", ElementId::Line),
        ("symbol", ElementId::Symbol),
        ("animate", ElementId::Animate),
        ("radialGradient", ElementId::RadialGradient),
        ("vkern", ElementId::Vkern),
        ("feFuncA", ElementId::FeFuncA),
        ("font-face-name", ElementId::FontFaceName),
        ("flowRoot", ElementId::FlowRoot),
        ("defs", ElementId::Defs),
        ("font", ElementId::Font),
        ("feFuncR", ElementId::FeFuncR),
        ("feTile", ElementId::FeTile),
        ("linearGradient", ElementId::LinearGradient),
        ("mpath", ElementId::Mpath),
        ("stop", ElementId::Stop),
        ("feBlend", ElementId::FeBlend),
        ("feDiffuseLighting", ElementId::FeDiffuseLighting),
        ("feDisplacementMap", ElementId::FeDisplacementMap),
        ("path", ElementId::Path),
        ("font-face-src", ElementId::FontFaceSrc),
        ("glyph", ElementId::Glyph),
        ("feSpecularLighting", ElementId::FeSpecularLighting),
        ("glyphRef", ElementId::GlyphRef),
        ("view", ElementId::View),
        ("metadata", ElementId::Metadata),
        ("polygon", ElementId::Polygon),
        ("marker", ElementId::Marker),
        ("feComposite", ElementId::FeComposite),
        ("ellipse", ElementId::Ellipse),
        ("rect", ElementId::Rect),
        ("switch", ElementId::Switch),
        ("hkern", ElementId::Hkern),
        ("set", ElementId::Set),
        ("color-profile", ElementId::ColorProfile),
        ("circle", ElementId::Circle),
        ("style", ElementId::Style),
    ]),
};

impl ElementId {
    /// Converts name into id.
    pub fn from_name(text: &str) -> Option<ElementId> {
        ELEMENTS.get(text).map(|x| *x)
    }

    /// Converts id into name.
    pub fn name(&self) -> &str {
        match *self {
            ElementId::A => "a",
            ElementId::AltGlyph => "altGlyph",
            ElementId::AltGlyphDef => "altGlyphDef",
            ElementId::AltGlyphItem => "altGlyphItem",
            ElementId::Animate => "animate",
            ElementId::AnimateColor => "animateColor",
            ElementId::AnimateMotion => "animateMotion",
            ElementId::AnimateTransform => "animateTransform",
            ElementId::Circle => "circle",
            ElementId::ClipPath => "clipPath",
            ElementId::ColorProfile => "color-profile",
            ElementId::Cursor => "cursor",
            ElementId::Defs => "defs",
            ElementId::Desc => "desc",
            ElementId::Ellipse => "ellipse",
            ElementId::FeBlend => "feBlend",
            ElementId::FeColorMatrix => "feColorMatrix",
            ElementId::FeComponentTransfer => "feComponentTransfer",
            ElementId::FeComposite => "feComposite",
            ElementId::FeConvolveMatrix => "feConvolveMatrix",
            ElementId::FeDiffuseLighting => "feDiffuseLighting",
            ElementId::FeDisplacementMap => "feDisplacementMap",
            ElementId::FeDistantLight => "feDistantLight",
            ElementId::FeFlood => "feFlood",
            ElementId::FeFuncA => "feFuncA",
            ElementId::FeFuncB => "feFuncB",
            ElementId::FeFuncG => "feFuncG",
            ElementId::FeFuncR => "feFuncR",
            ElementId::FeGaussianBlur => "feGaussianBlur",
            ElementId::FeImage => "feImage",
            ElementId::FeMerge => "feMerge",
            ElementId::FeMergeNode => "feMergeNode",
            ElementId::FeMorphology => "feMorphology",
            ElementId::FeOffset => "feOffset",
            ElementId::FePointLight => "fePointLight",
            ElementId::FeSpecularLighting => "feSpecularLighting",
            ElementId::FeSpotLight => "feSpotLight",
            ElementId::FeTile => "feTile",
            ElementId::FeTurbulence => "feTurbulence",
            ElementId::Filter => "filter",
            ElementId::FlowPara => "flowPara",
            ElementId::FlowRegion => "flowRegion",
            ElementId::FlowRoot => "flowRoot",
            ElementId::FlowSpan => "flowSpan",
            ElementId::Font => "font",
            ElementId::FontFace => "font-face",
            ElementId::FontFaceFormat => "font-face-format",
            ElementId::FontFaceName => "font-face-name",
            ElementId::FontFaceSrc => "font-face-src",
            ElementId::FontFaceUri => "font-face-uri",
            ElementId::ForeignObject => "foreignObject",
            ElementId::G => "g",
            ElementId::Glyph => "glyph",
            ElementId::GlyphRef => "glyphRef",
            ElementId::Hkern => "hkern",
            ElementId::Image => "image",
            ElementId::Line => "line",
            ElementId::LinearGradient => "linearGradient",
            ElementId::Marker => "marker",
            ElementId::Mask => "mask",
            ElementId::Metadata => "metadata",
            ElementId::MissingGlyph => "missing-glyph",
            ElementId::Mpath => "mpath",
            ElementId::Path => "path",
            ElementId::Pattern => "pattern",
            ElementId::Polygon => "polygon",
            ElementId::Polyline => "polyline",
            ElementId::RadialGradient => "radialGradient",
            ElementId::Rect => "rect",
            ElementId::Script => "script",
            ElementId::Set => "set",
            ElementId::Stop => "stop",
            ElementId::Style => "style",
            ElementId::Svg => "svg",
            ElementId::Switch => "switch",
            ElementId::Symbol => "symbol",
            ElementId::Text => "text",
            ElementId::TextPath => "textPath",
            ElementId::Title => "title",
            ElementId::Tref => "tref",
            ElementId::Tspan => "tspan",
            ElementId::Use => "use",
            ElementId::View => "view",
            ElementId::Vkern => "vkern",
        }
    }
}

impl fmt::Debug for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
