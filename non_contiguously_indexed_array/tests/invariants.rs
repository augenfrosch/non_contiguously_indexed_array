mod constants;
use constants::*;

use non_contiguously_indexed_array::{NciArrayInvariant, NciIndex, check_segment_data_invariants};

#[test]
fn constants_invariants_test() {
    assert!(ARRAY_1.fulfills_invariants());
    assert!(ARRAY_2.fulfills_invariants());
    assert!(ARRAY_3.fulfills_invariants());
    assert!(ARRAY_4.fulfills_invariants());
    assert!(ARRAY_5.fulfills_invariants());
}

#[test]
fn is_empty_equivalent_invariant_test() {
    assert!(check_segment_data_invariants::<i32>(&[], &[], 0).is_ok());
    assert!(check_segment_data_invariants(&[0], &[0], 1).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0], &[], 0),
        Err(NciArrayInvariant::IsEmptyEquivalent)
    );
    assert_eq!(
        check_segment_data_invariants::<i32>(&[], &[0], 0),
        Err(NciArrayInvariant::IsEmptyEquivalent)
    );
    assert_eq!(
        check_segment_data_invariants(&[0], &[0], 0),
        Err(NciArrayInvariant::IsEmptyEquivalent)
    );
    assert_eq!(
        check_segment_data_invariants::<i32>(&[], &[], 1),
        Err(NciArrayInvariant::IsEmptyEquivalent)
    );
    assert_eq!(
        check_segment_data_invariants(&[0], &[], 1),
        Err(NciArrayInvariant::IsEmptyEquivalent)
    );
}

#[test]
fn segment_data_length_equivalent_invariant_test() {
    assert!(check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2], 3).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5, 7], &[0, 1, 2], 3),
        Err(NciArrayInvariant::SegmentDataLengthEquivalent)
    );
    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2, 3], 3),
        Err(NciArrayInvariant::SegmentDataLengthEquivalent)
    );
}

#[test]
fn entries_start_at_zero_invariant_test() {
    assert!(check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2], 3).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5], &[1, 2, 3], 3),
        Err(NciArrayInvariant::EntriesStartAtMemoryIndexZero)
    );
}

#[test]
fn memory_indices_in_bounds_invariant_test() {
    assert!(check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2], 3).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2], 2),
        Err(NciArrayInvariant::MemoryIndicesInBounds)
    );
    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5, 7], &[0, 1, 2, 3], 3),
        Err(NciArrayInvariant::MemoryIndicesInBounds)
    );

    assert!(check_segment_data_invariants(&[0, 2, 4], &[0, 1, 2], 3).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0, 2, 4], &[0, 1, 3], 3),
        Err(NciArrayInvariant::MemoryIndicesInBounds)
    );
    assert_eq!(
        check_segment_data_invariants(&[0, 2, 4], &[0, 1, 5], 3),
        Err(NciArrayInvariant::MemoryIndicesInBounds)
    );
}

#[test]
fn non_strictly_monotonic_invariants_test() {
    assert!(check_segment_data_invariants(&[0, 3, 5], &[0, 1, 2], 3).is_ok());

    assert_eq!(
        check_segment_data_invariants(&[0, 5, 3], &[0, 1, 2], 3),
        Err(NciArrayInvariant::SegmentDataElementsStrictlyMonotonic)
    );
    assert_eq!(
        check_segment_data_invariants(&[0, 3, 5], &[0, 2, 1], 3),
        Err(NciArrayInvariant::SegmentDataElementsStrictlyMonotonic)
    );
}

#[test]
fn segments_disjoint() {
    // Checks affect only "inner" segments, i.e. segments with a following segment
    assert!(check_segment_data_invariants(&[0, 3], &[0, 2], 3).is_ok());
    assert_eq!(
        check_segment_data_invariants(&[0, 2], &[0, 2], 3),
        Err(NciArrayInvariant::SegmentDataMinimal)
    );
    assert_eq!(
        check_segment_data_invariants(&[0, 1], &[0, 2], 3),
        Err(NciArrayInvariant::SegmentsDisjoint)
    );
}

