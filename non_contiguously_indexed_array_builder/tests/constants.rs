use non_contiguously_indexed_array::NciArray;

pub const ARRAY_1: NciArray<u32> = NciArray {
    index_range_starting_indices: &[0, 10, 100],
    index_range_skip_amounts: &[0, 7, 95],
    values: &[0, 1, 2, 10, 11, 100],
};
pub const ARRAY_2: NciArray<u32> = NciArray {
    index_range_starting_indices: &[100, 200, 500],
    index_range_skip_amounts: &[100, 198, 497],
    values: &[100, 101, 200, 500, 501, 502],
};

#[test]
fn test_constants_1() {
    assert_eq!(ARRAY_1.index_range_starting_indices.len(), 3);
    assert_eq!(ARRAY_1.index_range_skip_amounts.len(), 3);
    assert_eq!(ARRAY_1.values.len(), 6);
}
