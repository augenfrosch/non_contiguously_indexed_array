mod generator;
pub use generator::*;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "V: serde::Serialize + serde::de::DeserializeOwned")
)]
pub struct NciArrayData<V, const R: usize, const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub index_range_starting_indices: [usize; R],
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub index_range_skip_amounts: [usize; R],
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub values: [V; N],
}

#[derive(Debug, PartialEq)]
pub struct NciArray<'a, V> {
    index_range_starting_indices: &'a [usize],
    index_range_skip_amounts: &'a [usize],
    values: &'a [V],
    // finger: usize, // TODO test if exponential search would make sense
}

impl<'a, V> NciArray<'a, V> {
    pub const fn new(
        index_range_starting_indices: &'a [usize],
        index_range_skip_amounts: &'a [usize],
        values: &'a [V],
    ) -> Self {
        Self {
            index_range_starting_indices,
            index_range_skip_amounts,
            values,
            // finger: 0,
        }
    }
}

impl<'a, V> std::ops::Index<usize> for NciArray<'a, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct NciArrayIndexIter<'a> {
    index_range_starting_indices: &'a [usize],
    index_range_skip_amounts: &'a [usize],
    index: usize,
    skipped: usize,
    next_range_index: usize,
    next_index_range_starting_index: Option<&'a usize>,
    next_index_range_skip_amount: Option<&'a usize>,
    true_index: usize, // MAYBE no longer needed
    value_count: usize,
}

impl<'a> NciArrayIndexIter<'a> {
    fn new(
        index_range_starting_indices: &'a [usize],
        index_range_skip_amounts: &'a [usize],
        value_count: usize,
    ) -> Self {
        let index = index_range_starting_indices[0];
        let skipped = index_range_skip_amounts[0];
        let next_index_range_starting_index = index_range_starting_indices.get(1);
        let next_index_range_skip_amount = index_range_skip_amounts.get(1);

        Self {
            index_range_starting_indices,
            index_range_skip_amounts,
            index,
            skipped,
            next_range_index: 1,
            next_index_range_starting_index,
            next_index_range_skip_amount,
            true_index: 0,
            value_count,
        }
    }
}

impl<'a> Iterator for NciArrayIndexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.true_index < self.value_count {
            let value = self.index;

            self.index += 1;
            if let (Some(next_index_range_starting_index), Some(next_index_range_skip_amount)) = (
                self.next_index_range_starting_index,
                self.next_index_range_skip_amount,
            ) {
                if next_index_range_starting_index - self.index
                    <= next_index_range_skip_amount - self.skipped
                {
                    self.index = *next_index_range_starting_index;
                    self.skipped = *next_index_range_skip_amount;

                    self.next_range_index += 1;
                    self.next_index_range_starting_index =
                        self.index_range_starting_indices.get(self.next_range_index);
                    self.next_index_range_skip_amount =
                        self.index_range_skip_amounts.get(self.next_range_index);
                }
            }

            self.true_index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a, V> NciArray<'a, V> {
    pub fn values(&self) -> core::slice::Iter<'a, V> {
        self.values.iter()
    }

    pub fn indices(&self) -> NciArrayIndexIter {
        NciArrayIndexIter::new(
            self.index_range_starting_indices,
            self.index_range_skip_amounts,
            self.values.len(),
        )
    }

    pub fn entries(&self) -> std::iter::Zip<NciArrayIndexIter, core::slice::Iter<'a, V>> {
        self.indices().zip(self.values())
    }

    pub fn get(&self, index: usize) -> Option<&V> {
        let range_index = match self.index_range_starting_indices.binary_search(&index) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };

        let index_range_starting_index = self
            .index_range_starting_indices
            .get(range_index)
            .cloned()
            .unwrap_or_default();
        let index_range_skipped = self
            .index_range_skip_amounts
            .get(range_index)
            .cloned()
            .unwrap_or_default();
        let slice_start = index_range_starting_index - index_range_skipped;
        let slice_end =
            if let (Some(next_index_range_starting_index), Some(next_index_range_skip_amount)) = (
                self.index_range_starting_indices.get(range_index + 1),
                self.index_range_skip_amounts.get(range_index + 1),
            ) {
                next_index_range_starting_index - next_index_range_skip_amount
            } else {
                self.values.len()
            };
        let slice: &[V] = &self.values[slice_start..slice_end];
        if index >= index_range_starting_index {
            slice.get(index - index_range_starting_index)
        } else {
            None // TODO look into finding a better way of checking this
        }
    }
}
