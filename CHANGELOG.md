# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
- A fallback value parsing from the \<paint\> type.
- Parse the `inherit` value from the `font-family` attribute.

### Changed
- `path::Segment` now returns only command char and segment data. All segment manipulation methods are removed.

### Removed
- `path::Command` struct.

## [0.0.2] - 2016-09-09
### Removed
- Remove `BlockProgression` and `TextAlign` from `AttributeId` enum since they are not a SVG attributes.

## 0.0.1 - 2016-08-26
### Added
- Initial release.

[Unreleased]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.2...HEAD
[0.0.2]: https://github.com/RazrFalcon/libsvgparser/compare/0.0.1...0.0.2
