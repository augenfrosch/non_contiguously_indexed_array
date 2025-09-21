#![no_std]

mod array;
pub use array::*;

mod index;
pub use index::*;

mod iter;
use iter::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NciArrayInvariant {
    EntriesStartAtMemoryIndexZero,
    IsEmptyEquivalent,
    MemoryIndicesInBounds,
    NciIndexCalculatesCorrectDistanceForAllEntries,
    NciIndexCanGenerateIndicesForAllEntries,
    SegmentsDisjoint,
    SegmentDataMinimal,
    SegmentDataElementsStrictlyMonotonic,
    SegmentDataLengthEquivalent,
}

/// Checks if provided `NciArray` segment data fulfills all invariants. The function assumes that the `NciIndex` is implemented correctly.
/// If all checks pass, `Ok(())` is returned, indicating that the data is safe to use to instantiate a `NciArray` struct.
///
/// # Error
///
/// Returns an `Err` variant containing the first invariant found to be violated running the checks.
pub fn check_segment_data_invariants<I: NciIndex>(
    segments_idx_begin: &[I],
    segments_mem_idx_begin: &[usize],
    values_len: usize,
) -> Result<(), NciArrayInvariant> {
    // All internal arrays must be empty or all non-empty.
    if segments_idx_begin.is_empty() == segments_mem_idx_begin.is_empty()
        && segments_mem_idx_begin.is_empty() == (values_len == 0)
    {
        if values_len == 0 {
            return Ok(());
        }
    } else {
        return Err(NciArrayInvariant::IsEmptyEquivalent);
    }

    // Segments must have corresponding representative elements in both.
    if segments_idx_begin.len() != segments_mem_idx_begin.len() {
        return Err(NciArrayInvariant::SegmentDataLengthEquivalent);
    }

    // First entries value must be at memory index `0`.
    // Note that the current `NciArray` implementation would still work if the first memory index were > 0 and < values_len,
    // however, this would result in some values being inaccessible.
    if segments_mem_idx_begin[0] != 0 {
        return Err(NciArrayInvariant::EntriesStartAtMemoryIndexZero);
    }
    // Test invariants for neighboring segment entries.
    for segment in 0..(segments_idx_begin.len() - 1) {
        // Memory indices must be in bounds.
        if segments_mem_idx_begin[segment + 1] >= values_len {
            return Err(NciArrayInvariant::MemoryIndicesInBounds);
        }

        // Segment elements must be strictly monotonically increasing.
        if (segments_idx_begin[segment] >= segments_idx_begin[segment + 1])
            || (segments_mem_idx_begin[segment] >= segments_mem_idx_begin[segment + 1])
        {
            return Err(NciArrayInvariant::SegmentDataElementsStrictlyMonotonic);
        }

        // It must be possible to generate indices for each element of the segment and the calculated distance must match.
        let idx_begin = segments_idx_begin[segment];
        let mem_idx_begin = segments_mem_idx_begin[segment];
        let mut idx = idx_begin;
        for mem_idx in (mem_idx_begin + 1)..segments_mem_idx_begin[segment + 1] {
            if let Some(next_idx) = idx.next() {
                // The indices generated within a segment must be between the representative indices.
                // An index outside of those bounds would result in some values being unused since the binary search would select a different segment.
                if next_idx >= segments_idx_begin[segment + 1] || next_idx < idx_begin {
                    return Err(NciArrayInvariant::SegmentsDisjoint);
                }
                if let Some(distance) = idx_begin.distance(next_idx)
                    && distance == (mem_idx - mem_idx_begin)
                {
                    idx = next_idx;
                } else {
                    return Err(NciArrayInvariant::NciIndexCalculatesCorrectDistanceForAllEntries);
                }
            } else {
                return Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries);
            }
        }

        // Two following segments must be separated (by at least one index).
        // Note that the current representation would still work if they are not separated,
        // however, this would indicate the construction is not as space efficient as it could be, since the segments could be merged into one.
        // For example, when if not checked, ([1, 2, 4]; [0, 1, 3]) would be a valid, even though it could be reduced to ([1]; [0]).
        if let Some(next_idx) = idx.next()
            && next_idx == segments_idx_begin[segment + 1]
        {
            return Err(NciArrayInvariant::SegmentDataMinimal);
        }
    }

    // Test invariants of last segment since it has no successor to check against.
    // For each entry, check if the generation of the next index is successful and the calculated distance match.
    let idx_begin = segments_idx_begin[segments_idx_begin.len() - 1];
    let mem_idx_begin = segments_mem_idx_begin[segments_idx_begin.len() - 1];
    let mut idx = idx_begin;
    for mem_idx in (mem_idx_begin + 1)..values_len {
        if let Some(next_idx) = idx.next() {
            // Same as above, but only with a lower bound since this is the last segment
            if next_idx < idx_begin {
                return Err(NciArrayInvariant::SegmentsDisjoint);
            }
            if let Some(distance) = idx_begin.distance(next_idx)
                && distance == (mem_idx - mem_idx_begin)
            {
                idx = next_idx;
            } else {
                return Err(NciArrayInvariant::NciIndexCalculatesCorrectDistanceForAllEntries);
            }
        } else {
            return Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries);
        }
    }
    Ok(())
}
