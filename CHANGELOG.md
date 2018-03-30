# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Changed
- **Breaking**. Use `failure` instead of `error-chain`.
- **Breaking**. A completely new error types.
- Minimum Rust version is 1.18.

## [0.7.0] - 2018-03-11
### Added
- **Breaking**. Added `AttributeValue::Path`.
- **Breaking**. Added `AttributeValue::Points`.
- **Breaking**. Added `AttributeValue::Style`.
- **Breaking**. Added `AttributeValue::Transform`.
- **Breaking**. Added `AttributeValue::ViewBox`.
- **Breaking**. Added `AttributeValue::AspectRatio`.
- `points` attribute tokenizer: `Points`.
- All tokenizers derive `Clone`, `Copy`, `PartialEq` and `Debug` now.
- `Debug` for `NumberList` and `LengthList`.
- Case-insensitive parsing for `rgb(...)` colors.
- `preserveAspectRatio` attribute parsing.

### Changed
- **Breaking**. `AttributeValue` parser no longer return the `points` attribute
  as a `NumberList`, but as `Points`.
- **Breaking**. `viewBox` attribute will be parsed as `AttributeValue::ViewBox`
  and not as `AttributeValue::NumberList`.
- **Breaking**. `NumberList` and `LengthList` implements `FromSpan` trait
  instead of custom `from_span` methods.
- **Breaking**. `TagName` and `AttrName` alias `QName` and not `Name` now.
  So tag and attribute names contain namespace prefix now.
  But not the namespace URI itself.
- **Breaking**. Prefixed attributes ID's removed. So:
  
  `AttributeId::XlinkHref` -> `xlink` prefix + `AttributeId::Href`.
- **Breaking**. The `cursor` attribute will be parsed as a string now.
- Relicense from MPL-2.0 to MIT/Apache-2.0.

### Fixed
- `style` attribute parsing when value contains comments.

## [0.6.4] - 2018-02-03
### Fixed
- Invalid files in the crate package.

## [0.6.3] - 2018-01-17
### Changed
- `log` 0.3 -> 0.4

## [0.6.2] - 2017-12-16
### Fixed
- Discard 0.6.1 changes.

## [0.6.1] - 2017-12-16
### Fixed
- Path parsing when subpath starts not with MoveTo.

## [0.6.0] - 2017-12-15
**Note:** this update contains a lot of breaking changes.

### Added
- Case-insensitive parsing for color names.
- `error-chain` crate for errors.
- `log` crate for warnings.

### Changed
- XML parsing moved to a separate crate: xmlparser.
- All tokenizer's return `Option<Result<T>>` now and not `Result<T>`.
- `path::Tokenizer` doesn't return errors anymore, because any error should stop parsing anyway.
- Rename `TextFrame` to `StrSpan`.
- Almost all fields in `svg::Token` enum are changed.

### Fixed
- Panic during style attribute parsing.

### Removed
- `Tokenize` trait. The default `Iterator` is used now.
- `Tokens` iterator. The default `Iterator` is used now.
- `EndOfStream` error. Iterator will return `None` instead.
- `Stream` struct.
- `warnln!` macro. `log::warn!` is used instead.

## [0.5.0] - 2017-09-26
**Note:** this update contains breaking changes.

### Added
- Text unescaping support via `TextUnescape`.
- `Tokens` iterator.

### Changed
- 100..900 values of the `font-weight` will be parsed as `ValueId` and not as `Number` now.
- All \*\_raw methods in `Stream` module are renamed to \*\_unchecked.
- `Stream::parse_number` returns only `InvalidNumber` error now.
- **Breaking change.** All `EndOfStream` tokens are removed.
  End of stream is indicated via `Error::EndOfStream` now.

  Prefer using a `Tokens` iterator.

### Fixed
- Panic during a `Color` parsing.

## [0.4.3] - 2017-09-03
### Added
- Character entity references for whitespaces parsing.
  So #x20, #x9, #xD, #xA will be parsed correctly now.

### Fixed
- Prefixed items parsing inside a style attribute.

## [0.4.2] - 2017-07-08
### Fixed
- `font-size='small'` parsing.

## [0.4.1] - 2017-06-15
### Fixed
- Parsing of transform list separated by a comma.

## [0.4.0] - 2017-05-14
### Added
- `AttributeValue::from_str`.
- `path::Tokenizer::from_str`.
- `style::Tokenizer::from_str`.
- `transform::Tokenizer::from_str`.
- `svg::Tokenizer::from_frame`.
- `Stream::from_str`, `Stream::to_text_frame`.
- `Tokenize` trait which defines general parsing methods.
- Implement `FromStr` trait for `Color`.
- `path::Tokenizer`, `svg::Tokenizer`, `style:Tokenizer`, `transform::Tokenizer` are
  implement `Tokenize` now.
- Implement `Display` trait for `AttributeId`, `ElementId` and `ValueId`.

