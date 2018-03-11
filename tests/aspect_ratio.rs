extern crate svgparser;

use std::str::FromStr;

use svgparser::{
    AspectRatio,
    Align,
};

macro_rules! test {
    ($name:ident, $text:expr, $result:expr) => (
        #[test]
        fn $name() {
            let v = AspectRatio::from_str($text).unwrap();
            assert_eq!(v, $result);
        }
    )
}

test!(defer_1, "none", AspectRatio {
    defer: false,
    align: Align::None,
    slice: false,
});

test!(defer_2, "defer none", AspectRatio {
    defer: true,
    align: Align::None,
    slice: false,
});

test!(align_1, "xMinYMid", AspectRatio {
    defer: false,
    align: Align::XMinYMid,
    slice: false,
});

test!(slice_1, "xMinYMid slice", AspectRatio {
    defer: false,
    align: Align::XMinYMid,
    slice: true,
});

test!(slice_2, "xMinYMid meet", AspectRatio {
    defer: false,
    align: Align::XMinYMid,
    slice: false,
});
