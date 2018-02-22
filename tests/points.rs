// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate svgparser;

use svgparser::{
    FromSpan,
    Points,
};

macro_rules! test {
    ($name:ident, $text:expr, $($value:expr),*) => (
        #[test]
        fn $name() {
            let mut pts = Points::from_str($text);
            $(
                assert_eq!(pts.next().unwrap(), $value);
            )*

            assert_eq!(pts.next().is_none(), true);
        }
    )
}

test!(points_1, "1 2 3 4",
    (1.0, 2.0),
    (3.0, 4.0)
);

test!(points_err_1, "1", );

test!(points_err_2, "1 2 3",
    (1.0, 2.0)
);

test!(points_err_4, "1 2 3 t",
    (1.0, 2.0)
);