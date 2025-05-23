use non_contiguously_indexed_array::NciBaseArray;

pub const GENERATED_2: NciBaseArray<i32, 3, 6> = NciBaseArray{
	index_ranges: [
		(100, 100),
		(200, 98),
		(500, 299),
	],
	data: [
		100,
		101,
		200,
		500,
		501,
		502,
	],
};