### Changed
- All warnings will be printed to stderr now.
- All `&[u8]` changed to `&str`.
- Rename `RgbColor` to `Color`.
- Rename `TransformToken` to `Token`.
- Rename `SegmentToken` to `Token`.
- `NumberList` and `LengthList` should be created using `from_frame` method now.
- Split `svg::Token::ElementStart` into `XmlElementStart` and `SvgElementStart`.
- Split `svg::Token::Attribute` into `XmlAttribute` and `SvgAttribute`.
- Split `svg::ElementEnd::Close` into `CloseXml` and `CloseSvg`.

### Removed
- `Color::from_stream`. Use `Color::from_frame` instead.
- `AttributeValue::from_stream`. Use `AttributeValue::from_frame` instead.
- `path::Tokenizer::new`. Use `path::Tokenizer::from_frame` instead.
- `style::Tokenizer::new`. Use `style::Tokenizer::from_frame` instead.
- `transform::Tokenizer::new`. Use `transform::Tokenizer::from_frame` instead.
- `svg::Tokenizer::new`. Use `svg::Tokenizer::from_str` instead.
- `Stream::new`. Use `Stream::from_str` instead.
- `Stream::global_pos`, `Stream::parent_text`.
- `path::SegmentData` and `path::Segment`. They are part of `path::Token` now.

## [0.3.1] - 2017-03-15
### Fixed
- Color attribute value parsing.
- Style attribute parsing.

## [0.3.0] - 2017-03-06
### Added
- `Error::Utf8Error` instead of panic during `str::from_utf8` unwrap.
- `Stream::is_letter_raw`.

### Changed
- Use default `f64` parser instead of a custom one.

### Removed
- The `u8_to_str` macro.
- `Error::ElementWithoutTagName`. `Error::InvalidSvgToken` will be emitted instead.
- `Stream::read_to_trimmed`.

### Fixed
- Few panics that can be triggered by an invalid input.
- `.` was parsed as `0` by `Stream::parse_number`. Now it's an error.
- Endless loop when a ClosePath segment was followed by a number.
- `Stream::parse_integer` error when a number is at the end of the stream.
- `Stream::parse_integer` will return an error on an integer overflow now.
- Numbers parsing with a decimal part and an exponent.

## [0.2.1] - 2017-02-01
### Fixed
- Building against new `phf` crate.

## [0.2.0] - 2017-01-14
### Added
- `Stream::global_pos`.
- `Stream::parent_text`.

### Changed
- `Error::EndOfStream` removed. Now all tokens enum contains it's own EndOfStream.
  It's allowed to simplify tokenization loops.
- All tokenizers does not implement `Iterator` trait now.
- Rename `Transform` into `TransformToken`.

### Removed
- Non-SVG elements: `flowPara`, `flowRegion`, `flowRoot` and `flowSpan` from the list of
  known elements.

## [0.1.0] - 2016-10-09
### Added
- `trim_trailing_spaces`, `read_to_trimmed` methods to the `Stream`.
- Support spaces around `=` while attribute parsing.
- Trim spaces from both sides of the attribute value.
- Hash impl for enums.
- All *Units attribute values are parsed now.
- The `InvalidLength` error.

### Changed
- Rename `AdvanceError` into `InvalidAdvance`.
- Rename `len_to_char_or_end` into `len_to_or_end`.
- Rename `jump_to_char_or_end` into `jump_to_or_end`.

## [0.0.3] - 2016-09-20
### Added
- A fallback value parsing from the \<paint\> type.
- Parse the `inherit` value from the `font-family` attribute.

### Changed
- `path::Segment` now returns only command char and segment data. All segment manipulation methods are removed.

### Removed
- `path::Command` struct.

### Fixed
- In SVG path there can be any command after `ClosePath`, not only `MoveTo`.
- Fix `&apos;` parsing inside the style attribute.

## [0.0.2] - 2016-09-09
### Removed
- Remove `BlockProgression` and `TextAlign` from `AttributeId` enum since they are not a SVG attributes.

## 0.0.1 - 2016-08-26
### Added
- Initial release.

[Unreleased]: https://github.com/RazrFalcon/svgparser/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/RazrFalcon/svgparser/compare/v0.6.4...v0.7.0
[0.6.4]: https://github.com/RazrFalcon/svgparser/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/RazrFalcon/svgparser/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/RazrFalcon/svgparser/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/RazrFalcon/svgparser/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/svgparser/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/RazrFalcon/svgparser/compare/v0.4.3...v0.5.0
[0.4.3]: https://github.com/RazrFalcon/svgparser/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/RazrFalcon/svgparser/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/svgparser/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/svgparser/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/RazrFalcon/svgparser/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RazrFalcon/svgparser/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/RazrFalcon/svgparser/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RazrFalcon/svgparser/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/RazrFalcon/svgparser/compare/0.0.3...v0.1.0
[0.0.3]: https://github.com/RazrFalcon/svgparser/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/RazrFalcon/svgparser/compare/0.0.1...0.0.2
