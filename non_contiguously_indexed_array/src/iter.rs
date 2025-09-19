use crate::{NciArray, NciIndex};

pub struct NciArrayIndexIterData<'a, I> {
    current_idx: I,
    current_mem_idx: usize,
    remaining_idx_begin: &'a [I],
    remaining_mem_idx_begin: &'a [usize],
    mem_idx_end: core::num::NonZero<usize>,
}

pub enum NciArrayIndexIter<'a, I> {
    NonEmpty(NciArrayIndexIterData<'a, I>),
    Empty,
}

impl<'a, I: NciIndex> NciArrayIndexIter<'a, I> {
    pub fn new<V>(arr: &'a NciArray<'_, I, V>) -> Self {
        #[allow(clippy::option_if_let_else)] // Using map_or as suggested makes it unreadable
        if let Ok(mem_idx_end) = core::num::NonZero::try_from(arr.values.len()) {
            // This assert improves performance by having a single panic check instead of
            // having separate panic checks for each of the indexing operations below.
            assert!(!arr.segments_idx_begin.is_empty() && !arr.segments_mem_idx_begin.is_empty());
            NciArrayIndexIter::NonEmpty(NciArrayIndexIterData {
                current_idx: arr.segments_idx_begin[0],
                current_mem_idx: 0,
                remaining_idx_begin: &arr.segments_idx_begin[1..],
                remaining_mem_idx_begin: &arr.segments_mem_idx_begin[1..],
                mem_idx_end,
            })
        } else {
            NciArrayIndexIter::Empty
        }
    }
}

impl<I: NciIndex> Iterator for NciArrayIndexIter<'_, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NciArrayIndexIter::NonEmpty(iter_data) => {
                let result = iter_data.current_idx;
                let next_mem_idx = iter_data.current_mem_idx + 1;
                if next_mem_idx != iter_data.mem_idx_end.get() {
                    iter_data.current_mem_idx = next_mem_idx;
                    iter_data.current_idx = if let Some(next_segment_mem_idx) =
                        iter_data.remaining_mem_idx_begin.first()
                        && next_mem_idx == *next_segment_mem_idx
                        && !iter_data.remaining_idx_begin.is_empty()
                    {
                        // Jump to next segment
                        iter_data.remaining_mem_idx_begin.split_off_first();
                        *iter_data.remaining_idx_begin.split_off_first().unwrap()
                    } else {
                        // If the data structure was properly constructed, `result.next()` should never yield `None` here.
                        // Using `unwrap_or` here is a deliberate decision to avoid generating a panic handler
                        // and also ensures that the iterator always returns exactly `self.len()` elements.
                        result.next().unwrap_or(result)
                    };
                } else {
                    *self = NciArrayIndexIter::Empty;
                }
                Some(result)
            }
            NciArrayIndexIter::Empty => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

impl<I: NciIndex> ExactSizeIterator for NciArrayIndexIter<'_, I> {
    fn len(&self) -> usize {
        match self {
            NciArrayIndexIter::NonEmpty(iter_data) => {
                iter_data.mem_idx_end.get() - iter_data.current_mem_idx
            }
            NciArrayIndexIter::Empty => 0,
        }
    }
}
