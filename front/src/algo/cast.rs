macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

pub(crate) use cast;

/*
macro_rules! cast_ref {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = &$target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

pub(crate) use cast_ref;
*/

/*
macro_rules! cast_mut {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = mut $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

pub(crate) use cast_mut;
*/

macro_rules! cast_mut_ref {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = &mut $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

pub(crate) use cast_mut_ref;
