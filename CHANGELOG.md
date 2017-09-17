# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
- Text unescaping support via `TextUnescape`.
- Character entity references for whitespaces parsing.
  So #x20, #x9, #xD, #xA will be parsed correctly now.

### Changed
- 100..900 values of the `font-weight` will be parsed as `ValueId` and not as `Number` now.
- All \*\_raw methods in `Stream` module are renamed to \*\_unchecked.
- `Stream::parse_number` returns only `InvalidNumber` error now.

### Fixed
- Panic during `Color` parsing.

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

[Unreleased]: https://github.com/RazrFalcon/libsvgparser/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/RazrFalcon/libsvgparser/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/libsvgparser/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/libsvgparser/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/RazrFalcon/libsvgparser/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RazrFalcon/libsvgparser/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/RazrFalcon/libsvgparser/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RazrFalcon/libsvgparser/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.3...v0.1.0
[0.0.3]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.1...0.0.2
