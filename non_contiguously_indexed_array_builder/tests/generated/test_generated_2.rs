use non_contiguously_indexed_array::NciArray;

pub const GENERATED_2: NciArray<u32, u32> = NciArray {
	segments_idx_begin: &[
		100,
		200,
		500,
	],
	segments_mem_idx_begin: &[
		0,
		2,
		3,
	],
	values: &[
		100,
		101,
		200,
		500,
		501,
		502,
	],
};
