use non_contiguously_indexed_array::NciBaseArray;

pub const BASE_ARRAY_1: NciBaseArray<i32, 2, 6> = NciBaseArray {
    index_ranges: [(10, 7), (100, 88)],
    data: [0, 1, 2, 10, 11, 100],
};
pub const BASE_ARRAY_2: NciBaseArray<i32, 3, 6> = NciBaseArray {
    index_ranges: [(100, 100), (200, 98), (500, 299)],
    data: [100, 101, 200, 500, 501, 502],
};

#[test]
fn test_constants_1() {
    assert_eq!(BASE_ARRAY_1.index_ranges.len(), 2);
    assert_eq!(BASE_ARRAY_1.data.len(), 6);
}
