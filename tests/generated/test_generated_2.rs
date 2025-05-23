use non_contiguously_indexed_array::NciArrayData;

pub const GENERATED_2: NciArrayData<i32, 3, 6> = NciArrayData{
	index_ranges: [
		(100, 100),
		(200, 98),
		(500, 299),
	],
	values: [
		100,
		101,
		200,
		500,
		501,
		502,
	],
};
