mod constants;
use constants::*;
use non_contiguously_indexed_array::NciIndex;

#[test]
fn basic_array_test_1() {
    let arr = ARRAY_1;
    let values = arr.values().copied().collect::<Vec<_>>();
    assert_eq!(values, ARRAY_1.values);

    assert_eq!(arr.get(0), Some(&0));
    assert_eq!(arr.get(1), Some(&1));
    assert_eq!(*arr.get(1).unwrap(), arr[1]);
    assert_eq!(arr.get(2), Some(&2));
    assert_eq!(arr.get(5), None);
    assert_eq!(arr.get(10), Some(&10));
    assert_eq!(arr.get(11), Some(&11));
    assert_eq!(*arr.get(11).unwrap(), arr[11]);
    assert_eq!(arr.get(55), None);
    assert_eq!(arr.get(99), None);
    assert_eq!(arr.get(100), Some(&100));
    assert_eq!(*arr.get(100).unwrap(), arr[100]);
    assert_eq!(arr.get(101), None);
}

#[test]
fn basic_array_test_2() {
    let arr = ARRAY_2;
    let values = arr.values().copied().collect::<Vec<_>>();
    assert_eq!(values, ARRAY_2.values);

    assert_eq!(arr.get(0), None);
    assert_eq!(arr.get(1), None);
    assert_eq!(arr.get(50), None);
    assert_eq!(arr.get(99), None);
    assert_eq!(arr.get(100), Some(&100));
    assert_eq!(arr.get(101), Some(&101));
    assert_eq!(*arr.get(101).unwrap(), arr[101]);
    assert_eq!(arr.get(102), None);
    assert_eq!(arr.get(150), None);
    assert_eq!(arr.get(199), None);
    assert_eq!(arr.get(200), Some(&200));
    assert_eq!(*arr.get(200).unwrap(), arr[200]);
    assert_eq!(arr.get(201), None);
    assert_eq!(arr.get(350), None);
    assert_eq!(arr.get(499), None);
    assert_eq!(arr.get(500), Some(&500));
    assert_eq!(arr.get(501), Some(&501));
    assert_eq!(arr.get(502), Some(&502));
    assert_eq!(*arr.get(502).unwrap(), arr[502]);
    assert_eq!(arr.get(503), None);
    assert_eq!(arr.get(750), None);
    assert_eq!(arr.get(999), None);
}

#[test]
fn basic_array_test_3() {
    let arr = ARRAY_3;
    let values = arr.values().copied().collect::<Vec<_>>();
    assert_eq!(values, ARRAY_3.values);

    assert_eq!(arr.get(-510), None);
    assert_eq!(arr.get(-501), None);
    assert_eq!(arr.get(-500), Some(&-500));
    assert_eq!(arr.get(-499), Some(&-499));
    assert_eq!(arr.get(-498), Some(&-498));
    assert_eq!(arr.get(-497), None);
    assert_eq!(arr.get(-495), None);
    assert_eq!(arr.get(-490), Some(&-490));
    assert_eq!(arr.get(-489), Some(&-489));
    assert_eq!(arr.get(-488), None);
    assert_eq!(arr.get(-445), None);
    assert_eq!(arr.get(-401), None);
    assert_eq!(arr.get(-400), Some(&-400));
    assert_eq!(arr.get(-399), None);
}

#[test]
fn basic_array_test_4() {
    let arr = ARRAY_4;
    let values = arr.values().copied().collect::<Vec<_>>();
    assert_eq!(values, ARRAY_4.values);

    assert_eq!(arr.get(-510), None);
    assert_eq!(arr.get(-501), None);
    assert_eq!(arr.get(-500), Some(&-500));
    assert_eq!(arr.get(-499), Some(&-499));
    assert_eq!(arr.get(-250), None);
    assert_eq!(arr.get(-10), None);
    assert_eq!(arr.get(-3), None);
    assert_eq!(arr.get(-2), Some(&-2));
    assert_eq!(arr.get(-1), Some(&-1));
    assert_eq!(arr.get(0), Some(&0));
    assert_eq!(arr.get(1), Some(&1));
    assert_eq!(arr.get(2), Some(&2));
    assert_eq!(arr.get(3), None);
    assert_eq!(arr.get(10), None);
    assert_eq!(arr.get(250), None);
    assert_eq!(arr.get(499), Some(&499));
    assert_eq!(arr.get(500), Some(&500));
    assert_eq!(arr.get(501), None);
}

