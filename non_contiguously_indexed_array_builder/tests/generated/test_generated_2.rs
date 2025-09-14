use non_contiguously_indexed_array::NciArray;

pub const GENERATED_2: NciArray<usize, usize> = NciArray {
	index_range_starting_indices: &[
		100,
		200,
		500,
	],
	index_range_skip_amounts: &[
		100,
		198,
		497,
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
