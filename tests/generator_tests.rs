use std::{
    fs::File,
    io::{BufWriter, Write},
    path,
};

use non_contiguously_indexed_array::{NciArray, NciBaseArrayGenerator};

mod constants;
use constants::*;

mod generated;

fn generate_test_1_array() {
    let arr = NciArray::new(&BASE_ARRAY_1.index_ranges, &BASE_ARRAY_1.data);
    let mut generator = NciBaseArrayGenerator::new();
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
        "{}",
        generator.build(
            "use non_contiguously_indexed_array::NciBaseArray;

pub const GENERATED_1: NciBaseArray<i32, {R}, {N}> = NciBaseArray",
            ";"
        ),
    )
    .unwrap();
}

fn generate_test_2_array() {
    let arr = NciArray::new(&BASE_ARRAY_2.index_ranges, &BASE_ARRAY_2.data);
    let mut generator = NciBaseArrayGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_2.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "{}",
        generator.build(
            "use non_contiguously_indexed_array::NciBaseArray;

pub const GENERATED_2: NciBaseArray<i32, {R}, {N}> = NciBaseArray",
            ";"
        ),
    )
    .unwrap();
}

#[test]
fn base_array_generator_test_1() {
    generate_test_1_array();
    assert_eq!(generated::test_generated_1::GENERATED_1, BASE_ARRAY_1);
}

#[test]
fn base_array_generator_test_2() {
    generate_test_2_array();
    assert_eq!(generated::test_generated_2::GENERATED_2, BASE_ARRAY_2);
}
