use std::{
    fs::File,
    io::{BufWriter, Write},
    path,
};

use non_contiguously_indexed_array::{NciArray, NciArrayDataGenerator};

mod constants;
use constants::*;

#[rustfmt::skip]
mod generated;

fn generate_test_1_array() {
    let arr = NciArray::new(
        &ARRAY_DATA_1.index_range_starting_indices,
        &ARRAY_DATA_1.index_range_skip_amounts,
        &ARRAY_DATA_1.values,
    );
    let mut generator = NciArrayDataGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_1.rs");
    dbg!(&path);
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArrayData;\n\npub const GENERATED_1: {} = NciArrayData {};",
        generator.build_type("i32"),
        generator.build(
            false
        ),
    )
    .unwrap();
}

fn generate_test_2_array() {
    let arr = NciArray::new(
        &ARRAY_DATA_2.index_range_starting_indices,
        &ARRAY_DATA_2.index_range_skip_amounts,
        &ARRAY_DATA_2.values,
    );
    let mut generator = NciArrayDataGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_2.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArrayData;\n\npub const GENERATED_2: {} = NciArrayData {};",
        generator.build_type("i32"),
        generator.build(
            false
        ),
    )
    .unwrap();
}

#[test]
fn array_data_generator_test_1() {
    generate_test_1_array();
    assert_eq!(generated::test_generated_1::GENERATED_1, ARRAY_DATA_1);
}

#[test]
fn array_data_generator_test_2() {
    generate_test_2_array();
    assert_eq!(generated::test_generated_2::GENERATED_2, ARRAY_DATA_2);
}

#[test]
#[cfg_attr(feature = "panic", should_panic)]
fn duplicate_entry_test() {
    let mut generator = NciArrayDataGenerator::new();

    generator.entry(0, 0);
    generator.entry(1, 1);
    generator.entry(1, 100);
    generator.entry(2, 2);

    let generated = generator.build(false);
    let string = format!("{generated}");
    assert_eq!(string.chars().filter(|c| *c == '1').count(), 1);
    assert_eq!(string.find("100"), None)
}
