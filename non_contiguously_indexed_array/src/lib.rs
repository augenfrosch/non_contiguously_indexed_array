#![no_std]

pub use non_contiguously_indexed_array_shared::*;

mod array;
pub use array::*;

mod iter;
use iter::*;

#[cfg(feature = "macros")]
pub use non_contiguously_indexed_array_macros::*;
