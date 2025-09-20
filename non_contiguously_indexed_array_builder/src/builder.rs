use non_contiguously_indexed_array::NciIndex;

pub struct NciArrayBuilder<I: NciIndex, V> {
    entries: Vec<(I, V)>,
}

pub enum OutputFormat {
    RustCodegen,
    RON,
    RONPretty,
}

pub enum ValueFormatting {
    Display,
    Debug,
    DisplayAlternate,
    DebugAlternate,
}

pub struct BuildConfiguration {
    pub output_format: OutputFormat,
    pub value_formatting: ValueFormatting,
}

impl<I: NciIndex + std::fmt::Debug, V: std::fmt::Display + std::fmt::Debug> Default
    for NciArrayBuilder<I, V>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<I: NciIndex + std::fmt::Debug, V: std::fmt::Display + std::fmt::Debug> NciArrayBuilder<I, V> {
    #[must_use]
    pub const fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn entry(&mut self, index: I, value: V) {
        self.entries.push((index, value));
    }

    fn ensure_output_preconditions(&mut self) {
        self.entries.sort_by_key(|(index, _value)| *index);
        for window in self.entries.windows(2) {
            assert!(
                window[0].0 != window[1].0,
                "Duplicate indices detected! index: {:?}; values: {:?}, {:?}",
                window[0].0,
                window[0].1,
                window[1].1
            );
        }
    }

    pub fn build(&mut self, build_config: &BuildConfiguration) -> String {
        use std::fmt::Write as _;

        self.ensure_output_preconditions();

        let mut segments_idx_begin = Vec::new();
        let mut segments_mem_idx_begin = Vec::new();

        for mem_idx in 0..self.entries.len() {
            let new_segment = if mem_idx == 0 {
                true
            } else {
                let prv_entry_idx = self.entries[mem_idx - 1].0;
                let cur_entry_idx = self.entries[mem_idx].0;
                prv_entry_idx.distance(cur_entry_idx) != Some(1)
            };
            if new_segment {
                segments_idx_begin.push(self.entries[mem_idx].0);
                segments_mem_idx_begin.push(mem_idx);
            }
        }

        assert!(
            non_contiguously_indexed_array::check_segment_data_invariants(
                &segments_idx_begin,
                &segments_mem_idx_begin,
                self.entries.len(),
            ).is_ok(),
            "Segment data does not fulfill invariants!"
        );

        let (struct_opening_str, struct_closing_str, array_opening_str, array_closing_str) =
            match build_config.output_format {
                OutputFormat::RustCodegen => ("{", "}", "&[", "]"),
                OutputFormat::RON | OutputFormat::RONPretty => ("(", ")", "(", ")"),
            };
        let (new_line_str, indentation_str, space_str) = match build_config.output_format {
            OutputFormat::RON => ("", "", ""),
            _ => ("\n", "\t", " "),
        };

        let mut output_string = format!("{struct_opening_str}{new_line_str}");

        write!(
            output_string,
            "{indentation_str}segments_idx_begin:{space_str}{array_opening_str}{new_line_str}"
        )
        .unwrap();
        for (i, idx_begin) in segments_idx_begin.iter().enumerate() {
            let comma_str = match build_config.output_format {
                OutputFormat::RON => {
                    if i == segments_idx_begin.len() - 1 {
                        ""
                    } else {
                        ","
                    }
                }
                _ => ",",
            };
            write!(
                output_string,
                "{indentation_str}{indentation_str}{:?}{comma_str}{new_line_str}",
                *idx_begin
            )
            .unwrap();
        }
        write!(
            output_string,
            "{indentation_str}{array_closing_str},{new_line_str}"
        )
        .unwrap();

        write!(
            output_string,
            "{indentation_str}segments_mem_idx_begin:{space_str}{array_opening_str}{new_line_str}"
        )
        .unwrap();
        for (i, mem_idx_begin) in segments_mem_idx_begin.iter().enumerate() {
            let comma_str = match build_config.output_format {
                OutputFormat::RON => {
                    if i == segments_mem_idx_begin.len() - 1 {
                        ""
                    } else {
                        ","
                    }
                }
                _ => ",",
            };
            write!(
                output_string,
                "{indentation_str}{indentation_str}{:?}{comma_str}{new_line_str}",
                *mem_idx_begin
            )
            .unwrap();
        }
        write!(
            output_string,
            "{indentation_str}{array_closing_str},{new_line_str}",
        )
        .unwrap();

        write!(
            output_string,
            "{indentation_str}values:{space_str}{array_opening_str}{new_line_str}",
        )
        .unwrap();
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
                ValueFormatting::DisplayAlternate => &format!(
                    "{indentation_str}{indentation_str}{:#}{comma_str}{new_line_str}",
                    *value
                ),
                ValueFormatting::DebugAlternate => &format!(
                    "{indentation_str}{indentation_str}{:#?}{comma_str}{new_line_str}",
                    *value
                ),
            };
            write!(output_string, "{entry_str}").unwrap();
        }
        let comma_str = match build_config.output_format {
            OutputFormat::RON => "",
            _ => ",",
        };
        write!(
            output_string,
            "{indentation_str}{array_closing_str}{comma_str}{new_line_str}"
        )
        .unwrap();
        write!(output_string, "{struct_closing_str}").unwrap();
        output_string
    }
}
