// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// We don't use cargo build script, since this data will be changed rarely.
// There are no point in regenerating them each time, only if you want to save a few KiB.

extern crate phf_codegen;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::io::prelude::*;

fn main() {
    gen_file("spec/elements.txt", "ElementId", "ELEMENTS",
             "List of all SVG elements.",
             "../src/element_id.rs");
    gen_file("spec/attributes.txt", "AttributeId", "ATTRIBUTES",
             "List of all SVG attributes.",
             "../src/attribute_id.rs");
    gen_file("spec/values.txt", "ValueId", "VALUES",
             "List of all values for presentation attributes.",
             "../src/value_id.rs");

    gen_colors();
}

fn gen_colors() {
    let path = Path::new("../src/colors.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let map_name = "COLORS";
    let struct_name = "Color";

    write_header(&mut file);

    writeln!(&mut file, "use {};\n", struct_name).unwrap();

    write!(&mut file, "static {}: ::phf::Map<&'static str, {}> = ", map_name, struct_name).unwrap();
    let mut map = phf_codegen::Map::new();

    let colors_file = File::open("spec/colors.txt").unwrap();
    let colors_file = BufReader::new(colors_file);
    let mut lines = colors_file.lines();
    while let Some(line) = lines.next() {
        let line1 = line.unwrap();
        let line2 = lines.next().unwrap().unwrap();

        let rgb: Vec<&str> = line2.split(',').collect();

        map.entry(line1, &format!("{}{{ red: {}, green: {}, blue: {} }}", struct_name,
                  rgb[0], rgb[1], rgb[2]));
    }
    map.build(&mut file).unwrap();
    writeln!(&mut file, ";\n").unwrap();

    writeln!(&mut file,
"pub fn rgb_color_from_name(text: &str) -> Option<{}> {{
    {}.get(text).cloned()
}}", struct_name, map_name).unwrap();
}

fn gen_file(spec_path: &str, enum_name: &str, map_name: &str, doc: &str, out_path: &str) {
    let path = Path::new(out_path);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let names = gen_names(spec_path);

    write_header(&mut file);

    writeln!(&mut file, "use std::fmt;\n").unwrap();

    // enum
    writeln!(&mut file, "/// {}", doc).unwrap();
    writeln!(&mut file, "#[derive(Copy,Clone,Eq,PartialEq,PartialOrd,Ord,Hash)]").unwrap();
    writeln!(&mut file, "#[allow(missing_docs)]").unwrap();
    writeln!(&mut file, "pub enum {} {{", enum_name).unwrap();
    for name in &names {
        writeln!(&mut file, "    {},", name.for_enum).unwrap();
    }
    writeln!(&mut file, "}}\n").unwrap();

    // hashmap
    write!(&mut file, "static {}: ::phf::Map<&'static str, {}> = ", map_name, enum_name).unwrap();
    let mut map = phf_codegen::Map::new();
    for name in &names {
        map.entry(name.orig.clone(), &format!("{}::{}", enum_name, name.for_enum));
    }
    map.build(&mut file).unwrap();
    writeln!(&mut file, ";\n").unwrap();

    // enum to string
    writeln!(&mut file, "impl {} {{", enum_name).unwrap();
    writeln!(&mut file, "    /// Converts name into id.").unwrap();
    writeln!(&mut file, "    pub fn from_name(text: &str) -> Option<{}> {{", enum_name).unwrap();
    writeln!(&mut file, "        {}.get(text).cloned()", map_name).unwrap();
    writeln!(&mut file, "    }}").unwrap();
    writeln!(&mut file, "").unwrap();
    writeln!(&mut file, "    /// Converts id into name.").unwrap();
    writeln!(&mut file, "    pub fn name(&self) -> &str {{").unwrap();
    writeln!(&mut file, "        match *self {{").unwrap();
    for name in &names {
        writeln!(&mut file, "            {}::{} => \"{}\",", enum_name, name.for_enum, name.orig).unwrap();
    }
    writeln!(&mut file, "        }}").unwrap();
    writeln!(&mut file, "    }}").unwrap();
    writeln!(&mut file, "}}").unwrap();
    writeln!(&mut file, "").unwrap();
    writeln!(&mut file,
"impl fmt::Debug for {} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, \"{{}}\", self.name())
    }}
}}", enum_name).unwrap();
}

struct Name {
    orig: String,
    for_enum: String,
}

fn gen_names(spec_path: &str) -> Vec<Name> {
    let f = File::open(spec_path).unwrap();
    let f = BufReader::new(f);

    let mut names = Vec::new();

    for line in f.lines() {
        let s = line.unwrap();

        names.push(Name {
            orig: s.clone(),
            for_enum: prepare_enum_name(&s),
        });
    }

    names
}

// some-string -> SomeString
// some_string -> SomeString
// some:string -> SomeString
// 100 -> N100
fn prepare_enum_name(name: &str) -> String {
    let mut chars: Vec<u8> = name.as_bytes().iter().cloned().collect();


    if (chars[0] as char).is_digit(10) {
        chars.insert(0, b'N')
    } else {
        chars[0] = (chars[0] as char).to_uppercase().next().unwrap() as u8;
    }

    // invalid char usually occurs only once, so this code is efficient
    while let Some(idx) = chars.iter().position(|x| *x == b'-' || *x == b'_' || *x == b':') {
        chars.remove(idx);
        chars[idx] = (chars[idx] as char).to_uppercase().next().unwrap() as u8;
    }

    String::from_utf8(chars).unwrap()
}

fn write_header(file: &mut BufWriter<File>) {
    writeln!(file, "// This Source Code Form is subject to the terms of the Mozilla Public").unwrap();
    writeln!(file, "// License, v. 2.0. If a copy of the MPL was not distributed with this").unwrap();
    writeln!(file, "// file, You can obtain one at http://mozilla.org/MPL/2.0/.\n").unwrap();

    writeln!(file, "// This file is autogenerated. Do not edit it!\n").unwrap();
}
