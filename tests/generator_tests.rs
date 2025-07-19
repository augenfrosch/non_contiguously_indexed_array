use std::{
    fs::File,
    io::{BufWriter, Write},
    path,
};

use non_contiguously_indexed_array::{
    BuildConfiguration, NciArrayGenerator, OutputFormat, ValueFormatting,
};

mod constants;
use constants::*;

#[rustfmt::skip]
mod generated;

const DEFAUTLT_BUILD_CONFIGURATION: BuildConfiguration = BuildConfiguration {
    output_format: OutputFormat::RustCodegen,
    value_formatting: ValueFormatting::Display,
};
#[cfg(feature = "serde")]
const DEFAUTLT_RON_BUILD_CONFIGURATION: BuildConfiguration = BuildConfiguration {
    output_format: OutputFormat::RON,
    value_formatting: ValueFormatting::Display,
};

fn generate_test_1_array() {
    let arr = ARRAY_1;
    let mut generator = NciArrayGenerator::new();
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
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_1: {} = NciArray {};",
        generator.build_type("i32"),
        generator.build(DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

fn generate_test_2_array() {
    let arr = ARRAY_2;
    let mut generator = NciArrayGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let path = path::absolute("./tests/generated")
        .unwrap()
        .join("test_generated_2.rs");
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writeln!(
        writer,
        "use non_contiguously_indexed_array::NciArray;\n\npub const GENERATED_2: {} = NciArray {};",
        generator.build_type("i32"),
        generator.build(DEFAUTLT_BUILD_CONFIGURATION),
    )
    .unwrap();
}

#[test]
fn array_generator_test_1() {
    generate_test_1_array();
    assert_eq!(generated::test_generated_1::GENERATED_1, ARRAY_1);
}

#[test]
fn array_generator_test_2() {
    generate_test_2_array();
    assert_eq!(generated::test_generated_2::GENERATED_2, ARRAY_2);
}

#[test]
#[cfg_attr(feature = "panic", should_panic)]
fn duplicate_entry_test() {
    let mut generator = NciArrayGenerator::new();

    generator.entry(0, 0);
    generator.entry(1, 1);
    generator.entry(1, 100);
    generator.entry(2, 2);

    let generated = generator.build(DEFAUTLT_BUILD_CONFIGURATION);
    let string = format!("{generated}");
    assert_eq!(string.chars().filter(|c| *c == '1').count(), 1);
    assert_eq!(string.find("100"), None)
}

#[test]
#[cfg(feature = "serde")]
fn serde_test_1() {
    use non_contiguously_indexed_array::NciArray;

    generate_test_1_array();
    //let serialized = ron::to_string(&generated::test_generated_1::GENERATED_1).unwrap();
    let arr = ARRAY_1;
    let mut generator = NciArrayGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let built_ron = format!("{}", generator.build(DEFAUTLT_RON_BUILD_CONFIGURATION));

    let deserialized: NciArray<i32, 3, 6> = ron::from_str(&built_ron).unwrap();
    assert_eq!(generated::test_generated_1::GENERATED_1, deserialized);

    // assert_eq!(
    //     built_ron,
    //     ron::to_string(&generated::test_generated_1::GENERATED_1).unwrap()
    // );

    let built_ron = format!(
        "{}",
        generator.build(BuildConfiguration {
            output_format: OutputFormat::RONPretty,
            value_formatting: ValueFormatting::Display
        })
    );
    let deserialized: NciArray<i32, 3, 6> = ron::from_str(&built_ron).unwrap();
    assert_eq!(generated::test_generated_1::GENERATED_1, deserialized);

    // assert_eq!(
    //     built_ron,
    //     ron::ser::to_string_pretty(
    //         &generated::test_generated_1::GENERATED_1,
    //         ron::ser::PrettyConfig::new()
    //             .indentor("\t".to_string())
    //             .separate_tuple_members(true) // `serde-big-array` seems to serialize them as tuples not arrays
    //     )
    //     .unwrap()
    // );
}

#[test]
#[cfg(feature = "serde")]
fn serde_test_2() {
    use non_contiguously_indexed_array::NciArray;

    generate_test_2_array();

    let arr = ARRAY_2;
    let mut generator = NciArrayGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let built_ron = format!("{}", generator.build(DEFAUTLT_RON_BUILD_CONFIGURATION));
    // println!("{}", built_ron);

    let deserialized: NciArray<i32, 3, 6> = ron::from_str(&built_ron).unwrap();
    assert_eq!(generated::test_generated_2::GENERATED_2, deserialized);

    // assert_eq!(
    //     built_ron,
    //     ron::to_string(&generated::test_generated_2::GENERATED_2).unwrap()
    // );

    let built_ron = format!(
        "{}",
        generator.build(BuildConfiguration {
            output_format: OutputFormat::RONPretty,
            value_formatting: ValueFormatting::Display
        })
    );
    // println!("{}", built_ron);

    let deserialized: NciArray<i32, 3, 6> = ron::from_str(&built_ron).unwrap();
    assert_eq!(generated::test_generated_2::GENERATED_2, deserialized);

    // assert_eq!(
    //     built_ron,
    //     ron::ser::to_string_pretty(
    //         &generated::test_generated_2::GENERATED_2,
    //         ron::ser::PrettyConfig::new()
    //             .indentor("\t".to_string())
    //             .separate_tuple_members(true)
    //     )
    //     .unwrap()
    // );
}

#[test]
#[cfg(feature = "serde")]
fn serde_test_3() {
    use non_contiguously_indexed_array::NciArray;

    generate_test_1_array();

    let arr = ARRAY_1;
    let mut generator = NciArrayGenerator::new();
    for (index, value) in arr.entries() {
        generator.entry(index, value);
    }
    let built_rust_codegen = generator.build(DEFAUTLT_BUILD_CONFIGURATION);

    let built_to_ron = non_contiguously_indexed_array::built_rust_codegen_to(&built_rust_codegen, OutputFormat::RON, None);
    println!("{}", built_to_ron);

    let deserialized: NciArray<i32, 3, 6> = ron::from_str(&built_to_ron).unwrap();
    assert_eq!(generated::test_generated_1::GENERATED_1, deserialized);
}
