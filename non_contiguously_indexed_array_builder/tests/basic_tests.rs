use std::{
    fs::File,
    io::{BufWriter, Write},
    path,
};

use non_contiguously_indexed_array_builder::{
    BuildConfiguration, NciArrayBuilder, OutputFormat, ValueFormatting,
};

mod constants;
use constants::*;

#[rustfmt::skip]
mod generated;

const DEFAUTLT_BUILD_CONFIGURATION: BuildConfiguration = BuildConfiguration {
    output_format: OutputFormat::RustCodegen,
    value_formatting: ValueFormatting::Display,
};

fn build_test_1_array() {
    let arr = ARRAY_1;
    let mut builder = NciArrayBuilder::new();
    for (index, value) in arr.entries() {
        builder.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_1.rs");
    dbg!(&path);
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_1: NciArray<u32, u32> = NciArray {};",
        builder.build(&DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

fn build_test_2_array() {
    let arr = ARRAY_2;
    let mut builder = NciArrayBuilder::new();
    for (index, value) in arr.entries() {
        builder.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_2.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_2: NciArray<u32, u32> = NciArray {};",
        builder.build(&DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

#[test]
fn array_generator_test_1() {
    build_test_1_array();
    assert_eq!(
        generated::test_generated_1::GENERATED_1
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<(u32, u32)>>(),
        ARRAY_1
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<(u32, u32)>>()
    );
}

#[test]
fn array_generator_test_2() {
    build_test_2_array();
    assert_eq!(
        generated::test_generated_2::GENERATED_2
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<(u32, u32)>>(),
        ARRAY_2
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<(u32, u32)>>()
    );
}
