## svgparser [![Build Status]](https://travis-ci.org/RazrFalcon/svgparser)

[Build Status]: https://travis-ci.org/RazrFalcon/svgparser.svg?branch=master

*svgparser* is a pull-based parser for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
data format without heap allocations.

It's not an XML parser since it does not only split the content into the XML nodes,
but also supports [SVG types](https://www.w3.org/TR/SVG/types.html#BasicDataTypes) parsing.

## Table of Contents
- [Documentation](#documentation)
- [Supported SVG types](#supported-svg-types)
- [Benefits](#benefits)
- [Limitations](#limitations)
- [Safety](#safety)
- [Alternatives](#alternatives)
- [Usage](#usage)
- [License](#license)
- [Contribution](#contribution)

### [Documentation](https://docs.rs/svgparser/)

### Supported SVG types
 - [\<color\>](https://www.w3.org/TR/SVG/types.html#DataTypeColor)
 - [\<paint\>](https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint)
 - [\<path\>](https://www.w3.org/TR/SVG/paths.html#PathData)
 - [\<number\>](https://www.w3.org/TR/SVG/types.html#DataTypeNumber) and \<list-of-numbers\>
 - [\<length\>](https://www.w3.org/TR/SVG/types.html#DataTypeLength) and \<list-of-lengths\>
 - [\<coordinate\>](https://www.w3.org/TR/SVG/types.html#DataTypeCoordinate)
 - [\<IRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeIRI)
 - [\<FuncIRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI)
 - [\<transform-list\>](https://www.w3.org/TR/SVG/types.html#DataTypeTransformList)
 - [\<style\>](https://www.w3.org/TR/SVG/styling.html#StyleAttribute)
 - [\<viewBox\>](https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute)
 - [\<list-of-points\>](https://www.w3.org/TR/SVG11/shapes.html#PointsBNF)

See the documentation for details.

### Benefits
 - Most of the common data parsed into internal representation, and not just as string
   (unlike typical XML parser). Tag names, attribute names, attributes value, etc.
 - Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
 - [Predefined SVG values](https://www.w3.org/TR/SVG/propidx.html) for presentation attributes,
   like `auto`, `normal`, `none`, `inherit`, etc. are parsed as `enum`, not as `String`.
 - Every type can be parsed separately, so you can parse just paths or transform
   or any other SVG value.
 - Good error processing. All error types contain position (line:column) where it occurred.
 - No heap allocations.
 - Pretty fast.

### Limitations
 - All keywords must be lowercase.
   Case-insensitive parsing is supported only for colors (requires allocation for named colors).
   Still, it's extremely rare.
 - The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
 - Only ENTITY objects are parsed from the DOCTYPE. Other ignored.
 - CSS styles does not processed. You should use an external CSS parser.
 - Comments inside attributes value supported only for the `style` attribute.
 - [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`, are not supported.
 - There is no separate `opacity-value` type. It will be parsed as `<number>`,
   but will be bound to 0..1 range.
 - Implicit path commands are not supported. All commands are parsed as explicit.
 - Implicit MoveTo commands will be automatically converted into explicit LineTo.

### Safety

 - The library should not panic. Any panic considered as a critical bug
   and should be reported.
 - The library forbids unsafe code.

### Alternatives

- [svg](https://crates.io/crates/svg), which has about 10% of `svgparser` features.
  Also has minimal writing capabilities, unlike `svgparser`, which is parser only.

If you need writing and DOM manipulations - checkout
[svgdom](https://crates.io/crates/svgdom) crate, which is built on top of `svgparser`.

If you know about other alternatives - please send a pull request.

### Usage

Dependency: [Rust](https://www.rust-lang.org/) >= 1.17

Add this to your `Cargo.toml`:

```toml
[dependencies]
svgparser = "0.7"
```

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
