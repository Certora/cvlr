//! Procedural macros for CVLR: [`macro@rule`], [`macro@mock_fn`],
//! [`macro@cvlr_predicate`], the condition DSL ([`cvlr_assert_that!`],
//! [`cvlr_assert_all!`], [`cvlr_assume_that!`], [`cvlr_assume_all!`],
//! [`cvlr_eval_that!`], [`cvlr_eval_all!`]), [`cvlr_rule_for_spec!`] and
//! [`cvlr_pif!`]. Use them through the `cvlr` crate, whose paths the
//! expansions refer to.

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::{parse_macro_input, parse_quote, Ident, ItemFn};

mod assert_that;
mod mock;
mod predicate;
mod rule_for_spec;
/// Mark a method as a CVT rule
///
/// A rule is an entry point for the Certora Prover: the Prover verifies each
/// rule separately, over all possible values of the nondeterministic inputs
/// the rule draws (`nondet()`, `cvlr_deserialize_nondet_accounts()`, ...). A
/// rule passes if every `cvlr_assert!` holds on every execution that satisfies
/// the rule's assumptions, or, if the rule contains `cvlr_satisfy!`, if some
/// such execution satisfies one. Also available as `cvlr_rule`.
///
/// The attribute:
///
/// * adds `#[no_mangle]`, so the function keeps its name in the compiled
///   program, which is the name the rule is selected and reported by. Rule
///   names must therefore be unique across the program.
/// * records that name in the `.cvlr_rules` section, one record per rule that
///   was compiled. A `#[rule]` removed by `cfg`, or written on `fn $name`
///   inside a macro that is never invoked, contributes nothing. The record is
///   a static, the first statement of the body, and it does not run.
/// * inserts `cvlr_rule_location!()` as the next statement, the first one that
///   runs, recording the rule's source location for the report.
/// * appends `cvlr_vacuity_check!()` as the last statement, which lets the
///   Prover's rule-sanity check report a rule whose end is unreachable, i.e.
///   one that passes vacuously (see `cvlr_vacuity_check!`).
///
/// A rule takes no parameters and returns `()`; because a statement is
/// appended to the body, the body must not end in a tail expression that
/// produces a value. An early `return` skips the appended vacuity check.
///
/// # Example
///
/// ```rust,no_run
/// use cvlr::prelude::*;
/// #[rule]
/// fn foo()  {
///    cvlr_assert!(false);
/// }
/// ```
#[proc_macro_attribute]
pub fn rule(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_ast = parse_macro_input!(item as ItemFn);
    // add #[no_mangle] attribute
    fn_ast.attrs.push(parse_quote! { #[no_mangle] });
    // The static is the first statement. `cvlr_rule_location!` is the first one that runs.
    fn_ast
        .block
        .stmts
        .insert(0, rule_manifest_stmt(&fn_ast.sig.ident));
    fn_ast
        .block
        .stmts
        .insert(1, parse_quote! { cvlr::log::cvlr_rule_location!(); });
    fn_ast
        .block
        .stmts
        .push(parse_quote! { cvlr::cvlr_vacuity_check!(); });
    fn_ast.into_token_stream().into()
}

/// A tag-1 record of the `#[no_mangle]` name, in `.cvlr_rules`.
///
/// `unraw` drops a raw-ident `r#` prefix. The export does not keep it.
fn rule_manifest_stmt(fn_name: &Ident) -> syn::Stmt {
    let rule_name = fn_name.unraw().to_string();
    let static_ident = format_ident!("CVLR_RULE_MANIFEST_{}", rule_name, span = fn_name.span());
    let export_name = format!("cvlr_rule_manifest.{rule_name}");
    let record = rule_manifest_record(&rule_name);
    let record_len = Literal::usize_unsuffixed(record.len());
    let bytes: Vec<Literal> = record
        .into_iter()
        .enumerate()
        .map(|(index, byte)| {
            if index >= 5 && byte.is_ascii_graphic() && byte != b'\'' && byte != b'\\' {
                Literal::byte_character(byte)
            } else {
                Literal::u8_unsuffixed(byte)
            }
        })
        .collect();
    parse_quote! {
        #[allow(non_upper_case_globals)]
        #[used]
        #[export_name = #export_name]
        #[link_section = ".cvlr_rules"]
        static #static_ident: [u8; #record_len] = [#(#bytes),*];
    }
}

fn rule_manifest_record(name: &str) -> Vec<u8> {
    let len = u32::try_from(name.len()).expect("a rule name fits in a u32");
    let mut record = Vec::with_capacity(5 + name.len());
    record.push(1);
    record.extend_from_slice(&len.to_le_bytes());
    record.extend_from_slice(name.as_bytes());
    record
}

/// Replace a function with a mock when a cargo feature is enabled.
///
/// Use this to swap code the Prover cannot or should not analyze (a CPI call, a
/// heavy computation, an external dependency) for a verification-only model,
/// without touching its call sites.
///
/// # Syntax
///
/// ```ignore
/// #[cvlr::mock_fn(with = path::to::mock)]
/// #[cvlr::mock_fn(with = path::to::mock, when = "feature-name")]
/// ```
///
/// * `with` (required) — path to the replacement item, normally a function
///   with the same signature as the annotated one.
/// * `when` (optional, default `"certora"`) — name of the cargo feature that
///   selects the mock. The feature is checked in the crate that contains the
///   annotated function, so that crate must declare it.
///
/// # Expansion
///
/// ```ignore
/// #[cfg(not(feature = "certora"))]
/// pub fn original(..) { .. }            // the annotated function, unchanged
///
/// #[cfg(feature = "certora")]
/// pub use path::to::mock as original;   // same visibility as the original
/// ```
///
/// With the feature on, the original body is not compiled at all and every use
/// of `original` resolves to the mock; with it off, the attribute has no
/// effect. Because the mock is re-exported under the original name, it must be
/// callable exactly as the original is at each call site.
///
/// # Example
///
/// ```ignore
/// #[cvlr::mock_fn(with = crate::certora::mocks::spl_mint_shares)]
/// pub fn spl_mint_shares(amount: u64, shares: &AccountInfo) -> ProgramResult {
///     // real CPI to the token program
/// }
///
/// // crate::certora::mocks
/// pub fn spl_mint_shares(amount: u64, shares: &AccountInfo) -> ProgramResult {
///     // model of the token program's effect
/// }
/// ```
///
/// It is common to also gate the attribute itself, as
/// `#[cfg_attr(feature = "certora", cvlr::mock_fn(with = ...))]`, so that
/// non-verification builds do not depend on `cvlr` at all.
#[proc_macro_attribute]
pub fn mock_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    mock::mock_fn_impl(attr, item)
}

/// Converts a function into a CVLR predicate.
///
/// This attribute macro transforms a function into a struct that implements
/// `cvlr::spec::CvlrFormula` for the specified context type.
/// The function body is parsed and used to generate `eval`, `assert`, and `assume`
/// methods using the same helpers as `cvlr::spec::cvlr_def_predicate!`.
/// Also available as `#[cvlr::predicate]`; use [`cvlr_pif!`] to name the
/// generated struct from the function name.
///
/// # Syntax
///
/// Single-state predicate:
/// ```ignore
/// #[cvlr_predicate]
/// pub fn predicate_name(c: &Ctx) {
///     c.x > 0;
///     c.y < 100;
/// }
/// ```
///
/// Two-state predicate:
/// ```ignore
/// #[cvlr_predicate]
/// pub fn solvency_post(c: &SolvencyCtx, old: &SolvencyCtx) {
///     c.borrow_value > old.borrow_value;
/// }
/// ```
///
/// # Parameters
///
/// * The function must have exactly one or two parameters of type `&Ctx` or `&mut Ctx`
/// * For two-parameter predicates, both parameters must have the same context type
/// * Two-parameter predicates use `eval_with_states`, `assert_with_states`, and `assume_with_states` methods
/// * The first parameter represents the post-state, the second represents the pre-state
/// * The function body can contain one or more expressions (statements ending with `;`)
/// * The function name will be converted from snake_case to PascalCase for the struct name
///
/// # Body
///
/// The body may contain only `let` statements and condition expressions. All
/// `let` statements are evaluated first, before any condition, wherever they
/// appear. Each condition uses the DSL of [`cvlr_assert_that!`]: a comparison,
/// any other `bool` expression, or `if guard { cond } else { cond2 }`.
/// `assert` checks each condition as a separate assertion (comparisons log
/// both operands), `assume` assumes each one, and `eval` is their
/// conjunction. Because `eval` needs every condition to be a `bool`, a
/// guarded condition must have an `else` branch: write
/// `if guard { cond } else { true }` for an implication.
///
/// # Generated items
///
/// The struct has the function's visibility. The original function is kept
/// (marked `dead_code`) only so that editors report errors in the body; do not
/// call it. A two-parameter predicate is two-state: use it only as an ensures
/// clause. Its single-state `eval`/`assert`/`assume` fail the rule with
/// `cvlr_assert!(false)`. The expansion refers to `::cvlr::...` paths, so the
/// crate must depend on `cvlr`.
///
/// # Examples
///
/// ```ignore
/// use cvlr_macros::cvlr_predicate;
/// use cvlr_spec::CvlrFormula;
///
/// struct Ctx {
///     x: i32,
///     y: i32,
/// }
///
/// #[cvlr_predicate]
/// pub fn x_gt_zero(c: &Ctx) {
///     c.x > 0;
/// }
///
/// // This generates:
/// // pub struct XGtZero;
/// // impl CvlrFormula for XGtZero { type Context = Ctx; ... }
///
/// let ctx = Ctx { x: 5, y: 10 };
/// let pred = XGtZero;
/// assert!(pred.eval(&ctx));
/// ```
///
/// Two-state predicate example:
/// ```ignore
/// #[cvlr_predicate]
/// pub fn x_increased(c: &Ctx, old: &Ctx) {
///     c.x > old.x;
/// }
///
/// let pre = Ctx { x: 1, y: 2 };
/// let post = Ctx { x: 5, y: 10 };
/// let pred = XIncreased;
/// assert!(pred.eval_with_states(&post, &pre));
/// ```
///
/// # Generated Code
///
/// For single-parameter predicates, the macro generates a struct and implementation similar to `cvlr_def_predicate!`:
///
/// ```ignore
/// pub struct XGtZero;
/// impl CvlrFormula for XGtZero {
///     type Context = Ctx;
///     fn eval(&self, ctx: &Ctx) -> bool {
///         let c = ctx;
///         cvlr_eval_all!(c.x > 0)
///     }
///     fn assert(&self, ctx: &Ctx) {
///         let c = ctx;
///         cvlr_assert_all!(c.x > 0);
///     }
///     fn assume(&self, ctx: &Ctx) {
///         let c = ctx;
///         cvlr_assume_all!(c.x > 0);
///     }
/// }
/// ```
///
/// For two-parameter predicates, the macro generates methods using `_with_states`:
///
/// ```ignore
/// pub struct XIncreased;
/// impl CvlrFormula for XIncreased {
///     type Context = Ctx;
///     fn eval_with_states(&self, ctx0: &Ctx, ctx1: &Ctx) -> bool {
///         let c = ctx0;  // post-state
///         let old = ctx1;  // pre-state
///         cvlr_eval_all!(c.x > old.x)
///     }
///     // ... similar for assert_with_states and assume_with_states
/// }
/// ```
#[proc_macro_attribute]
pub fn cvlr_predicate(attr: TokenStream, item: TokenStream) -> TokenStream {
    predicate::cvlr_predicate_impl(attr, item)
}

/// Assert a condition using a DSL syntax
///
/// This macro provides a convenient DSL for writing assertions and automatically detects
/// comparison operators to expand to the appropriate `cvlr_assert_*` macros.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_assert_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let x = 5;
/// let y = 10;
///
/// cvlr_assert_that!(x < y);        // expands to cvlr_assert_lt!(x, y)
/// cvlr_assert_that!(x <= y);       // expands to cvlr_assert_le!(x, y)
/// cvlr_assert_that!(x > 0);        // expands to cvlr_assert_gt!(x, 0)
/// cvlr_assert_that!(x >= 0);       // expands to cvlr_assert_ge!(x, 0)
/// cvlr_assert_that!(x == 5);       // expands to cvlr_assert_eq!(x, 5)
/// cvlr_assert_that!(x != 0);       // expands to cvlr_assert_ne!(x, 0)
/// ```
///
/// ## Boolean expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let flag = true;
/// let x = 5;
/// let y = 3;
///
/// cvlr_assert_that!(flag);                    // expands to cvlr_assert!(flag)
/// cvlr_assert_that!(x > 0 && y < 10);         // expands to cvlr_assert!(x > 0 && y < 10)
/// ```
///
/// ## Complex expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_that;
///
/// let a = 1;
/// let d = 4;
/// let p = 5;
/// let x = 5;
/// let y = 3;
/// let z = 10;
///
/// // Complex conditions
/// cvlr_assert_that!(a < d);                   // expands to cvlr_assert_lt!(a, d)
/// cvlr_assert_that!(x + 1 > 0 && y * 2 < z); // expands to cvlr_assert!(x + 1 > 0 && y * 2 < z)
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assertion macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assert_<op>!`,
///   which logs both operands so a counterexample shows their values
/// - Boolean expressions expand to `cvlr_assert!`
///
/// # Guarded conditions
///
/// `if guard { cond }` and `if guard { cond } else { cond2 }` (including
/// `else if` chains and nesting) expand to the same `if` with the check in
/// each branch, i.e. the condition is checked only on the path where its
/// branch is taken. Each branch must contain exactly one expression.
/// Conditions inside a branch are checked with plain `cvlr_assert!`, so their
/// operands are not logged.
///
/// # Details
///
/// * Only a top-level comparison is split: `(a < b)` is recognized through
///   parentheses, but `a < b && c < d` is a single `cvlr_assert!` of the whole
///   expression. To check (and log) the parts separately, use
///   [`cvlr_assert_all!`] with one condition per part.
/// * The literal `true` expands to `()`, i.e. nothing is checked.
/// * The expansion is a statement and refers to `::cvlr::asserts::...` paths,
///   so the calling crate must depend on `cvlr`.
#[proc_macro]
pub fn cvlr_assert_that(input: TokenStream) -> TokenStream {
    assert_that::assert_that_impl(input)
}

/// Assert multiple conditions using the same DSL syntax as `cvlr_assert_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_assert_that!`)
/// and expands to multiple calls to `cvlr_assert_that!`. Expressions can be
/// separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_assert_all!(expr1, expr2, expr3);`
/// - `cvlr_assert_all!(expr1; expr2; expr3);`
/// - `cvlr_assert_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as [`cvlr_assert_that!`], including
/// guarded `if` conditions. Each becomes a separate assertion, so a
/// counterexample identifies which condition failed. An empty list expands to
/// nothing.
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assert_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple assertions
/// cvlr_assert_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// cvlr_assert_all!(x > 0; y < 20; x < y);
///
/// // Mixed separators
/// cvlr_assert_all!(x > 0, y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assertion macros (not to `cvlr_assert_that!` calls):
///
/// ```text
/// // Input:
/// cvlr_assert_all!(x > 0, x < y);
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assert_gt!(x, 0);
/// ::cvlr::asserts::cvlr_assert_lt!(x, y);
/// ```
#[proc_macro]
pub fn cvlr_assert_all(input: TokenStream) -> TokenStream {
    assert_that::assert_all_impl(input)
}

/// Assume a condition using a DSL syntax (analogous to `cvlr_assert_that!`)
///
/// This macro provides the same DSL syntax as `cvlr_assert_that!` but expands to
/// `cvlr_assume_*` macros instead of `cvlr_assert_*` macros.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_assume_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ## Comparisons
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_that;
///
/// let x = 5;
/// let y = 10;
///
/// cvlr_assume_that!(x < y);        // expands to cvlr_assume_lt!(x, y)
/// cvlr_assume_that!(x <= y);       // expands to cvlr_assume_le!(x, y)
/// cvlr_assume_that!(x > 0);        // expands to cvlr_assume_gt!(x, 0)
/// cvlr_assume_that!(x >= 0);       // expands to cvlr_assume_ge!(x, 0)
/// cvlr_assume_that!(x == 5);       // expands to cvlr_assume_eq!(x, 5)
/// cvlr_assume_that!(x != 0);       // expands to cvlr_assume_ne!(x, 0)
/// ```
///
/// ## Boolean expressions
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_that;
///
/// let flag = true;
/// let x = 5;
/// let y = 3;
///
/// cvlr_assume_that!(flag);                    // expands to cvlr_assume!(flag)
/// cvlr_assume_that!(x > 0 && y < 10);         // expands to cvlr_assume!(x > 0 && y < 10)
/// ```
///
/// # Expansion
///
/// The macro automatically detects comparison operators and expands to the
/// appropriate assume macro:
///
/// - Comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`) expand to `cvlr_assume_<op>!`,
///   which logs both operands
/// - Boolean expressions expand to `cvlr_assume!`
///
/// # Guarded conditions
///
/// `if guard { cond }` and `if guard { cond } else { cond2 }` (including
/// `else if` chains and nesting) expand to the same `if` with the assumption in
/// each branch, i.e. the condition is assumed only on the path where its
/// branch is taken. Each branch must contain exactly one expression.
/// Conditions inside a branch are assumed with plain `cvlr_assume!`, so their
/// operands are not logged.
///
/// # Details
///
/// * Only a top-level comparison is split: `(a < b)` is recognized through
///   parentheses, but `a < b && c < d` is a single `cvlr_assume!` of the whole
///   expression. To assume (and log) the parts separately, use
///   [`cvlr_assume_all!`] with one condition per part.
/// * The literal `true` expands to `()`, i.e. nothing is assumed.
/// * The expansion is a statement and refers to `::cvlr::asserts::...` paths,
///   so the calling crate must depend on `cvlr`.
#[proc_macro]
pub fn cvlr_assume_that(input: TokenStream) -> TokenStream {
    assert_that::assume_that_impl(input)
}

/// Assume multiple conditions using the same DSL syntax as `cvlr_assume_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_assume_that!`)
/// and expands directly to the underlying `cvlr_assume_*` macros. Expressions can be
/// separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_assume_all!(expr1, expr2, expr3);`
/// - `cvlr_assume_all!(expr1; expr2; expr3);`
/// - `cvlr_assume_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as [`cvlr_assume_that!`], including
/// guarded `if` conditions. An empty list expands to nothing.
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_assume_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple assumptions
/// cvlr_assume_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// cvlr_assume_all!(x > 0; y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands directly to the underlying assume macros:
///
/// ```text
/// // Input:
/// cvlr_assume_all!(x > 0, x < y);
///
/// // Expands to:
/// ::cvlr::asserts::cvlr_assume_gt!(x, 0);
/// ::cvlr::asserts::cvlr_assume_lt!(x, y);
/// ```
#[proc_macro]
pub fn cvlr_assume_all(input: TokenStream) -> TokenStream {
    assert_that::assume_all_impl(input)
}

/// Evaluate a condition as a boolean expression using the same DSL syntax as `cvlr_assert_that!`
///
/// This macro provides the same DSL syntax as `cvlr_assert_that!` but instead of asserting,
/// it evaluates the condition as a boolean expression. The result is wrapped in its own scope.
///
/// # Syntax
///
/// The macro accepts:
/// - **Expression**: `cvlr_eval_that!(condition)`
///
/// The `condition` can be:
/// - A comparison: `a < b`, `x >= y`, `p == q`, etc.
/// - A boolean expression: `flag`, `x > 0 && y < 10`, etc.
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_eval_that;
///
/// let x = 5;
/// let y = 10;
///
/// let result = cvlr_eval_that!(x < y);        // expands to: { x < y }
/// let flag = cvlr_eval_that!(x > 0 && y < 20); // expands to: { x > 0 && y < 20 }
/// ```
///
/// # Expansion
///
/// Expressions expand to `{ condition }`: the condition is used as a plain
/// Rust expression, with no comparison splitting and no logging. It must have
/// type `bool`, so a guarded condition needs an `else` branch
/// (`if guard { cond } else { true }`).
#[proc_macro]
pub fn cvlr_eval_that(input: TokenStream) -> TokenStream {
    assert_that::eval_that_impl(input)
}

/// Evaluate multiple conditions as a boolean expression using the same DSL syntax as `cvlr_eval_that!`
///
/// This macro takes a list of DSL expressions (same syntax as `cvlr_eval_that!`)
/// and evaluates them all, aggregating the results with `&&`. The result is wrapped in its own scope.
/// Expressions can be separated by either commas (`,`) or semicolons (`;`).
///
/// # Syntax
///
/// Expressions can be separated by commas or semicolons:
/// - `cvlr_eval_all!(expr1, expr2, expr3);`
/// - `cvlr_eval_all!(expr1; expr2; expr3);`
/// - `cvlr_eval_all!(expr1, expr2; expr3);`  // Mixed separators are also allowed
///
/// Each expression follows the same syntax as [`cvlr_eval_that!`]; each must be
/// a `bool`, so a guarded condition needs an `else` branch. Conditions are
/// evaluated left to right and combined with `&&`, so evaluation stops at the
/// first `false` one. An empty list evaluates to `true`.
///
/// # Examples
///
/// ```rust,no_run
/// use cvlr_macros::cvlr_eval_all;
///
/// let x = 5;
/// let y = 10;
///
/// // Multiple expressions
/// let result = cvlr_eval_all!(x > 0, y < 20, x < y);
///
/// // Using semicolons
/// let result = cvlr_eval_all!(x > 0; y < 20; x < y);
/// ```
///
/// # Expansion
///
/// This macro expands to a block that evaluates each expression and aggregates with `&&`:
///
/// ```text
/// // Input:
/// cvlr_eval_all!(x > 0, y > 0);
///
/// // Expands to:
/// {
///     let mut __cvlr_eval_all_res = true;
///     __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
///     __cvlr_eval_all_res = __cvlr_eval_all_res && { y > 0 };
///     __cvlr_eval_all_res
/// }
/// ```
#[proc_macro]
pub fn cvlr_eval_all(input: TokenStream) -> TokenStream {
    assert_that::eval_all_impl(input)
}

/// Generate a rule name and call `cvlr_impl_rule!` macro
///
/// This macro takes a name, spec expression, and base function identifier,
/// and generates a call to `cvlr_impl_rule!` with a combined rule name.
///
/// `cvlr_impl_rule!` is not provided by CVLR: the calling crate must define or
/// import a macro of that name, taking
/// `(rule_name:ident, spec:expr, base:ident)`, that expands to the rule
/// (normally a `#[rule]` function that calls `base` with `spec`). Specs usually
/// use this through `cvlr::spec::cvlr_rules!` or
/// `cvlr::spec::cvlr_invariant_rules!`, whose documentation describes that
/// contract.
///
/// The rule name is `<name>_<base>`, where `name` is lowercased with every
/// character other than a letter, digit or `_` replaced by `_` (runs
/// collapsed), and a leading `base_` is removed from `base`.
///
/// # Syntax
///
/// ```ignore
/// cvlr_rule_for_spec! {
///     name: "rule_name",
///     spec: MySpec,
///     base: base_function_name,
/// }
/// ```
///
/// # Parameters
///
/// * `name`: A string literal that will be converted to snake_case
/// * `spec`: An instance implementing `CvlrSpec`
/// * `base`: An identifier of a function (if it starts with `base_`, that prefix is stripped)
///
/// # Examples
///
/// ```ignore
/// use cvlr_macros::cvlr_rule_for_spec;
///
/// cvlr_rule_for_spec! {
///     name: "solvency",
///     spec: MySpec,
///     base: base_update_exchange_price_no_interest_free_new,
/// }
///
/// // Expands to:
/// // cvlr_impl_rule!{solvency_update_exchange_price_no_interest_free_new, MySpec, base_update_exchange_price_no_interest_free_new}
/// ```
#[proc_macro]
pub fn cvlr_rule_for_spec(input: TokenStream) -> TokenStream {
    rule_for_spec::cvlr_rule_for_spec_impl(input)
}

/// Convert a `cvlr::predicate` annotated function name to `CvlrPredicate`
///
/// The macro is used to adapt function that define predicates into instance of
/// `CvlrPredicate`.
///
/// # Syntax
///
/// ```ignore
/// cvlr_pif!(identifier_name)
/// ```
///
/// # Examples
///
/// Technically, the macro converts snake_case identifiers to PascalCase:
///
/// - `cvlr_pif!(my_struct)` expands to `MyStruct`
/// - `cvlr_pif!(x_gt_zero)` expands to `XGtZero`
/// - `cvlr_pif!(some_long_name)` expands to `SomeLongName`
///
/// # Technical Details
///
/// A `CvlrPredicate` is a trait that is implemented by a struct. It provides
/// both the `CvlrPredicate` marker and requires implementation of `CvlrFormula`.
/// The most convenient way to define a predicate is via a Rust function that is
/// annotated with `cvlr::predicate` attribute.
/// This macro converts the function name to the struct name by converting
/// snake_case to PascalCase.
///
/// Thus, the macro returns a struct name that implements `CvlrPredicate`.
/// Since the struct is a unit struct, the name is also a value of that type.
/// Also available as `cvlr::pif!`.
///
/// # Examples
///
/// ```ignore
/// use cvlr::prelude::*;
///
/// #[cvlr::predicate]
/// fn vault_solvent(c: &VaultCtx) {
///     c.assets >= c.liabilities;
/// }
///
/// let spec = cvlr_spec! {
///     requires: cvlr::pif!(vault_solvent),
///     ensures: cvlr::pif!(vault_solvent),
/// };
/// ```
#[proc_macro]
pub fn cvlr_pif(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    // Convert snake_case to PascalCase
    let pascal_case = predicate::to_pascal_case(&ident.to_string());

    // Create new identifier with the same span as the input
    let new_ident = Ident::new(&pascal_case, ident.span());

    quote! { #new_ident }.into()
}
