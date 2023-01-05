macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = &mut $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

pub(crate) use cast;
