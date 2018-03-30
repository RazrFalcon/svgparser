// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use xmlparser::{
    Stream,
    StrSpan,
};

use error::{
    StreamError,
    StreamResult,
};


/// Representation of the `align` value of the [`preserveAspectRatio`] attribute.
///
/// [`preserveAspectRatio`]: https://www.w3.org/TR/SVG/coords.html#PreserveAspectRatioAttribute
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Align {
    None,
    XMinYMin,
    XMidYMin,
    XMaxYMin,
    XMinYMid,
    XMidYMid,
    XMaxYMid,
    XMinYMax,
    XMidYMax,
    XMaxYMax,
}

/// Representation of the [`preserveAspectRatio`] attribute.
///
/// [`preserveAspectRatio`]: https://www.w3.org/TR/SVG/coords.html#PreserveAspectRatioAttribute
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AspectRatio {
    /// `<defer>` value.
    ///
    /// Set to `true` when `defer` value is present.
    pub defer: bool,
    /// `<align>` value.
    pub align: Align,
    /// `<meetOrSlice>` value.
    ///
    /// - Set to `true` when `slice` value is present.
    /// - Set to `false` when `meet` value is present or value is not set at all.
    pub slice: bool,
}

impl AspectRatio {
    /// Parses `AspectRatio` from `StrSpan`.
    pub fn from_span(span: StrSpan) -> StreamResult<Self> {
        let mut s = Stream::from_span(span);

        s.skip_spaces();

        let defer = s.starts_with(b"defer");
        if defer {
            s.advance(5);
            s.consume_byte(b' ')?;
            s.skip_spaces();
        }

        let align = s.consume_name()?.to_str();
        let align = match align {
            "none" => Align::None,
            "xMinYMin" => Align::XMinYMin,
            "xMidYMin" => Align::XMidYMin,
            "xMaxYMin" => Align::XMaxYMin,
            "xMinYMid" => Align::XMinYMid,
            "xMidYMid" => Align::XMidYMid,
            "xMaxYMid" => Align::XMaxYMid,
            "xMinYMax" => Align::XMinYMax,
            "xMidYMax" => Align::XMidYMax,
            "xMaxYMax" => Align::XMaxYMax,
            _ => return {
                Err(StreamError::InvalidAlignType(align.into()))
            }
        };

        s.skip_spaces();

        let mut slice = false;
        if !s.at_end() {
            let v = s.consume_name()?.to_str();
            match v {
                "meet" => {}
                "slice" => slice = true,
                "" => {}
                _ => return {
                    Err(StreamError::InvalidAlignSlice(v.into()))
                }
            };
        }

        Ok(AspectRatio {
            defer,
            align,
            slice,
        })
    }
}

impl FromStr for AspectRatio {
    type Err = StreamError;

    fn from_str(text: &str) -> StreamResult<Self> {
        AspectRatio::from_span(StrSpan::from_str(text))
    }
}
