#![allow(unused)]
use cvlr::log::CvlrLog;
use cvlr_derive::CvlrLog;

// Test struct with named fields
#[derive(CvlrLog)]
struct Point {
    x: u64,
    y: u64,
}

// Test unit struct
#[derive(CvlrLog)]
struct UnitStruct;

// Test struct with various field types
#[derive(CvlrLog)]
struct MixedTypes {
    a: u8,
    b: i16,
    c: u32,
    d: i64,
    e: bool,
}

// Test tuple struct (unnamed fields)
#[derive(CvlrLog)]
struct TupleStruct(u64, i32, bool);

#[test]
fn test_point_log() {
    let point = Point { x: 1, y: 2 };
    let mut logger = cvlr::log::CvlrLogger::new();
    point.log("point", &mut logger);
}

#[test]
fn test_unit_struct_log() {
    let unit = UnitStruct;
    let mut logger = cvlr::log::CvlrLogger::new();
    unit.log("unit", &mut logger);
}

#[test]
fn test_mixed_types_log() {
    let mixed = MixedTypes {
        a: 1,
        b: -2,
        c: 3,
        d: -4,
        e: true,
    };
    let mut logger = cvlr::log::CvlrLogger::new();
    mixed.log("mixed", &mut logger);
}

#[test]
fn test_tuple_struct_log() {
    let tuple = TupleStruct(1, -2, true);
    let mut logger = cvlr::log::CvlrLogger::new();
    tuple.log("tuple", &mut logger);
}

// Test enum with unit variants
#[derive(CvlrLog)]
enum SimpleEnum {
    Variant1,
    Variant2,
}

// Test enum with unnamed fields
#[derive(CvlrLog)]
enum EnumWithUnnamed {
    Variant1,
    Variant2(u64),
    Variant3(u64, i32),
}

// Test enum with named fields
#[derive(CvlrLog)]
enum EnumWithNamed {
    Variant1,
    Variant2 { x: u64 },
    Variant3 { x: u64, y: i32 },
}

#[test]
fn test_simple_enum_log() {
    let e1 = SimpleEnum::Variant1;
    let e2 = SimpleEnum::Variant2;
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
}

#[test]
fn test_enum_with_unnamed_log() {
    let e1 = EnumWithUnnamed::Variant1;
    let e2 = EnumWithUnnamed::Variant2(42);
    let e3 = EnumWithUnnamed::Variant3(10, -20);
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
    e3.log("e3", &mut logger);
}

#[test]
fn test_enum_with_named_log() {
    let e1 = EnumWithNamed::Variant1;
    let e2 = EnumWithNamed::Variant2 { x: 42 };
    let e3 = EnumWithNamed::Variant3 { x: 10, y: -20 };
    let mut logger = cvlr::log::CvlrLogger::new();
    e1.log("e1", &mut logger);
    e2.log("e2", &mut logger);
    e3.log("e3", &mut logger);
}
