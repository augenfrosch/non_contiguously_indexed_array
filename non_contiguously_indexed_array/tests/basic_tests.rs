mod constants;
use constants::*;

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
