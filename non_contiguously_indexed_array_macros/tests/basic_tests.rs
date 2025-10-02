use non_contiguously_indexed_array::NciArray;
use non_contiguously_indexed_array_macros::nci_array;

// TODO: remove copy-paste; maybe a general test crate makes sense
macro_rules! basic_array_test_normal_case {
    ($a:tt, $( $i:expr ),*) => {
        $(
            assert_eq!($a.get($i), Some(&$i));
			assert!($a.has_entry($i));
        )*
    };
}

macro_rules! basic_array_test_edge_case {
    ($a:tt, $( $i:expr ),*) => {
        $(
            assert_eq!($a.get($i), None);
			assert!(!$a.has_entry($i));
        )*
    };
}

#[test]
fn basic_array_test_1() {
    const ARRAY_1: NciArray<u32, u32> = nci_array! {
        0 => 0,
        1 => 1,
        2 => 2,
        10 => 10,
        11 => 11,
        100 => 100,
    };

    assert_eq!(ARRAY_1.segments_idx_begin.len(), 3);
    assert_eq!(ARRAY_1.segments_mem_idx_begin.len(), 3);
    assert_eq!(ARRAY_1.values.len(), 6);

    basic_array_test_normal_case!(ARRAY_1, 0, 1, 2, 10, 11, 100);
    basic_array_test_edge_case!(ARRAY_1, 3, 5, 9, 55, 99, 101, 500);
}

#[test]
fn basic_array_test_4() {
    pub const ARRAY_4: NciArray<i32, i32> = nci_array! {
        -500 => -500,
        -499 => -499,
        -2 => -2,
        -1 => -1,
        0 => 0,
        1 => 1,
        2 => 2,
        499 => 499,
        500 => 500,
    };

    assert_eq!(ARRAY_4.segments_idx_begin.len(), 3);
    assert_eq!(ARRAY_4.segments_mem_idx_begin.len(), 3);
    assert_eq!(ARRAY_4.values.len(), 9);

    basic_array_test_normal_case!(ARRAY_4, -500, -499, -2, -1, 0, 1, 2, 499, 500);
    basic_array_test_edge_case!(
        ARRAY_4, -510, -501, -498, -250, -10, -3, 3, 10, 250, 498, 501, 999
    );
}
