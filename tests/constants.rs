use non_contiguously_indexed_array::NciArrayData;

pub const ARRAY_DATA_1: NciArrayData<i32, 2, 6> = NciArrayData {
    index_ranges: [(10, 7), (100, 88)],
    values: [0, 1, 2, 10, 11, 100],
};
pub const ARRAY_DATA_2: NciArrayData<i32, 3, 6> = NciArrayData {
    index_ranges: [(100, 100), (200, 98), (500, 299)],
    values: [100, 101, 200, 500, 501, 502],
};

#[test]
fn test_constants_1() {
    assert_eq!(ARRAY_DATA_1.index_ranges.len(), 2);
    assert_eq!(ARRAY_DATA_1.values.len(), 6);
}
