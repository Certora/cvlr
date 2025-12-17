use cvlr_macros::cvlr_rule_for_spec;

// Mock macro for testing - this would be provided by the user in real usage
// The macro generates cvlr_impl_rule!{rule_name, spec, base} with braces
macro_rules! cvlr_impl_rule {
    {$rule_name:ident, $spec:expr, $base:ident} => {
        // Just verify the macro expands correctly - the actual implementation
        // would be provided by the user
        {
            let _rule_name = stringify!($rule_name);
            let _spec = $spec;
            let _base = stringify!($base);
        }
    };
}

pub fn test_basic_with_base_prefix() {
    let expr = true;
    
    // Basic usage with base_ prefix (should be stripped)
    cvlr_rule_for_spec! {
        name: "solvency",
        spec: expr,
        base: base_update_exchange_price_no_interest_free_new,
    }
}

pub fn test_without_base_prefix() {
    let expr = false;
    
    // Base identifier without base_ prefix (should not be stripped)
    cvlr_rule_for_spec! {
        name: "liquidity",
        spec: expr,
        base: update_price,
    }
}

pub fn test_name_conversion() {
    let expr = 42;
    
    // Test name conversion to snake_case
    cvlr_rule_for_spec! {
        name: "My Rule Name",
        spec: expr,
        base: base_test_function,
    }
    
    // Test with hyphens
    cvlr_rule_for_spec! {
        name: "test-rule-name",
        spec: expr,
        base: base_another_function,
    }
    
    // Test already snake_case
    cvlr_rule_for_spec! {
        name: "already_snake_case",
        spec: expr,
        base: base_simple_func,
    }
}

pub fn test_complex_spec_expressions() {
    // Test with complex spec expressions
    let complex_expr = || { true };
    
    cvlr_rule_for_spec! {
        name: "complex",
        spec: complex_expr(),
        base: base_handler,
    }
    
    cvlr_rule_for_spec! {
        name: "nested",
        spec: (1 + 2) * 3,
        base: base_calculator,
    }
}

pub fn test_trailing_comma() {
    let expr = true;
    
    // Test with trailing comma (should be optional)
    cvlr_rule_for_spec! {
        name: "trailing",
        spec: expr,
        base: base_test,
    }
}

pub fn test_edge_cases() {
    let expr = 0;
    
    // Test with single character name
    cvlr_rule_for_spec! {
        name: "x",
        spec: expr,
        base: base_func,
    }
    
    // Test with very long names
    cvlr_rule_for_spec! {
        name: "very_long_rule_name_that_should_work",
        spec: expr,
        base: base_very_long_function_name_that_should_also_work,
    }
    
    // Test with numbers in name (should be converted properly)
    cvlr_rule_for_spec! {
        name: "rule123",
        spec: expr,
        base: base_test456,
    }
}

pub fn test_spec_with_method_calls() {
    let expr = vec![1, 2, 3];
    
    // Test with method calls in spec
    cvlr_rule_for_spec! {
        name: "method",
        spec: expr.len() > 0,
        base: base_check,
    }
    
    // Test with chained method calls
    cvlr_rule_for_spec! {
        name: "chained",
        spec: expr.iter().sum::<i32>() > 0,
        base: base_sum,
    }
}

pub fn main() {
    test_basic_with_base_prefix();
    test_without_base_prefix();
    test_name_conversion();
    test_complex_spec_expressions();
    test_trailing_comma();
    test_edge_cases();
    test_spec_with_method_calls();
}

