mod rt_decls {
    #[cfg_attr(
        all(target_family = "wasm", target_os = "none"),
        link(wasm_import_module = "env")
    )]
    extern "C" {
        pub fn CVT_assume(_c: bool);
        pub fn CVT_assert(_c: bool);
        pub fn CVT_satisfy(_c: bool);
        pub fn CVT_sanity(_c: bool);
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_assume(c: bool) {
        if !c {
            panic!()
        }
    }

    #[no_mangle]
    pub extern "C" fn CVT_assert(c: bool) {
        assert!(c);
    }

    #[no_mangle]
    pub extern "C" fn CVT_satisfy(c: bool) {
        assert!(c);
    }

    #[no_mangle]
    pub extern "C" fn CVT_sanity(c: bool) {
        CVT_satisfy(c)
    }
}

use rt_decls::*;

/// Asserts `c` without recording a source location.
///
/// The function behind [`cvlr_assert!`](crate::cvlr_assert); specs should use
/// the macro, which also attaches the file and line to the assertion so the
/// report links to it. See [`cvlr_assert!`](crate::cvlr_assert) for the
/// semantics.
#[inline(always)]
pub fn cvlr_assert_checked(c: bool) {
    unsafe {
        CVT_assert(c);
    }
}

/// Assumes `c`. The function behind [`cvlr_assume!`](crate::cvlr_assume), with
/// identical behavior.
#[inline(always)]
pub fn cvlr_assume_checked(c: bool) {
    unsafe {
        CVT_assume(c);
    }
}

/// Adds a reachability goal `c` without recording a source location.
///
/// The function behind [`cvlr_satisfy!`](crate::cvlr_satisfy); specs should use
/// the macro, which also attaches the file and line. See
/// [`cvlr_satisfy!`](crate::cvlr_satisfy) for the semantics.
#[inline(always)]
pub fn cvlr_satisfy_checked(c: bool) {
    unsafe {
        CVT_satisfy(c);
    }
}

/// Marks a point whose reachability the Prover's rule-sanity check verifies.
///
/// In the rule itself the call is removed and has no effect. When rule sanity
/// checking is enabled in the Prover (`--rule_sanity`, on at its `basic` level
/// by default), the Solana Prover also generates, for every rule that contains
/// no [`cvlr_satisfy!`](crate::cvlr_satisfy), a companion rule named
/// `<rule>rule_not_vacuous_cvlr` in which each `cvlr_sanity_checked(c)` becomes
/// `cvlr_satisfy!(c)` and every assertion becomes an assumption. That companion
/// rule fails if no execution reaches the call with `c` true, which reveals
/// that the original rule passed vacuously (for example because its assumptions
/// are contradictory).
///
/// `#[rule]` already appends a call with `c = true` at the end of every rule,
/// via [`cvlr_vacuity_check!`](crate::cvlr_vacuity_check); calling this
/// directly is only needed to check reachability of other points.
#[inline(always)]
pub fn cvlr_sanity_checked(c: bool) {
    unsafe {
        CVT_sanity(c);
    }
}

/// Asserts that `cond` holds: the rule fails if some execution reaches this
/// point with `cond` false.
///
/// Syntax: `cvlr_assert!(cond)` or `cvlr_assert!(cond, "description")`.
/// `cond` must be a `bool` and is evaluated once; the optional description
/// must be a string literal and is currently ignored. The file and line of the
/// macro call are attached to the assertion, so the report links a failure to
/// it.
///
/// The Prover checks every assertion of a rule over all executions that satisfy
/// the assumptions made before it; a counterexample is an execution that
/// violates one. Nothing about `cond` is logged: to see the values involved,
/// log them with `clog!` or use a comparison form such as
/// [`cvlr_assert_eq!`](crate::cvlr_assert_eq) or
/// [`cvlr_assert_le!`](crate::cvlr_assert_le), which record both operands.
///
/// In a rule that also contains [`cvlr_satisfy!`](crate::cvlr_satisfy),
/// assertions are treated as assumptions instead (see there).
///
/// With the `rt` feature this is an ordinary `assert!`.
#[macro_export]
macro_rules! cvlr_assert {
    ($cond: expr $(, $desc: literal)?) => {{
        let c_ = $cond;
        $crate::add_loc!();
        $crate::cvlr_assert_checked(c_);
    }};
}

