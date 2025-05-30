#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "V: serde::Serialize + serde::de::DeserializeOwned")
)]
pub struct NciArrayData<V, const R: usize, const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub index_range_starting_indices: [usize; R],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub index_range_skip_amounts: [usize; R],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
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

    pub fn build(
        &mut self,
        prefix: &str,
        suffix: &str,
        use_debug_format: bool,
    ) -> impl std::fmt::Display {
        if !self.entries_ordered_monotonically_increasing {
            self.entries
                .sort_by(|(first_index, _), (second_index, _)| first_index.cmp(second_index));

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
                        #[cfg(not(feature = "panic"))]
                        continue; // skip duplicate entries for the same index
                    }
                } else {
                    self.index_ranges.push((*index, *index));
                }

                self.last_added_entry_index = Some(*index);
            }

            self.entries_ordered_monotonically_increasing = true;
        }

        let mut main_output = "{\n".to_string();

        main_output.push_str("\tindex_range_starting_indices: [\n");
        for (starting_index, _) in &self.index_ranges {
            main_output.push_str(&format!("\t\t{:?},\n", *starting_index));
        }
        main_output.push_str(&format!("\t],\n"));

        main_output.push_str("\tindex_range_skip_amounts: [\n");
        let mut total_skip_amount = 0;
        for (_, skip_amount) in &self.index_ranges {
            total_skip_amount += skip_amount;
            main_output.push_str(&format!("\t\t{:?},\n", total_skip_amount));
        }
        main_output.push_str(&format!("\t],\n"));

        main_output.push_str("\tvalues: [\n");
        self.last_added_entry_index = None;
        let mut entry_count = 0usize;
        for (index, value) in &self.entries {
            if let Some(last_added_entry_index) = self.last_added_entry_index {
                if *index == last_added_entry_index {
                    #[cfg(feature = "panic")]
                    panic!("Duplicate index `{}`", *index);
                    #[cfg(not(feature = "panic"))]
                    continue; // skip duplicate entries for the same index
                }
            }

            self.last_added_entry_index = Some(*index);
            entry_count += 1;
            let entry_str = if !use_debug_format {
                &format!("\t\t{},\n", *value)
            } else {
                &format!("\t\t{:?},\n", *value)
            };
            main_output.push_str(entry_str);
        }
        main_output.push_str(&format!("\t],\n"));
        main_output.push_str("}");

        format!(
            "{}{}{}",
            prefix
                .replacen("{R}", &self.index_ranges.len().to_string(), 1)
                .replacen("{N}", &entry_count.to_string(), 1),
            main_output,
            suffix.replacen(
                "{comment}",
                &format!(
                    "// Generated with R={}, N={}",
                    self.index_ranges.len(),
                    entry_count
                ),
                1
            ),
        )
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