#[test]
fn basic_array_test_5() {
    let arr = ARRAY_5;
    let values = arr.values().copied().collect::<Vec<_>>();
    assert_eq!(values, ARRAY_5.values);

    assert_eq!(arr.get(0), Some(&0));
    assert_eq!(arr.get(1), Some(&1));
    assert_eq!(arr.get(101), None);
    assert_eq!(arr.get(u128::from(u64::MAX)), None);
    assert_eq!(arr.get(u128::MAX / 2), Some(u128::MAX / 2).as_ref());
    assert_eq!(arr.get(u128::MAX / 2 + 1), Some(u128::MAX / 2 + 1).as_ref());
    assert_eq!(arr.get(u128::MAX / 2 + u128::MAX / 4), None);
    assert_eq!(arr.get(u128::MAX - 2), None);
    assert_eq!(arr.get(u128::MAX - 1), Some(u128::MAX - 1).as_ref());
    assert_eq!(arr.get(u128::MAX), Some(u128::MAX).as_ref());
}

#[test]
fn basic_index_test_1() {
    assert_eq!(0u32.next(), Some(1u32));
    assert_eq!(41u32.next(), Some(42u32));
    assert_eq!(u32::MAX.next(), None);

    assert_eq!(0u32.distance(500), Some(500));
    assert_eq!(42u32.distance(500), Some(500 - 42));

    assert_eq!(42u32.distance(41), Some(1));
    assert_eq!(500u32.distance(0), Some(500));
}

#[test]
fn basic_index_test_2() {
    assert_eq!((-500i32).next(), Some(-499i32));
    assert_eq!((-43i32).next(), Some(-42i32));
    assert_eq!(0i32.next(), Some(1i32));
    assert_eq!(41i32.next(), Some(42i32));
    assert_eq!(i32::MIN.next(), Some(i32::MIN + 1));
    assert_eq!(i32::MAX.next(), None);

    assert_eq!((-500i32).distance(500), Some(1000));
    assert_eq!((-333i32).distance(-1), Some(332));
    assert_eq!((-43i32).distance(-42), Some(1));
    assert_eq!(0i32.distance(500), Some(500));
    assert_eq!(42i32.distance(500), Some(500 - 42));

    assert_eq!((-500i32).distance(-750), Some(250));
    assert_eq!((-41i32).distance(-42), Some(1));
    assert_eq!(1i32.distance(0), Some(1));
    assert_eq!(42i32.distance(41), Some(1));
    assert_eq!(500i32.distance(0), Some(500));
}

#[test]
fn basic_index_test_3() {
    assert_eq!(i8::MIN.distance(i8::MAX), Some(usize::from(u8::MAX)));
    assert_eq!(i16::MIN.distance(i16::MAX), Some(usize::from(u16::MAX)));

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    assert_eq!(u128::MIN.distance(u128::MAX), None);
}

#[test]
fn basic_array_iterator_test_1() {
    let arr = ARRAY_1;

    let mut entries = arr.entries();
    let mut indices = arr.indices();
    let mut values = arr.values();

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
}

#[test]
fn basic_array_iterator_test_2() {
    let arr = ARRAY_2;

    let mut entries = arr.entries();
    let mut indices = arr.indices();
    let mut values = arr.values();

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
}

#[test]
fn basic_array_iterator_test_3() {
    let arr = ARRAY_3;

    let mut entries = arr.entries();
    let mut indices = arr.indices();
    let mut values = arr.values();

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
}

#[test]
fn basic_array_iterator_test_4() {
    let arr = ARRAY_4;

    let mut entries = arr.entries();
    let mut indices = arr.indices();
    let mut values = arr.values();

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
}

#[test]
fn basic_array_iterator_test_5() {
    let arr = ARRAY_5;

    let mut entries = arr.entries();
    let mut indices = arr.indices();
    let mut values = arr.values();

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
}