/// Assumes that `cond` holds: executions in which it is false at this point are
/// excluded from the analysis.
///
/// Syntax: `cvlr_assume!(cond)` or `cvlr_assume!(cond, "description")`. `cond`
/// must be a `bool` and is evaluated once; the optional description must be a
/// string literal and is currently ignored. Nothing is logged; the comparison
/// forms such as [`cvlr_assume_le!`](crate::cvlr_assume_le) record their
/// operands.
///
/// Use it to state preconditions (valid inputs, account relationships, bounds).
/// Every assumption shrinks the set of executions that assertions are checked
/// against, and contradictory assumptions make every later assertion pass
/// vacuously; `#[rule]`'s built-in sanity check (see
/// [`cvlr_vacuity_check!`](crate::cvlr_vacuity_check)) and
/// [`cvlr_satisfy!`](crate::cvlr_satisfy) detect that.
///
/// With the `rt` feature this panics if `cond` is false.
#[macro_export]
macro_rules! cvlr_assume {
    ($cond: expr $(, $desc: literal)?) => {
        $crate::cvlr_assume_checked($cond)
    };
}

/// Requires that some execution reaches this point with `cond` true.
///
/// Syntax: `cvlr_satisfy!(cond)` or `cvlr_satisfy!(cond, "description")`.
/// `cond` must be a `bool` and is evaluated once; the optional description
/// must be a string literal and is currently ignored. The file and line of the
/// macro call are attached to it.
///
/// A rule that contains any `cvlr_satisfy!` is a *satisfy rule*: it passes if
/// the Prover finds an execution that satisfies all the assumptions, reaches a
/// `cvlr_satisfy!`, and has its condition true there. That execution is
/// reported as a witness, including everything logged along the way. The rule
/// fails if no such execution exists. Use it to show that a scenario is
/// possible (e.g. that a withdrawal can succeed), or that assumptions do not
/// rule out the interesting cases.
///
/// Under the Solana Prover:
///
/// * In a satisfy rule, every `cvlr_assert!` (and assertion-producing macro)
///   is treated as an assumption, so it restricts the witness instead of being
///   checked.
/// * All `cvlr_satisfy!` calls in one rule are checked together, in a single
///   query: one witness reaching any of them suffices. To show that each of
///   several scenarios is reachable, put each in its own rule.
///
/// With the `rt` feature this is an ordinary `assert!`.
#[macro_export]
macro_rules! cvlr_satisfy {
    ($cond: expr $(, $desc: literal)?) => {{
        let c_ = $cond;
        $crate::add_loc!();
        $crate::cvlr_satisfy_checked(c_);
    }};
}

/// Checks that the end of the rule is reachable. `#[rule]` appends it to every
/// rule, so specs rarely call it themselves.
///
/// By default it expands to `cvlr_sanity_checked(true)`, which the Prover's
/// rule-sanity check turns into a companion rule
/// (`<rule>rule_not_vacuous_cvlr`) that fails if no execution reaches this
/// point, i.e. if the rule passed vacuously; see [`cvlr_sanity_checked`].
///
/// With the `vacuity` feature it expands to `cvlr_assert!(false)` instead, the
/// older manual form of the same check: a rule whose end is reachable then
/// *fails*, and a rule that still passes is vacuous.
#[cfg(feature = "vacuity")]
#[macro_export]
macro_rules! cvlr_vacuity_check {
    () => {
        $crate::cvlr_assert!(false)
    };
}

/// Checks that the end of the rule is reachable. `#[rule]` appends it to every
/// rule, so specs rarely call it themselves.
///
/// By default it expands to `cvlr_sanity_checked(true)`, which the Prover's
/// rule-sanity check turns into a companion rule
/// (`<rule>rule_not_vacuous_cvlr`) that fails if no execution reaches this
/// point, i.e. if the rule passed vacuously; see [`cvlr_sanity_checked`].
///
/// With the `vacuity` feature it expands to `cvlr_assert!(false)` instead, the
/// older manual form of the same check: a rule whose end is reachable then
/// *fails*, and a rule that still passes is vacuous.
#[cfg(not(feature = "vacuity"))]
#[macro_export]
macro_rules! cvlr_vacuity_check {
    () => {
        $crate::cvlr_sanity_checked(true)
    };
}
