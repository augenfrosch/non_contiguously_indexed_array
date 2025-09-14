use crate::NciIndex;

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

impl<'a, I, V> NciArray<'_, I, V> {
    pub const fn new() -> Self {
        Self {
            segments_idx_begin: &[],
            segments_mem_idx_begin: &[],
            values: &[],
        }
    }
}

impl<I: NciIndex, V> std::ops::Index<I> for NciArray<'_, I, V> {
    type Output = V;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).unwrap()
    }
}

struct NciArrayIndexIter<'a, I: NciIndex> {
    current_idx: Option<I>,
    current_mem_idx: usize,
    remaining_idx_begin: &'a [I],
    remaining_mem_idx_begin: &'a [usize],
    remaining_items: usize,
}

impl<I: NciIndex> Iterator for NciArrayIndexIter<'_, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_idx) = self.current_idx
            && self.remaining_items > 0
        {
            self.remaining_items -= 1;
            if let Some(next_mem_idx) = self.remaining_mem_idx_begin.first()
                && self.current_mem_idx + 1 == *next_mem_idx
            {
                self.current_idx = self.remaining_idx_begin.split_off_first().copied();
                self.current_mem_idx = *self.remaining_mem_idx_begin.split_off_first().unwrap();
            } else {
                self.current_idx = current_idx.next();
                self.current_mem_idx += 1;
            }
            Some(current_idx)
        } else {
            None
        }
    }
}

impl<I: NciIndex, V> NciArray<'_, I, V> {
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }

    #[must_use]
    pub fn indices(&self) -> impl Iterator<Item = I> {
        let idx_split = self.segments_idx_begin.split_first();
        let mem_idx_split = self.segments_mem_idx_begin.split_first();
        NciArrayIndexIter {
            current_idx: idx_split.map(|split| *split.0),
            current_mem_idx: mem_idx_split.map(|split| *split.0).unwrap_or_default(),
            remaining_idx_begin: idx_split.map(|split| split.1).unwrap_or_default(),
            remaining_mem_idx_begin: mem_idx_split.map(|split| split.1).unwrap_or_default(),
            remaining_items: self.values.len(),
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = (I, &V)> {
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
