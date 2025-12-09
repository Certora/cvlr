#![allow(unused)]
use cvlr::nondet::Nondet;
use cvlr_derive::Nondet;

// Test unit struct
#[derive(Nondet)]
struct UnitStruct;

// Test struct with named fields
#[derive(Nondet)]
struct NamedFields {
    x: u64,
    y: u64,
    z: i32,
}

// Test struct with unnamed fields (tuple struct)
#[derive(Nondet)]
struct UnnamedFields(u64, i32, bool);

// Test struct with various field types
#[derive(Nondet)]
struct MixedTypes {
    a: u8,
    b: i16,
    c: u32,
    d: i64,
    e: bool,
}

// Test nested struct
#[derive(Nondet)]
struct Nested {
    point: NamedFields,
    value: u64,
}

// Test enum with unit variant
#[derive(Nondet)]
enum SimpleEnum {
    Variant1,
    Variant2,
}

// Test enum with unnamed fields
#[derive(Nondet)]
enum EnumWithUnnamed {
    Variant1,
    Variant2(u64),
    Variant3(u64, i32),
}

// Test enum with named fields
#[derive(Nondet)]
enum EnumWithNamed {
    Variant1,
    Variant2 { x: u64 },
    Variant3 { x: u64, y: i32 },
}

// Test enum with mixed variants
#[derive(Nondet)]
enum MixedEnum {
    Unit,
    Unnamed(u64, i32),
    Named { x: u64, y: bool },
}

#[test]
fn test_unit_struct() {
    let _unit = UnitStruct::nondet();
}

#[test]
fn test_named_fields() {
    let point = NamedFields::nondet();
    // Just verify it compiles and can be called
    let _x = point.x;
    let _y = point.y;
    let _z = point.z;
}

#[test]
fn test_unnamed_fields() {
    let tuple = UnnamedFields::nondet();
    // Just verify it compiles and can be accessed
    let _0 = tuple.0;
    let _1 = tuple.1;
    let _2 = tuple.2;
}

#[test]
fn test_mixed_types() {
    let mixed = MixedTypes::nondet();
    // Verify all fields are accessible
    let _a = mixed.a;
    let _b = mixed.b;
    let _c = mixed.c;
    let _d = mixed.d;
    let _e = mixed.e;
}

#[test]
fn test_nested_struct() {
    let nested = Nested::nondet();
    // Verify nested struct is created
    let _point = nested.point;
    let _value = nested.value;
}

#[test]
fn test_nondet_trait_method() {
    // Test that the generated impl works with the trait method
    let point: NamedFields = cvlr::nondet::nondet();
    let _x = point.x;
}

#[test]
fn test_nondet_with() {
    // Test that nondet_with works (uses the trait method)
    let point = NamedFields::nondet_with(|p| p.x == 0);
    let _x = point.x;
}

#[test]
fn test_simple_enum() {
    let _e = SimpleEnum::nondet();
}

#[test]
fn test_enum_with_unnamed() {
    let e = EnumWithUnnamed::nondet();
    match e {
        EnumWithUnnamed::Variant1 => {}
        EnumWithUnnamed::Variant2(_) => {}
        EnumWithUnnamed::Variant3(_, _) => {}
    }
}

#[test]
fn test_enum_with_named() {
    let e = EnumWithNamed::nondet();
    match e {
        EnumWithNamed::Variant1 => {}
        EnumWithNamed::Variant2 { x: _ } => {}
        EnumWithNamed::Variant3 { x: _, y: _ } => {}
    }
}

#[test]
fn test_mixed_enum() {
    let e = MixedEnum::nondet();
    match e {
        MixedEnum::Unit => {}
        MixedEnum::Unnamed(_, _) => {}
        MixedEnum::Named { x: _, y: _ } => {}
    }
}

#[test]
fn test_enum_nondet_trait_method() {
    let e: SimpleEnum = cvlr::nondet::nondet();
    match e {
        SimpleEnum::Variant1 | SimpleEnum::Variant2 => {}
    }
}

#[test]
fn expand_tests() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/expand/test_nondet_union.rs");
}
