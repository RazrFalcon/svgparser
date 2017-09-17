// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use std::str;

use svgparser::{TextUnescape, XmlSpace};

macro_rules! unescape {
    ($name:ident, $text:expr, $result:expr) => {
        #[test]
        fn $name() {
            assert_eq!(TextUnescape::unescape($text, XmlSpace::Preserve).unwrap(), $result);
        }
    };
}

macro_rules! spaces {
    ($name:ident, $text:expr, $result1:expr, $result2:expr) => {
        #[test]
        fn $name() {
            assert_eq!(TextUnescape::unescape($text, XmlSpace::Preserve).unwrap(), $result1);
            assert_eq!(TextUnescape::unescape($text, XmlSpace::Default).unwrap(), $result2);
        }
    };
}

unescape!(unescape_1,
    "text",
    "text"
);

unescape!(unescape_2,
    "&#x9;&#xA;&#xD;&#x20;Text&#x09;&#x0a;&#x0d;&#x20;",
    "   Text   "
);

unescape!(unescape_3,
    "&#x30;Text&#x40;",
    "0Text@"
);

unescape!(unescape_4,
    "&apos;Text&apos;",
    "'Text'"
);

unescape!(unescape_5,
    "&quot;&amp;&apos;&lt;&gt;",
    "\"&'<>"
);

unescape!(unescape_6,
    "\t\n\rText\t\r\n",
    "  Text  "
);

// Decimal numeric character reference.
unescape!(unescape_7,
    "&#48;Text&#64;",
    "0Text@"
);

unescape!(unescape_8,
    "&#48;Текст&#64;",
    "0Текст@"
);

// Non-latin hexadecimal numeric character reference.
unescape!(unescape_9,
    "&#x410;",
    "\u{0410}"
);

unescape!(unescape_10,
    "&#x1000;",
    "\u{1000}"
);

spaces!(spaces_1,
    "&#x9;&#xA;&#xD;&#x20;Text&#x09;&#x0a;&#x0d;&#x20;",
    "   Text   ",
    " Text "
);

spaces!(spaces_2,
    "\t\n\r Text\t\r\n ",
    "   Text   ",
    " Text "
);
