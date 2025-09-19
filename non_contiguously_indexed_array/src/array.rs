use crate::{NciArrayIndexIter, NciIndex};

#[derive(Debug, Clone, Copy, Default)]
pub struct NciArray<'a, I, V> {
    /// The user-defined index of the first element of each segment.
    /// Example: `segments_idx_begin[2] == 5` means the first element of the third segment has user-defined index 5.
    pub segments_idx_begin: &'a [I],

    /// The memory index of the first element of each segment.
    /// Example: `segments_mem_idx_begin[2] = 3` means the first element of the third segment is stored in memory index 3.
    pub segments_mem_idx_begin: &'a [usize],

    /// All the values stored in this array.
    pub values: &'a [V],
}

impl<I, V> NciArray<'_, I, V> {
    pub const fn new() -> Self {
        Self {
            segments_idx_begin: &[],
            segments_mem_idx_begin: &[],
            values: &[],
        }
    }
}

impl<I: NciIndex, V> core::ops::Index<I> for NciArray<'_, I, V> {
    type Output = V;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<I: NciIndex, V> NciArray<'_, I, V> {
    pub fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.values.iter()
    }

    pub fn indices(&self) -> impl ExactSizeIterator<Item = I> {
        NciArrayIndexIter::new(self)
    }

    pub fn entries(&self) -> impl ExactSizeIterator<Item = (I, &V)> {
        self.indices().zip(self.values())
    }

    pub fn has_entry(&self, index: I) -> bool {
        self.find_candidate_segment(index).is_some_and(|segment| {
            let distance = self.segments_idx_begin[segment].distance(index);
            distance.is_some_and(|distance| distance < self.segment_len(segment))
        })
    }

    pub fn get(&self, index: I) -> Option<&V> {
        if let Some(segment) = self.find_candidate_segment(index) {
            let distance = self.segments_idx_begin[segment].distance(index)?;
            if distance >= self.segment_len(segment) {
                return None;
            }
            let element_mem_idx = self.segments_mem_idx_begin[segment] + distance;
            Some(&self.values[element_mem_idx])
        } else {
            None
        }
    }

    /// Returns the segment that potentially contains the given index.
    fn find_candidate_segment(&self, index: I) -> Option<usize> {
        let candidate_segment_plus_one = self
            .segments_idx_begin
            .partition_point(|segment_idx_begin| index.ge(segment_idx_begin));
        candidate_segment_plus_one.checked_sub(1)
    }

    /// Returns the length of the `i`-th segment.
    /// Panics in case there are fewer than `i + 1` segments.
    fn segment_len(&self, segment: usize) -> usize {
        let mem_idx_begin = self.segments_mem_idx_begin[segment];
        let mem_idx_end = *self
            .segments_mem_idx_begin
            .get(segment + 1)
            .unwrap_or(&self.values.len());
        mem_idx_end - mem_idx_begin
    }
}
