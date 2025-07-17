#[derive(Debug)]
pub struct NciArrayDataGenerator<V> {
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

    pub fn build(&mut self, build_config: BuildConfiguration) -> impl std::fmt::Display + use<V> {
        self.ensure_output_preconditions();

        let (struct_opening_char, struct_closing_char, array_opening_char, array_closing_char) =
            match build_config.output_format {
                OutputFormat::RustCodegen => ('{', '}', '[', ']'),
                OutputFormat::RON | OutputFormat::RONPretty => ('(', ')', '(', ')'),
            };
        let (new_line_str, indentation_str, space_str) = match build_config.output_format {
            OutputFormat::RON => ("", "", ""),
            _ => ("\n", "\t", " "),
        };

        let mut output_string = format!("{}{new_line_str}", struct_opening_char);

        output_string.push_str(&format!(
            "{indentation_str}index_range_starting_indices:{space_str}{}{new_line_str}",
            array_opening_char
        ));
        for (i, (starting_index, _)) in self.index_ranges.iter().enumerate() {
            let comma_str = match build_config.output_format {
                OutputFormat::RON => {
                    if i == self.index_ranges.len() - 1 {
                        ""
                    } else {
                        ","
                    }
                }
                _ => ",",
            };
            output_string.push_str(&format!(
                "{indentation_str}{indentation_str}{:?}{comma_str}{new_line_str}",
                *starting_index
            ));
        }
        output_string.push_str(&format!(
            "{indentation_str}{},{new_line_str}",
            array_closing_char
        ));

        output_string.push_str(&format!(
            "{indentation_str}index_range_skip_amounts:{space_str}{}{new_line_str}",
            array_opening_char
        ));
        let mut total_skip_amount = 0;
        for (i, (_, skip_amount)) in self.index_ranges.iter().enumerate() {
            let comma_str = match build_config.output_format {
                OutputFormat::RON => {
                    if i == self.index_ranges.len() - 1 {
                        ""
                    } else {
                        ","
                    }
                }
                _ => ",",
            };
            total_skip_amount += skip_amount;
            output_string.push_str(&format!(
                "{indentation_str}{indentation_str}{:?}{comma_str}{new_line_str}",
                total_skip_amount
            ));
        }
        output_string.push_str(&format!(
            "{indentation_str}{},{new_line_str}",
            array_closing_char
        ));

        output_string.push_str(&format!(
            "{indentation_str}values:{space_str}{}{new_line_str}",
            array_opening_char
        ));
        for (i, (_, value)) in self.entries.iter().enumerate() {
            let comma_str = match build_config.output_format {
                OutputFormat::RON => {
                    if i == self.entries.len() - 1 {
                        ""
                    } else {
                        ","
                    }
                }
                _ => ",",
            };
            let entry_str = match build_config.value_formatting {
                ValueFormatting::Display => &format!(
                    "{indentation_str}{indentation_str}{}{comma_str}{new_line_str}",
                    *value
                ),
                ValueFormatting::Debug => &format!(
                    "{indentation_str}{indentation_str}{:?}{comma_str}{new_line_str}",
                    *value
                ),
            };
            output_string.push_str(entry_str);
        }
        let comma_str = match build_config.output_format {
            OutputFormat::RON => "",
            _ => ",",
        };
        output_string.push_str(&format!(
            "{indentation_str}{}{comma_str}{new_line_str}",
            array_closing_char
        ));
        output_string.push_str(&format!("{}", struct_closing_char));
        output_string
    }
}
