use non_contiguously_indexed_array::NciArray;

pub const GENERATED_1: NciArray<u32, u32> = NciArray::new(
	&[
		0,
		10,
		100,
	],
	&[
		0,
		3,
		5,
	],
	&[
		0,
		1,
		2,
		10,
		11,
		100,
	],
);
