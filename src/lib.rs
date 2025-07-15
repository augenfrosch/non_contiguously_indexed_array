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

#[derive(Debug)]
pub struct NciArrayDataGenerator<V> {
    entries_ordered_monotonically_increasing: bool,
    last_added_entry_index: Option<usize>,
    // values of the format (start_index, skipped_since_last); the skip amounts are relative (as opposed to those of NciArray[Data]) to potentially allow for inserting entries out of order without requiring a full sweep for `build` in the future
    index_ranges: Vec<(usize, usize)>,
    entries: Vec<(usize, V)>,
}

impl<V: std::fmt::Display + std::fmt::Debug> NciArrayDataGenerator<V> {
    pub fn new() -> Self {
        Self {
            entries_ordered_monotonically_increasing: true,
            last_added_entry_index: None,
            index_ranges: vec![],
            entries: vec![],
        }
    }

    pub fn entry(&mut self, index: usize, value: V) {
        if self.entries_ordered_monotonically_increasing {
            if let Some(last_added_entry_index) = self.last_added_entry_index {
                if index > last_added_entry_index {
                    let index_difference = index - last_added_entry_index;
                    if index_difference != 1 {
                        self.index_ranges.push((index, index_difference - 1));
                    }
                } else {
                    #[cfg(feature = "panic")]
                    if index == last_added_entry_index {
                        panic!("Duplicate index `{}`", index);
                    }
                    self.entries_ordered_monotonically_increasing = false;
                }
            } else {
                self.index_ranges.push((index, index));
            }

            self.last_added_entry_index = Some(index);
            if !self.entries_ordered_monotonically_increasing {
                self.last_added_entry_index = None;
                self.index_ranges = vec![];
            }
        }

        self.entries.push((index, value));
    }

    fn ensure_output_preconditions(&mut self) {
        if !self.entries_ordered_monotonically_increasing {
            self.entries
                .sort_by(|(first_index, _), (second_index, _)| first_index.cmp(second_index));
            #[cfg(not(feature = "panic"))]
            {
                // filter duplicate entries, first entry with index is retained
                let mut expected_index = self.entries.first().unwrap().0;
                self.entries.retain(|(entry_index, _)| {
                    let index_as_expected = *entry_index == expected_index;
                    if index_as_expected {
                        expected_index = expected_index + 1;
                    }
                    index_as_expected
                });
            }

            for (index, _) in &self.entries {
                if let Some(last_added_entry_index) = self.last_added_entry_index {
                    if *index > last_added_entry_index {
                        let index_difference = index - last_added_entry_index;
                        if index_difference != 1 {
                            self.index_ranges.push((*index, index_difference - 1));
                        }
                    } else {
                        #[cfg(feature = "panic")]
                        panic!("Duplicate index `{}`", *index);
                    }
                } else {
                    self.index_ranges.push((*index, *index));
                }

                self.last_added_entry_index = Some(*index);
            }

            self.entries_ordered_monotonically_increasing = true;
        }
    }

    pub fn build_type(&mut self, value_type_str: &str) -> impl std::fmt::Display + use<V> {
        self.ensure_output_preconditions();

        let index_range_count = self.index_ranges.len();
        let value_count = self.entries.len();

        format!("NciArrayData<{value_type_str}, {index_range_count}, {value_count}>")
    }

    pub fn build(&mut self, use_debug_format: bool) -> impl std::fmt::Display + use<V> {
        self.ensure_output_preconditions();

        let mut output_string = "{\n".to_string();

        output_string.push_str("\tindex_range_starting_indices: [\n");
        for (starting_index, _) in &self.index_ranges {
            output_string.push_str(&format!("\t\t{:?},\n", *starting_index));
        }
        output_string.push_str(&format!("\t],\n"));

        output_string.push_str("\tindex_range_skip_amounts: [\n");
        let mut total_skip_amount = 0;
        for (_, skip_amount) in &self.index_ranges {
            total_skip_amount += skip_amount;
            output_string.push_str(&format!("\t\t{:?},\n", total_skip_amount));
        }
        output_string.push_str(&format!("\t],\n"));

        output_string.push_str("\tvalues: [\n");
        for (_, value) in &self.entries {
            let entry_str = if !use_debug_format {
                &format!("\t\t{},\n", *value)
            } else {
                &format!("\t\t{:?},\n", *value)
            };
            output_string.push_str(entry_str);
        }
        output_string.push_str(&format!("\t],\n"));
        output_string.push_str("}");
        output_string
    }
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
