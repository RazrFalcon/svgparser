# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
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

[Unreleased]: https://github.com/RazrFalcon/libsvgparser/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.3...v0.1.0
[0.0.3]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.1...0.0.2
