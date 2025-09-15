#[allow(unused_macros)]
macro_rules! basic_array_test_normal_case {
    ($a:tt, $( $i:expr ),*) => {
        $(
            assert_eq!($a.get($i), Some(&$i));
        )*
    };
}

#[allow(unused_macros)]
macro_rules! basic_array_test_edge_case {
    ($a:tt, $( $i:expr ),*) => {
        $(
            assert_eq!($a.get($i), None);
        )*
    };
}

#[allow(unused_macros)]
macro_rules! basic_next_test_normal_case {
    ( $( $i:expr ),* ) => {
        $(
            assert_eq!($i.next(), Some($i.checked_add(1).unwrap()));
        )*
    };
}

#[allow(unused_macros)]
macro_rules! basic_next_test_edge_case {
    ( $( $i:expr ),* ) => {
        $(
            assert_eq!($i.next(), None);
        )*
    };
}

#[allow(unused_macros)]
macro_rules! basic_distance_test_normal_case {
    ( $( $i:expr ),* ) => {
        $(
            let (first, second, expected) = $i;
            assert_eq!(first.distance(second), Some(expected));
            assert_eq!(second.distance(first), Some(expected));
        )*
    };
}

#[allow(unused_macros)]
macro_rules! basic_distance_test_edge_case {
    ( $( $i:expr ),* ) => {
        $(
            let (first, second) = $i;
            assert_eq!(first.distance(second), None);
            assert_eq!(second.distance(first), None);
        )*
    };
}
