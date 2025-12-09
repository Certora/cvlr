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
