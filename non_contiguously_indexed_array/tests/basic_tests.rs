mod constants;
use constants::*;
#[macro_use] // TODO: Import the macros properly, without needing to suppress warnings
mod macros;

use non_contiguously_indexed_array::{NciArray, NciIndex};

#[test]
fn test_try() {
    const TEST: NciArray<u32, u32> = NciArray::new(&[0, 10, 100],
        &[0, 3, 5],
        &[0, 1, 2, 10, 11, 100],
    );
    dbg!(TEST);
}

#[test]
fn basic_array_test_1() {
    basic_array_test_normal_case!(ARRAY_1, 0, 1, 2, 10, 11, 100);
    basic_array_test_edge_case!(ARRAY_1, 3, 5, 9, 55, 99, 101, 500);
}

#[test]
fn basic_array_test_2() {
    basic_array_test_normal_case!(ARRAY_2, 100, 101, 200, 500, 501, 502);
    basic_array_test_edge_case!(
        ARRAY_2, 0, 1, 50, 99, 102, 150, 199, 201, 350, 499, 503, 750, 999
    );
}

#[test]
fn basic_array_test_3() {
    basic_array_test_normal_case!(ARRAY_3, -500, -499, -498, -490, -489, -400);
    basic_array_test_edge_case!(
        ARRAY_3, -510, -501, -497, -495, -491, -488, -445, -401, -399, 0
    );
}

#[test]
fn basic_array_test_4() {
    basic_array_test_normal_case!(ARRAY_4, -500, -499, -2, -1, 0, 1, 2, 499, 500);
    basic_array_test_edge_case!(
        ARRAY_4, -510, -501, -498, -250, -10, -3, 3, 10, 250, 498, 501, 999
    );
}

#[test]
fn basic_array_test_5() {
    basic_array_test_normal_case!(
        ARRAY_5,
        0,
        1,
        u128::MAX / 2,
        u128::MAX / 2 + 1,
        u128::MAX - 1,
        u128::MAX
    );
    basic_array_test_edge_case!(
        ARRAY_5,
        50,
        101,
        u128::from(u64::MAX),
        u128::MAX / 2 + u128::MAX / 4,
        u128::MAX - 2
    );
}

#[test]
fn basic_index_test_1() {
    basic_next_test_normal_case!(
        0u8,
        5u16,
        1000u32,
        41u32,
        u32::MAX - 1,
        u64::MAX / 4,
        u128::MAX / 2
    );
    basic_next_test_edge_case!(u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX);

    basic_distance_test_normal_case!(
        (0u32, 500, 500),
        (42u32, 500, 500 - 42),
        (42u32, 41, 1),
        (500u32, 0, 500)
    );
}

#[test]
fn basic_index_test_2() {
    basic_next_test_normal_case!(
        -31i8,
        -5i16,
        -500i32,
        -43i32,
        0i32,
        41i32,
        i32::MIN,
        i32::MAX - 1,
        i64::MAX / 4,
        i128::MIN,
        i128::MAX / 2
    );
    basic_next_test_edge_case!(i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX);

    basic_distance_test_normal_case!(
        (-500i32, 500, 1000),
        (-333i32, -1, 332),
        (-43i32, -42, 1),
        (-500i32, -750, 250),
        (42i32, 41, 1),
        (500i32, 0, 500)
    );
}

#[test]
fn basic_index_test_3() {
    basic_distance_test_normal_case!(
        (i8::MIN, i8::MAX, usize::from(u8::MAX)),
        (i16::MIN, i16::MAX, usize::from(u16::MAX))
    );

    #[cfg(target_pointer_width = "16")]
    {
        basic_distance_test_edge_case!(
            (u32::MIN, u32::MAX),
            (i32::MIN, i32::MAX),
            (u64::MIN, u64::MAX),
            (i64::MIN, i64::MAX)
        );
    }
    #[cfg(target_pointer_width = "32")]
    {
        basic_distance_test_normal_case!((i32::MIN, i32::MAX, usize::try_from(u32::MAX).unwrap()));
        basic_distance_test_edge_case!((u64::MIN, u64::MAX), (i64::MIN, i64::MAX));
    }
    #[cfg(target_pointer_width = "64")]
    {
        basic_distance_test_normal_case!(
            (i32::MIN, i32::MAX, usize::try_from(u32::MAX).unwrap()),
            (i64::MIN, i64::MAX, usize::try_from(u64::MAX).unwrap())
        );
    }
    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    basic_distance_test_edge_case!(
        (u128::MIN, u128::MAX),
        (i128::MIN, i128::MAX),
        (i128::from(u64::MIN), i128::from(u64::MAX) + 1),
        (i128::from(i64::MIN), i128::from(i64::MAX) + 1)
    );
}

macro_rules! basic_iterator_test {
    ($a:tt) => {
        let mut entries = $a.entries();
        let mut indices = $a.indices();
        let mut values = $a.values();

        while let (Some(entry), Some(index), Some(value)) =
            (entries.next(), indices.next(), values.next())
        {
            assert_eq!(entry.0, index);
            assert_eq!(entry.1, value);
            assert_eq!(entry.0, *entry.1); // not generally true
        }

        assert_eq!(entries.next(), None);
        assert_eq!(indices.next(), None);
        assert_eq!(values.next(), None);
    };
}

#[test]
fn basic_iterator_test_array_1() {
    basic_iterator_test!(ARRAY_1);
}

#[test]
fn basic_iterator_test_array_2() {
    basic_iterator_test!(ARRAY_2);
}

#[test]
fn basic_iterator_test_array_3() {
    basic_iterator_test!(ARRAY_3);
}

#[test]
fn basic_iterator_test_array_4() {
    basic_iterator_test!(ARRAY_4);
}

#[test]
fn basic_array_iterator_test_5() {
    basic_iterator_test!(ARRAY_5);
}
