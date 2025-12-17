use cvlr_macros::cvlr_rule_for_spec;
pub fn test_basic_with_base_prefix() {
    let expr = true;
    {
        let _rule_name = "solvency_update_exchange_price_no_interest_free_new";
        let _spec = expr;
        let _base = "base_update_exchange_price_no_interest_free_new";
    }
}
pub fn test_without_base_prefix() {
    let expr = false;
    {
        let _rule_name = "liquidity_update_price";
        let _spec = expr;
        let _base = "update_price";
    }
}
pub fn test_name_conversion() {
    let expr = 42;
    {
        let _rule_name = "my_rule_name_test_function";
        let _spec = expr;
        let _base = "base_test_function";
    }
    {
        let _rule_name = "test_rule_name_another_function";
        let _spec = expr;
        let _base = "base_another_function";
    }
    {
        let _rule_name = "already_snake_case_simple_func";
        let _spec = expr;
        let _base = "base_simple_func";
    }
}
pub fn test_complex_spec_expressions() {
    let complex_expr = || { true };
    {
        let _rule_name = "complex_handler";
        let _spec = complex_expr();
        let _base = "base_handler";
    }
    {
        let _rule_name = "nested_calculator";
        let _spec = (1 + 2) * 3;
        let _base = "base_calculator";
    }
}
pub fn test_trailing_comma() {
    let expr = true;
    {
        let _rule_name = "trailing_test";
        let _spec = expr;
        let _base = "base_test";
    }
}
pub fn test_edge_cases() {
    let expr = 0;
    {
        let _rule_name = "x_func";
        let _spec = expr;
        let _base = "base_func";
    }
    {
        let _rule_name = "very_long_rule_name_that_should_work_very_long_function_name_that_should_also_work";
        let _spec = expr;
        let _base = "base_very_long_function_name_that_should_also_work";
    }
    {
        let _rule_name = "rule123_test456";
        let _spec = expr;
        let _base = "base_test456";
    }
}
pub fn test_spec_with_method_calls() {
    let expr = <[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3]));
    {
        let _rule_name = "method_check";
        let _spec = expr.len() > 0;
        let _base = "base_check";
    }
    {
        let _rule_name = "chained_sum";
        let _spec = expr.iter().sum::<i32>() > 0;
        let _base = "base_sum";
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
