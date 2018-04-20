// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    Color,
    Error,
    FromSpan,
    Result,
    Stream,
    StreamExt,
    StrSpan,
};

/// Representation of the fallback part of the [`<paint>`] type.
///
/// [`<paint>`]: https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaintFallback {
    /// The `none` value.
    None,
    /// The `currentColor` value.
    CurrentColor,
    /// [`<color>`] value.
    ///
    /// [`<color>`]: https://www.w3.org/TR/SVG/types.html#DataTypeColor
    Color(Color),
}

/// Representation of the [`<paint>`] type.
///
/// [`<paint>`]: https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Paint<'a> {
    /// The `none` value.
    None,
    /// The `inherit` value.
    Inherit,
    /// The `currentColor` value.
    CurrentColor,
    /// [`<color>`] value.
    ///
    /// [`<color>`]: https://www.w3.org/TR/SVG/types.html#DataTypeColor
    Color(Color),
    /// [`<FuncIRI>`] value.
    ///
    /// [`<FuncIRI>`]: https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI
    FuncIRI(&'a str, Option<PaintFallback>),
}

impl<'a> Paint<'a> {
    /// Parsers a `Paint` from a `&str`.
    ///
    /// We can't use the `FromStr` trait because it requires
    /// an owned value as a return type.
    pub fn from_str(s: &'a str) -> Result<Paint<'a>> {
        Paint::from_span(StrSpan::from(s))
    }

    /// Parsers a `Paint` from a `StrSpan`.
    ///
    /// We can't use the `FromSpan` trait because it requires
    /// an owned value as a return type.
    pub fn from_span(span: StrSpan<'a>) -> Result<Self> {
        let span2 = span.trim();

        match span2.to_str() {
            "none" => Ok(Paint::None),
            "inherit" => Ok(Paint::Inherit),
            "currentColor" => Ok(Paint::CurrentColor),
            _ => {
                let mut s = Stream::from(span2);
                match s.parse_func_iri() {
                    Ok(link) => {
                        s.skip_spaces();

                        // get fallback
                        if !s.at_end() {
                            let fallback = s.slice_tail();
                            match fallback.to_str() {
                                "none" => {
                                    Ok(Paint::FuncIRI(link, Some(PaintFallback::None)))
                                }
                                "currentColor" => {
                                    Ok(Paint::FuncIRI(link, Some(PaintFallback::CurrentColor)))
                                }
                                _ => {
                                    let color = Color::from_span(fallback)?;
                                    Ok(Paint::FuncIRI(link, Some(PaintFallback::Color(color))))
                                }
                            }
                        } else {
                            Ok(Paint::FuncIRI(link, None))
                        }
                    }
                    Err(_) => {
                        match Color::from_span(span2) {
                            Ok(c) => Ok(Paint::Color(c)),
                            Err(_) => Err(Error::InvalidPaint),
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Paint::from_str($text).unwrap(), $result);
            }
        )
    }

    test!(parse_1, "none", Paint::None);
    test!(parse_2, "  none   ", Paint::None);
    test!(parse_3, " inherit ", Paint::Inherit);
    test!(parse_4, " currentColor ", Paint::CurrentColor);
    test!(parse_5, " red ", Paint::Color(Color::new(255, 0, 0)));
    test!(parse_6, " url(#qwe) ", Paint::FuncIRI("qwe", None));
    test!(parse_7, " url(#qwe) none ", Paint::FuncIRI("qwe", Some(PaintFallback::None)));
    test!(parse_8, " url(#qwe) currentColor ", Paint::FuncIRI("qwe", Some(PaintFallback::CurrentColor)));
    test!(parse_9, " url(#qwe) red ", Paint::FuncIRI("qwe", Some(PaintFallback::Color(Color::new(255, 0, 0)))));

    macro_rules! test_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(Paint::from_str($text).unwrap_err().to_string(), $result);
            }
        )
    }

    test_err!(parse_err_1, "qwe", "invalid paint value");
    test_err!(parse_err_2, "red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)", "invalid paint value");
    test_err!(parse_err_3, "url(#qwe) red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)", "invalid color at 1:15");
}