#[test]
fn segment_data_minimal_invariant_test() {
    assert_eq!(
        check_segment_data_invariants(&[1, 2, 4], &[0, 1, 3], 4),
        Err(NciArrayInvariant::SegmentDataMinimal)
    );
    assert!(check_segment_data_invariants(&[1], &[0], 4).is_ok());
}

#[test]
fn nci_index_can_generate_indices_invariant_test() {
    // Final segment
    assert!(check_segment_data_invariants(&[i32::MAX], &[0], 1).is_ok());
    assert_eq!(
        check_segment_data_invariants(&[i32::MAX], &[0], 2),
        Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries)
    );
    assert_eq!(
        check_segment_data_invariants(&[u8::MIN], &[0], usize::from(u8::MAX) + 2),
        Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries)
    );
}

// The following tests are all for the `NciIndex` related invariants using custom implementations,
// since `NciArray`s using the implementation for the primitive integers cannot (at least should not) violate them.

#[test]
fn custom_nci_index_next_segments_disjoint_test() {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct DownwardSpikeIndex(i32);
    impl NciIndex for DownwardSpikeIndex {
        fn next(self) -> Option<Self> {
            let i = self.0.checked_add(1)?;
            if (i % 10) > 5 {
                Some(Self(i - 100))
            } else {
                Some(Self(i))
            }
        }

        // Using a wildly inefficient implementation to force the distance check to not fail
        // In real implementations this would probably not be the case.
        fn distance(self, other: Self) -> Option<usize> {
            let mut distance = 0;
            let mut idx = self;
            while idx != other {
                idx = idx.next()?;
                distance += 1;
            }
            Some(distance)
        }
    }

    assert!(check_segment_data_invariants(&[DownwardSpikeIndex(0)], &[0], 6).is_ok());
    assert_eq!(
        check_segment_data_invariants(&[DownwardSpikeIndex(0)], &[0], 7),
        Err(NciArrayInvariant::SegmentsDisjoint)
    );

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct UpwardSpikeIndex(i32);
    impl NciIndex for UpwardSpikeIndex {
        fn next(self) -> Option<Self> {
            let i = self.0.checked_add(1)?;
            if (i % 10) > 5 {
                Some(Self(i + 100))
            } else {
                Some(Self(i))
            }
        }

        // Same as above
        fn distance(self, other: Self) -> Option<usize> {
            let mut distance = 0;
            let mut idx = self;
            while idx != other {
                idx = idx.next()?;
                distance += 1;
            }
            Some(distance)
        }
    }

    assert!(check_segment_data_invariants(&[UpwardSpikeIndex(0)], &[0], 10).is_ok());
    assert_eq!(
        check_segment_data_invariants(&[UpwardSpikeIndex(0), UpwardSpikeIndex(9)], &[0, 7], 8),
        Err(NciArrayInvariant::SegmentsDisjoint)
    );

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct ZigZagIndex(u32);
    impl NciIndex for ZigZagIndex {
        // Goes 0 2 1 3 5 4 6 8 7 ...
        fn next(self) -> Option<Self> {
            if (self.0 % 3) == 2 {
                self.0.checked_sub(1).map(Self)
            } else {
                self.0.checked_add(2).map(Self)
            }
        }

        // Same as above
        fn distance(self, other: Self) -> Option<usize> {
            let mut distance = 0;
            let mut idx = self;
            while idx != other {
                idx = idx.next()?;
                distance += 1;
            }
            Some(distance)
        }
    }

    // The checks currently don't test if the index returned by `NciIndex::next` is strictly monotonically increasing,
    // which the documentation of `NciIndex` curremtly states to be a requirement.
    // This might not be entirely correct as the `ZigZagIndex` technically doesn't violate the current invariants
    // if the segments remain disjoint as shown by the asserts below
    // TODO: Determine if this acceptable, and if yes, fix `NciIndex` documentation, if not, add checks / invariant.
    assert!(
        check_segment_data_invariants(&[ZigZagIndex(0), ZigZagIndex(10)], &[0, 10], 21).is_ok()
    );
    assert_eq!(
        check_segment_data_invariants(&[UpwardSpikeIndex(0), UpwardSpikeIndex(1)], &[0, 2], 3),
        Err(NciArrayInvariant::SegmentsDisjoint)
    );
}

