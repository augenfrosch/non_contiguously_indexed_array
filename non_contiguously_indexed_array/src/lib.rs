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
    NciIndexCanGenerateIndicesForAllEntries,
    SegmentDataMinimal, // could also be called something like `SegmentsSeparated`
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

        // Distance to the next representative index must be greater than the number of entries of the segment.
        // Note that the current representation would still work if the distance is equals to the number of entries,
        // however, this would indicate the construction is not as space efficient as it could be, since the segments could be merged into one.
        // Testing for it to be strictly greater restricts the representation to be unique.
        // For example, when accepting equals, ([1, 2, 4]; [0, 1, 3]) would be a valid, even though it could be reduced to ([1]; [0]).
        // TODO: This currently assumes that NciArray is implemented correctly. Determine if this is acceptable or replace with for loop similar to below + strict monotonicity check (which would then also be needed below)
        match segments_idx_begin[segment]
            .distance(segments_idx_begin[segment + 1])
            .unwrap_or(usize::MAX)
            .cmp(&(segments_mem_idx_begin[segment + 1] - segments_mem_idx_begin[segment]))
        {
            core::cmp::Ordering::Less => {
                return Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries);
            }
            core::cmp::Ordering::Equal => return Err(NciArrayInvariant::SegmentDataMinimal),
            core::cmp::Ordering::Greater => {} // fulfills both invariants
        }
    }

    // Test invariants of last segment since it has no successor to check against.
    // For each entry, check if the generation of the next index is successful.
    // The check is skipped for the final entry since no further index has to be generated.
    let mut idx = segments_idx_begin[segments_idx_begin.len() - 1];
    for _mem_idx in segments_mem_idx_begin[segments_mem_idx_begin.len() - 1]..(values_len - 1) {
        if let Some(next_idx) = idx.next() {
            idx = next_idx;
        } else {
            return Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries);
        }
    }
    Ok(())
}
