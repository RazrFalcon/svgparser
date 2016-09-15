## libsvgparser

*libsvgparser* is a streaming parser/tokenizer for [SVG 1.1 Full](https://www.w3.org/TR/SVG/)
data format without heap allocations.

It's not a XML parser, since it's not only splits the content into the XML nodes,
but also supports [SVG types](https://www.w3.org/TR/SVG/types.html#BasicDataTypes) parsing.

### [Documentation](https://docs.rs/svgparser/)

### Supported SVG types
 - [\<color\>](https://www.w3.org/TR/SVG/types.html#DataTypeColor)
 - [\<paint\>](https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint)
 - [\<path\>](https://www.w3.org/TR/SVG/paths.html#PathData)
 - [\<number\>](https://www.w3.org/TR/SVG/types.html#DataTypeNumber) and \<list-of-number\>
 - [\<length\>](https://www.w3.org/TR/SVG/types.html#DataTypeLength) and \<list-of-length\>
 - [\<coordinate\>](https://www.w3.org/TR/SVG/types.html#DataTypeCoordinate)
 - [\<IRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeIRI)
 - [\<FuncIRI\>](https://www.w3.org/TR/SVG/types.html#DataTypeFuncIRI)
 - [\<transform-list\>](https://www.w3.org/TR/SVG/types.html#DataTypeTransformList)
 - [\<style\>](https://www.w3.org/TR/SVG/styling.html#StyleAttribute)

See documentation for details.

### Benefits
 - Most of the common data parsed into internal representation, and not just as string
   (unlike typical XML parser). Tag names, attribute names, attributes value, etc.
 - Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
 - [Predefined SVG values](https://www.w3.org/TR/SVG/propidx.html) for presentation attributes,
   like `auto`, `normal`, `none`, `inherit`, etc. are parsed as enum, not as string.
 - Every type can be parsed separately, so you can parse just paths, or transform
   or any other SVG value.
 - Good error processing. All error types contains position (line:column) where it occurred.
 - No heap allocations.
 - Pretty fast.

### Limitations
 - All keywords must be lowercase. Case-insensitive parsing is not supported.
   Still it's extremely rare.
 - The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
 - Only ENTITY objects are parsed from DOCTYPE. Other ignored.
 - CSS styles does not processed. You should use external CSS parser.
 - Comments inside attributes value supported only for `style` attribute.
 - User agent colors, aka `fill="AppWorkspace"`, is not suppored.
 - There is no separate `opacity-value` type. It will be parsed as `<number>`,
   but will be bound to 0..1 range.
 - An implicit path commands is not supported. All commands are parsed as explicit.
 - An implicit MoveTo commands automatically converted to an explicit LineTo.

### Differences between *libsvgparser* and SVG spec
 - `<percentage>` type is part of `<length>` type.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
svgparser = "0.0.2"
```

### Roadmap

V0.1.0
 - [ ] Parsing `f64` from a string is pretty slow (`Stream::parse_number`).

V0.2.0
 - [ ] Add an `<angle>` type support.

### License

*libsvgparser* is licensed under the [MPLv2.0](https://www.mozilla.org/en-US/MPL/).