#[test]
fn custom_nci_index_next_invariant_test() {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct ShortChainIndex(u32);
    impl NciIndex for ShortChainIndex {
        fn next(self) -> Option<Self> {
            let i = self.0.checked_add(1)?;
            if (i % 10) > 5 { None } else { Some(Self(i)) }
        }

        fn distance(self, other: Self) -> Option<usize> {
            self.0.abs_diff(other.0).try_into().ok()
        }
    }

    assert!(check_segment_data_invariants(&[ShortChainIndex(0)], &[0], 6).is_ok());
    assert_eq!(
        check_segment_data_invariants(&[ShortChainIndex(0)], &[0], 7),
        Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries)
    );
    assert!(
        check_segment_data_invariants(&[ShortChainIndex(0), ShortChainIndex(9)], &[0, 6], 12)
            .is_ok()
    );
    assert_eq!(
        check_segment_data_invariants(&[ShortChainIndex(0), ShortChainIndex(9)], &[0, 6], 14),
        Err(NciArrayInvariant::NciIndexCanGenerateIndicesForAllEntries)
    );
}

#[test]
fn custom_nci_index_distance_invariant_test() {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct OverestimatingIndex(u32);
    impl NciIndex for OverestimatingIndex {
        fn next(self) -> Option<Self> {
            self.0.checked_add(1).map(Self)
        }

        fn distance(self, other: Self) -> Option<usize> {
            (self.0.abs_diff(other.0) + 1).try_into().ok()
        }
    }

    assert!(
        check_segment_data_invariants(
            &[OverestimatingIndex(0), OverestimatingIndex(2)],
            &[0, 1],
            2
        )
        .is_ok()
    );
    assert_eq!(
        check_segment_data_invariants(
            &[OverestimatingIndex(0), OverestimatingIndex(3)],
            &[0, 2],
            3
        ),
        Err(NciArrayInvariant::NciIndexCalculatesCorrectDistanceForAllEntries)
    );

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct ExtremelyOverestimatingIndex(u32);
    impl NciIndex for ExtremelyOverestimatingIndex {
        fn next(self) -> Option<Self> {
            self.0.checked_add(1).map(Self)
        }

        fn distance(self, _other: Self) -> Option<usize> {
            None
        }
    }

    assert!(
        check_segment_data_invariants(
            &[
                ExtremelyOverestimatingIndex(0),
                ExtremelyOverestimatingIndex(2)
            ],
            &[0, 1],
            2
        )
        .is_ok()
    );
    assert_eq!(
        check_segment_data_invariants(
            &[
                ExtremelyOverestimatingIndex(0),
                ExtremelyOverestimatingIndex(3)
            ],
            &[0, 2],
            3
        ),
        Err(NciArrayInvariant::NciIndexCalculatesCorrectDistanceForAllEntries)
    );

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct UnderestimatingIndex(u32);
    impl NciIndex for UnderestimatingIndex {
        fn next(self) -> Option<Self> {
            self.0.checked_add(1).map(Self)
        }

        fn distance(self, other: Self) -> Option<usize> {
            (self.0.abs_diff(other.0).saturating_sub(1)).try_into().ok()
        }
    }

    assert!(
        check_segment_data_invariants(
            &[UnderestimatingIndex(0), UnderestimatingIndex(2)],
            &[0, 1],
            2
        )
        .is_ok()
    );
    assert_eq!(
        check_segment_data_invariants(
            &[UnderestimatingIndex(0), UnderestimatingIndex(3)],
            &[0, 2],
            3
        ),
        Err(NciArrayInvariant::NciIndexCalculatesCorrectDistanceForAllEntries)
    );
}
