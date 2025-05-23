#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "T: serde::Serialize + serde::de::DeserializeOwned")
)]
pub struct NciBaseArray<T, const R: usize, const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub index_ranges: [(usize, usize); R],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub data: [T; N],
}

#[derive(Debug)]
pub struct NciBaseArrayGenerator<T> {
    entries_ordered_monotonically_increasing: bool,
    last_added_entry_index: Option<usize>,
    index_ranges: Vec<(usize, usize)>,
    entries: Vec<(usize, T)>,
}

impl<T: std::fmt::Display> NciBaseArrayGenerator<T> {
    pub fn new() -> Self {
        Self {
            entries_ordered_monotonically_increasing: true,
            last_added_entry_index: None,
            index_ranges: vec![],
            entries: vec![],
        }
    }

    pub fn entry(&mut self, index: usize, value: T) {
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
            } else if index > 0 {
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

    pub fn build(&mut self, prefix: &str, suffix: &str) -> impl std::fmt::Display {
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
                } else if index > &0 {
                    self.index_ranges.push((*index, *index));
                }

                self.last_added_entry_index = Some(*index);
            }

            self.entries_ordered_monotonically_increasing = true;
        }

        let mut main_output = "{\n".to_string();

        main_output.push_str("\tindex_ranges: [\n");
        for index_range in &self.index_ranges {
            main_output.push_str(&format!("\t\t{:?},\n", *index_range));
        }
        main_output.push_str(&format!("\t],\n"));

        main_output.push_str("\tdata: [\n");
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
            main_output.push_str(&format!("\t\t{:},\n", *value));
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
pub struct NciArray<'a, T> {
    index_ranges: &'a [(usize, usize)],
    data: &'a [T],
}

impl<'a, T> NciArray<'a, T> {
    pub fn new(index_ranges: &'a [(usize, usize)], data: &'a [T]) -> Self {
        Self { index_ranges, data }
    }
}

impl<'a, T> std::ops::Index<usize> for NciArray<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct NciIndexIter<'a> {
    index_ranges: &'a [(usize, usize)],
    next_index_range: usize,
    true_index: usize,
    data_len: usize,
    index: usize,
}

impl<'a> NciIndexIter<'a> {
    fn new(index_ranges: &'a [(usize, usize)], data_len: usize) -> Self {
        let (initial_index, initial_next_index_range) = index_ranges
            .get(0)
            .map(|(range_start, skipped)| {
                if range_start - skipped == 0 {
                    (*range_start, 1)
                } else {
                    (0, 0)
                }
            })
            .unwrap_or_default();
        Self {
            index_ranges,
            next_index_range: initial_next_index_range,
            true_index: 0,
            data_len,
            index: initial_index,
        }
    }
}

impl<'a> Iterator for NciIndexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.true_index < self.data_len {
            let value = self.index;

            self.index += 1;
            if let Some((range_start, skipped)) = self.index_ranges.get(self.next_index_range) {
                if range_start - skipped <= self.index {
                    self.index = *range_start;
                    self.next_index_range += 1;
                }
            }

            self.true_index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a, T> NciArray<'a, T> {
    pub fn data(&self) -> core::slice::Iter<'a, T> {
        self.data.iter()
    }

    pub fn indices(&self) -> NciIndexIter {
        NciIndexIter::new(self.index_ranges, self.data.len())
    }

    pub fn entries(&self) -> std::iter::Zip<NciIndexIter, core::slice::Iter<'a, T>> {
        self.indices().zip(self.data())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut index_offset = 0;
        let mut index_range_start = 0;
        let mut index_range_end = None;
        for (range_start, skipped) in self.index_ranges {
            if range_start > &index {
                index_range_end = Some(range_start - skipped);
                break;
            } else {
                index_range_start = *range_start;
                index_offset += skipped;
            }
        }
        let slice_start = index_range_start - index_offset;
        let slice_end = index_range_end
            .map(|index_range_end| index_range_end - index_offset)
            .unwrap_or(self.data.len());
        let slice: &[T] = &self.data[slice_start..slice_end];
        slice.get(index - index_range_start)
    }
}
