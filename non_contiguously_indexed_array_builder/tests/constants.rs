use non_contiguously_indexed_array::NciArray;

pub const ARRAY_1: NciArray<u32, u32> = NciArray {
    segments_idx_begin: &[0, 10, 100],
    segments_mem_idx_begin: &[0, 3, 5],
    values: &[0, 1, 2, 10, 11, 100],
};

pub const ARRAY_2: NciArray<u32, u32> = NciArray {
    segments_idx_begin: &[100, 200, 500],
    segments_mem_idx_begin: &[0, 2, 3],
    values: &[100, 101, 200, 500, 501, 502],
};

pub const ARRAY_3: NciArray<i32, i32> = NciArray {
    segments_idx_begin: &[-500, -490, -400],
    segments_mem_idx_begin: &[0, 3, 5],
    values: &[-500, -499, -498, -490, -489, -400],
};

pub const ARRAY_4: NciArray<i32, i32> = NciArray {
    segments_idx_begin: &[-500, -2, 499],
    segments_mem_idx_begin: &[0, 2, 7],
    values: &[-500, -499, -2, -1, 0, 1, 2, 499, 500],
};

#[test]
fn test_constants_1() {
    assert_eq!(ARRAY_1.segments_idx_begin.len(), 3);
    assert_eq!(ARRAY_1.segments_mem_idx_begin.len(), 3);
    assert_eq!(ARRAY_1.values.len(), 6);
}
