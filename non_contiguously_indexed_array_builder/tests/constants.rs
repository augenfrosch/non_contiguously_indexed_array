use non_contiguously_indexed_array::NciArray;

pub const ARRAY_1: NciArray<u32, u32> = NciArray::new(
    &[0, 10, 100],
    &[0, 3, 5],
    &[0, 1, 2, 10, 11, 100],
);

pub const ARRAY_2: NciArray<u32, u32> = NciArray::new(
    &[100, 200, 500],
    &[0, 2, 3],
    &[100, 101, 200, 500, 501, 502],
);

pub const ARRAY_3: NciArray<i32, i32> = NciArray::new(
    &[-500, -490, -400],
    &[0, 3, 5],
    &[-500, -499, -498, -490, -489, -400],
);

pub const ARRAY_4: NciArray<i32, i32> = NciArray::new(
    &[-500, -2, 499],
    &[0, 2, 7],
    &[-500, -499, -2, -1, 0, 1, 2, 499, 500],
);
