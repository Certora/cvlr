//! Test file to verify doctest examples compile correctly

extern crate cvlr;

use cvlr_spec::spec::CvlrLemma;
use cvlr_spec::*;

// Test the cvlr_predicate example
#[test]
fn test_cvlr_predicate_example() {
    use cvlr_spec::cvlr_predicate;

    struct Counter {
        value: i32,
    }

    let ctx = Counter { value: 5 };

    // Create an anonymous predicate
    let pred = cvlr_predicate! { | c : Counter | -> {
        c.value > 0;
        c.value < 100
    } };

    assert!(pred.eval(&ctx));
}

// Test the cvlr_lemma example (simplified - can't call verify() without Nondet/CvlrLog)
#[test]
fn test_cvlr_lemma_example_syntax() {
    use cvlr_spec::cvlr_lemma;

    #[derive(cvlr::derive::Nondet, cvlr::derive::CvlrLog)]
    struct Counter {
        value: i32,
    }

    // Define a lemma: if value > 0, then value > 0 (trivial but demonstrates syntax)
    cvlr_lemma! {
        CounterPositiveLemma(c: Counter) {
            requires -> {
                c.value > 0
            }
            ensures -> {
                c.value > 0
            }
        }
    }

    // Use the lemma
    let lemma = CounterPositiveLemma;
    let ctx = Counter { value: 5 };

    // Test that requires and ensures work
    assert!(lemma.requires().eval(&ctx));
    assert!(lemma.ensures().eval(&ctx));
}

#[test]
fn test_cvlr_lemma_complex_example() {
    use cvlr_spec::cvlr_lemma;

    #[derive(cvlr::derive::Nondet, cvlr::derive::CvlrLog)]
    struct Counter {
        value: i32,
    }

    cvlr_lemma! {
        CounterDoublesLemma(c: Counter) {
            requires -> {
                c.value > 0;
                c.value < 100
            }
            ensures -> {
                c.value > 0;
                c.value * 2 > c.value
            }
        }
    }

    let lemma = CounterDoublesLemma;
    let ctx = Counter { value: 5 };

    assert!(lemma.requires().eval(&ctx));
    assert!(lemma.ensures().eval(&ctx));
}
