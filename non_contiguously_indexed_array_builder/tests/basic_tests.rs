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

macro_rules! build_test_array {
    ( $id:literal, $iter:expr, $ty:tt, $conf:expr) => {
        let mut builder = NciArrayBuilder::new();
        for (index, value) in $iter {
            builder.entry(index, value);
        }
        let path = path::absolute("./tests/generated")
            .unwrap()
            .join(format!("test_generated_{}.rs", $id));
        let mut writer = BufWriter::new(File::create(path).unwrap());
        writeln!(
            writer,
            "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_{}: NciArray<{}, {}> = NciArray{};",
            $id,
            $ty.0,
            $ty.1,
            builder.build($conf),
        )
        .unwrap();
    };
}

#[test]
fn array_builder_test_1() {
    build_test_array!(
        1,
        ARRAY_1.entries(),
        ("u32", "u32"),
        &DEFAUTLT_BUILD_CONFIGURATION
    );
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
fn array_builder_test_2() {
    build_test_array!(
        2,
        ARRAY_2.entries(),
        ("u32", "u32"),
        &DEFAUTLT_BUILD_CONFIGURATION
    );
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
fn array_builder_test_3() {
    build_test_array!(
        3,
        ARRAY_3.entries(),
        ("i32", "i32"),
        &DEFAUTLT_BUILD_CONFIGURATION
    );
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
fn array_builder_test_4() {
    build_test_array!(
        4,
        ARRAY_4.entries(),
        ("i32", "i32"),
        &DEFAUTLT_BUILD_CONFIGURATION
    );
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
fn array_builder_test_5() {
    build_test_array!(
        5,
        (i8::MIN..=i8::MAX).zip(i8::MIN..=i8::MAX),
        ("i8", "i8"),
        &DEFAUTLT_BUILD_CONFIGURATION
    );
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

#[test]
#[should_panic]
fn array_builder_test_panic_on_duplicate() {
    let mut builder = NciArrayBuilder::new();
    builder.entry(0, i8::MIN);
    builder.entry(0, i8::MAX);
    assert!(builder.build(&DEFAUTLT_BUILD_CONFIGURATION).is_empty()); // Assertion fails, but should never be executed
}
