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
