#[derive(Debug)]
pub struct NciArrayGenerator<V> {
    entries_ordered_monotonically_increasing: bool,
    last_added_entry_index: Option<usize>,
    // values of the format (start_index, skipped_since_last); the skip amounts are relative
    index_ranges: Vec<(usize, usize)>,
    entries: Vec<(usize, V)>,
}

pub enum OutputFormat {
    RustCodegen,
    RON,
    RONPretty,
}

pub enum ValueFormatting {
    Display,
    Debug,
}

pub struct BuildConfiguration {
    pub output_format: OutputFormat,
    pub value_formatting: ValueFormatting,
}

impl<V: std::fmt::Display + std::fmt::Debug> NciArrayGenerator<V> {
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

    pub fn build_type(&mut self, value_type_str: &str) -> String {
        self.ensure_output_preconditions();

        let index_range_count = self.index_ranges.len();
        let value_count = self.entries.len();

        format!("NciArray<{value_type_str}, {index_range_count}, {value_count}>")
    }

    pub fn build(&mut self, build_config: BuildConfiguration) -> String {
        self.ensure_output_preconditions();

        let mut output_string = "{\n".to_string();

        output_string.push_str("\tindex_range_starting_indices: [\n");
        for (starting_index, _) in &self.index_ranges {
            output_string.push_str(&format!(
                "\t\t{:?},\n",
                *starting_index
            ));
        }
        output_string.push_str("\t],\n");

        output_string.push_str("\tindex_range_skip_amounts: [\n");
        let mut total_skip_amount = 0;
        for (_, skip_amount) in &self.index_ranges {
            total_skip_amount += skip_amount;
            output_string.push_str(&format!(
                "\t\t{:?},\n",
                total_skip_amount
            ));
        }
        output_string.push_str("\t],\n");

        output_string.push_str("\tvalues: [\n");
        for (_, value) in &self.entries {
            let entry_str = match build_config.value_formatting {
                ValueFormatting::Display => &format!(
                                "\t\t{},\n",
                                *value
                            ),
                ValueFormatting::Debug => &format!(
                                "\t\t{:?},\n",
                                *value
                            ),
            };
            output_string.push_str(entry_str);
        }
        output_string.push_str("\t],\n");
        output_string.push_str("}");

        match build_config.output_format {
            OutputFormat::RustCodegen => output_string,
            OutputFormat::RON => output_string.replace(['\t', '\n', ' '], "")
                .replace(['{', '['], "(").replace(['}', ']'], ")"),
            OutputFormat::RONPretty => output_string.replace(['{', '['], "(").replace(['}', ']'], ")"),
        }
    }

}

pub fn built_rust_codegen_to(rust_codegen: &str, output_format: OutputFormat, additional_filters: Option<&[&str]>) -> String {
    let mut output_string = rust_codegen.to_string();
    if let Some(additional_filters) = additional_filters {
        for filter in additional_filters {
            output_string = output_string.replace(filter, "");
        }
    }
    match output_format {
        OutputFormat::RustCodegen => output_string,
        OutputFormat::RON => output_string.replace(['\t', '\n', ' '], "")
            .replace(['{', '['], "(").replace(['}', ']'], ")"),
        OutputFormat::RONPretty => output_string.replace(['{', '['], "(").replace(['}', ']'], ")"),
    }
}
