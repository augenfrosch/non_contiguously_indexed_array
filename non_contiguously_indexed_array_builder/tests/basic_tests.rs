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

fn build_test_3_array() {
    let arr = ARRAY_3;
    let mut builder = NciArrayBuilder::new();
    for (index, value) in arr.entries() {
        builder.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_3.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_3: NciArray<i32, i32> = NciArray {};",
        builder.build(&DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

fn build_test_4_array() {
    let arr = ARRAY_4;
    let mut builder = NciArrayBuilder::new();
    for (index, value) in arr.entries() {
        builder.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_4.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_4: NciArray<i32, i32> = NciArray {};",
        builder.build(&DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

fn build_test_5_array() {
    let mut builder = NciArrayBuilder::new();
    for i in i8::MIN..=i8::MAX {
        builder.entry(i, i);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_5.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_5: NciArray<i8, i8> = NciArray {};",
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
            .collect::<Vec<_>>(),
        ARRAY_1
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>()
    );
}

#[test]
fn array_generator_test_2() {
    build_test_2_array();
    assert_eq!(
        generated::test_generated_2::GENERATED_2
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>(),
        ARRAY_2
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>()
    );
}

#[test]
fn array_generator_test_3() {
    build_test_3_array();
    assert_eq!(
        generated::test_generated_3::GENERATED_3
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>(),
        ARRAY_3
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>()
    );
}

#[test]
fn array_generator_test_4() {
    build_test_4_array();
    assert_eq!(
        generated::test_generated_4::GENERATED_4
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>(),
        ARRAY_4
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>()
    );
}

#[test]
fn array_generator_test_5() {
    build_test_5_array();
    assert_eq!(
        generated::test_generated_5::GENERATED_5
            .entries()
            .map(|(index, element)| (index, *element))
            .collect::<Vec<_>>(),
        (i8::MIN..=i8::MAX)
            .zip(i8::MIN..=i8::MAX)
            .collect::<Vec<_>>()
    );
    for i in i8::MIN..=i8::MAX {
        assert_eq!(generated::test_generated_5::GENERATED_5.get(i), Some(&i));
    }
}
