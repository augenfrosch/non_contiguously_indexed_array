#![no_std]

mod array;
pub use array::*;

mod index;
pub use index::*;

mod iter;
use iter::*;

/// Checks if provided `NciArray` segment data fulfills all invariants.
/// If `true` is returned, the data is safe to use to instantiate a `NciArray` struct, otherwise doing so will cause it to function incorrectly.
pub fn check_segment_data_invariants<I: NciIndex>(
    segments_idx_begin: &[I],
    segments_mem_idx_begin: &[usize],
    values_len: usize,
) -> bool {
    // All internal arrays must be empty or all non-empty.
    if segments_idx_begin.is_empty() == segments_mem_idx_begin.is_empty()
        && segments_mem_idx_begin.is_empty() == (values_len == 0)
    {
        if values_len == 0 {
            return true;
        }
    } else {
        return false;
    }

    // Segments must have corresponding representative elements in both.
    let mut result = segments_idx_begin.len() == segments_mem_idx_begin.len();

    // Test invariants for neighboring segment entries & that memory indices are in bounds.
    result &= segments_mem_idx_begin[0] < values_len;
    for segment in 0..(segments_idx_begin.len() - 1) {
        result &= segments_mem_idx_begin[segment + 1] < values_len;

        // Segment elements must be strictly monotonically increasing.
        result &= segments_idx_begin[segment] < segments_idx_begin[segment + 1];
        result &= segments_mem_idx_begin[segment] < segments_mem_idx_begin[segment + 1];

        // Distance to the next representative index must be greater than the number of entries of the segment.
        // Note that the current representation would still work if the distance is equals to the number of entries,
        // however, this would indicate the construction is not as space efficient as it could be, since the segments could be merged into one.
        // Testing for it to be strictly greater restricts the representation to be unique.
        // For example, when accepting equals, ([1, 2, 4]; [0, 1, 3]) would be a valid, even though it could be reduced to ([1]; [0]).
        result &= segments_idx_begin[segment]
            .distance(segments_idx_begin[segment + 1])
            .unwrap_or(usize::MAX)
            > (segments_mem_idx_begin[segment + 1] - segments_mem_idx_begin[segment]);
    }
    // Test invariants of last segment since it has no successor to check against.
    // For each entry, check if the generation of the next index is successful.
    // The check is skipped for the final entry since no further index has to be generated.
    let mut idx = segments_idx_begin[segments_idx_begin.len() - 1];
    for _mem_idx in segments_mem_idx_begin[segments_mem_idx_begin.len() - 1]..(values_len - 1) {
        if let Some(next_idx) = idx.next() {
            idx = next_idx;
        } else {
            return false;
        }
    }
    result
}
