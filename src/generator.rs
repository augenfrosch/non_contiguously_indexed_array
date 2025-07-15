#[derive(Debug)]
pub struct NciArrayDataGenerator<V> {
    entries_ordered_monotonically_increasing: bool,
    last_added_entry_index: Option<usize>,
    // values of the format (start_index, skipped_since_last); the skip amounts are relative
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
