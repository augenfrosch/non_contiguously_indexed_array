pub struct NciArray<'a, I: NciIndex, V> {
    pub index_range_starting_indices: &'a [I],
    pub index_range_skip_amounts: &'a [I],
    pub values: &'a [V],
}

pub trait NciIndex:
    core::ops::Sub<Output = Self>
    + core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + Ord
    + PartialOrd
    + Sized
    + Clone
    + Copy
    + Default
{
    const ZERO: Self;
    const ONE: Self;

    fn as_usize(self) -> usize;
}

impl NciIndex for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline]
    fn as_usize(self) -> usize {
        self
    }
}

impl<I: NciIndex, V> std::ops::Index<I> for NciArray<'_, I, V> {
    type Output = V;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).unwrap()
    }
}

struct NciArrayIndexIter<'a, I: NciIndex> {
    index_range_starting_indices: &'a [I],
    index_range_skip_amounts: &'a [I],
    index: I,
    skipped: I,
    next_range_index: usize,
    next_index_range_starting_index: Option<&'a I>,
    next_index_range_skip_amount: Option<&'a I>,
    true_index: usize, // MAYBE no longer needed
    value_count: usize,
}

impl<'a, I: NciIndex> NciArrayIndexIter<'a, I> {
    fn new(
        index_range_starting_indices: &'a [I],
        index_range_skip_amounts: &'a [I],
        value_count: usize,
    ) -> Self {
        if let (Some(initial_index), Some(initial_skip_amount)) = (
            index_range_starting_indices.first(),
            index_range_skip_amounts.first(),
        ) {
            let next_index_range_starting_index = index_range_starting_indices.get(1);
            let next_index_range_skip_amount = index_range_skip_amounts.get(1);

            Self {
                index_range_starting_indices,
                index_range_skip_amounts,
                index: *initial_index,
                skipped: *initial_skip_amount,
                next_range_index: 1,
                next_index_range_starting_index,
                next_index_range_skip_amount,
                true_index: 0,
                value_count,
            }
        } else {
            Self {
                index_range_starting_indices,
                index_range_skip_amounts,
                index: I::default(),
                skipped: I::default(),
                next_range_index: usize::MAX,
                next_index_range_starting_index: None,
                next_index_range_skip_amount: None,
                true_index: usize::MAX,
                value_count,
            }
        }
    }
}

impl<I: NciIndex> Iterator for NciArrayIndexIter<'_, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.true_index < self.value_count {
            let value = self.index;

            self.index += I::ONE;
            if let (Some(next_index_range_starting_index), Some(next_index_range_skip_amount)) = (
                self.next_index_range_starting_index,
                self.next_index_range_skip_amount,
            ) && *next_index_range_starting_index - self.index
                <= *next_index_range_skip_amount - self.skipped
            {
                self.index = *next_index_range_starting_index;
                self.skipped = *next_index_range_skip_amount;

                self.next_range_index += 1;
                self.next_index_range_starting_index =
                    self.index_range_starting_indices.get(self.next_range_index);
                self.next_index_range_skip_amount =
                    self.index_range_skip_amounts.get(self.next_range_index);
            }

            self.true_index += 1;
            Some(value)
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
        NciArrayIndexIter::new(
            self.index_range_starting_indices,
            self.index_range_skip_amounts,
            self.values.len(),
        )
    }

    pub fn entries(&self) -> impl Iterator<Item = (I, &V)> {
        self.indices().zip(self.values())
    }

    pub fn has_entry(&self, index: I) -> bool {
        let range_index = match self.index_range_starting_indices.binary_search(&index) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };

        let index_range_starting_index = self
            .index_range_starting_indices
            .get(range_index)
            .copied()
            .unwrap_or_default();
        let index_range_skipped = self
            .index_range_skip_amounts
            .get(range_index)
            .copied()
            .unwrap_or_default();

        let true_starting_index = index_range_starting_index - index_range_skipped;

        let index_range_end =
            if let (Some(next_index_range_starting_index), Some(next_index_range_skip_amount)) = (
                self.index_range_starting_indices.get(range_index + 1),
                self.index_range_skip_amounts.get(range_index + 1),
            ) {
                (index_range_starting_index
                    + (*next_index_range_starting_index - *next_index_range_skip_amount)
                    - true_starting_index)
                    .as_usize()
            } else {
                index_range_starting_index.as_usize() + self.values.len()
                    - true_starting_index.as_usize()
            };

        index >= index_range_starting_index && index.as_usize() < index_range_end
    }

    pub fn get(&self, index: I) -> Option<&V> {
        let range_index = match self.index_range_starting_indices.binary_search(&index) {
            Ok(index) => index,
            Err(index) => index.checked_sub(1)?,
        };

        let index_range_starting_index = self
            .index_range_starting_indices
            .get(range_index)
            .copied()
            .unwrap_or_default();
        let index_range_skipped = self
            .index_range_skip_amounts
            .get(range_index)
            .copied()
            .unwrap_or_default();
        let slice_start = (index_range_starting_index - index_range_skipped).as_usize();
        let slice_end =
            if let (Some(next_index_range_starting_index), Some(next_index_range_skip_amount)) = (
                self.index_range_starting_indices.get(range_index + 1),
                self.index_range_skip_amounts.get(range_index + 1),
            ) {
                (*next_index_range_starting_index - *next_index_range_skip_amount).as_usize()
            } else {
                self.values.len()
            };
        let slice: &[V] = &self.values[slice_start..slice_end];
        slice.get((index - index_range_starting_index).as_usize())
    }
}
