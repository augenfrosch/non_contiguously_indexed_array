use non_contiguously_indexed_array::NciArray;

pub const GENERATED_2: NciArray<u32, u32> = NciArray::new(
	&[
		100,
		200,
		500,
	],
	&[
		0,
		2,
		3,
	],
	&[
		100,
		101,
		200,
		500,
		501,
		502,
	],
);
