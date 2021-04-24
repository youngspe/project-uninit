
// Assert that $a is not a prefix to any of the token sequences in $b
#[doc(hidden)]
#[macro_export]
macro_rules! __assert_not_from_self {
    (
        $head:expr,
        [$($a:tt)+],
        [$($b:tt)*],
    // $d should be the '$' symbol
    $d:tt) => {{
        // define a macro that errors when the given tokens start with $a:
        macro_rules! __fail_if_starts_with {
            // $a and $b are equal:
            ([$($a)+]) => {
                compile_error!(concat!(
                    "Cannot mutably borrow '",
                    stringify!($head),
                    concat!(".", $(stringify!($a)),+),
                    "' more than once at a time",
                ));
            };
            // $a is a prefix to $b
            ([$($a)+$d($d else:tt)+]) => {
                compile_error!(concat!(
                    "Cannot mutably borrow '",
                    stringify!($head),
                    concat!(".", $(stringify!($a)),+),
                    concat!($d(stringify!($d else)),+),
                    "' and and its parent '",
                    stringify!($head),
                    concat!(".", $(stringify!($a)),+),
                    "' at the same time.",
                ));
            };
            ($d else:tt) => {};
        }
        // run this macro on every string in $b
        $(__fail_if_starts_with!($b);)*
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_unique {
    // __assert_unique!(@inner ... ) tests each field path against every other field used in the macro
    // invocation.
    // $head is the expression for the MaybeUninit that we're projecting into
    // current represents the field path we're currently testing against the others
    // [$prev...] are the field paths that come before
    // [$next $rest...] are the field paths that come after
    (@inner $head:expr, $current:tt, [$($prev:tt)*], [$next:tt $($rest:tt)*] ) => {
        // test $current against every path in { $prev, $next, $rest }
        $crate::__assert_not_from_self!($head, $current, [$($prev)* $next $($rest)*], $);
        // move $current to $prev, $next to $current
        $crate::__assert_unique!(@inner $head, $next, [$($prev)* $current], [$($rest)*]);
    };
    // $current is the last field path we need to check, so only test against prev and don't recurse
    (@inner $head:expr, $current:tt, [$($prev:tt)*], [] ) => {
        $crate::__assert_not_from_self!($head, $current, [$($prev)*], $);
    };
    // there is more than one field path; recurse into (@inner...)
    ($head:expr, [$first:tt $($rest:tt)+]) => {
        $crate::__assert_unique!(@inner $head, $first, [], [$($rest)*]);
    };
    // there is at most one field path
    ($head:expr, [$($first:tt)?]) => { /* no problem */ };
}
