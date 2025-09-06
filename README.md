# non_contiguously_indexed_array

`non_contiguously_indexed_array` is a library for arrays that are indexed non-contiguously, i.e., arrays where some index ranges do not map to entries.

The library takes inspiration from the design of [`rust-phf`](https://github.com/rust-phf/rust-phf), as it was developed as a partial drop-in replacement for use cases where the keys are unsigned integers. For this use case, this library offers a potentially more space-efficient representation, at the cost of decreased performance.

> [!WARNING]
> The design of the library is not finalised. Breaking changes can occur on minor version changes.

## Performance
The performance of the data structure is lower than that of `phf::OrderedMap` or similar. In a basic test, it was around 2 to 3 times slower, but this depends on the number of valid index ranges.
Indexing requires a binary search over valid index ranges, with each iteration requiring a memory read, so the performance is proportional to the logarithm of the number of index ranges.

The current data structure for the array is only space-efficient if the average length of continuous index ranges is long enough, as for each range, two values of the index type have to be stored. 

## Usage
The current main way of generating a `non_contiguously_indexed_array::NciArray` is by Rust codegen via a `non_contiguously_indexed_array::NciArrayBuilder`, e.g., using a build script.
