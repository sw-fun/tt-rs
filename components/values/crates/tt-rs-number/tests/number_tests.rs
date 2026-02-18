//! Integration tests for Number widget.

use tt_rs_number::{ArithOperator, Number};

#[test]
fn test_integer_creation() {
    let n = Number::new(42);
    assert_eq!(n.numerator(), 42);
    assert_eq!(n.denominator(), 1);
    assert!(n.is_integer());
}

#[test]
fn test_rational_reduction() {
    let n = Number::rational(4, 8);
    assert_eq!(n.numerator(), 1);
    assert_eq!(n.denominator(), 2);
}

#[test]
fn test_addition() {
    let a = Number::new(1);
    let b = Number::new(2);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 3);
}

#[test]
fn test_rational_addition() {
    let a = Number::rational(1, 2);
    let b = Number::rational(1, 3);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 5);
    assert_eq!(result.denominator(), 6);
}

#[test]
fn test_subtraction() {
    let a = Number::new(3).with_operator(ArithOperator::Subtract);
    let b = Number::new(10);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 7);
}

#[test]
fn test_multiplication() {
    let a = Number::new(4).with_operator(ArithOperator::Multiply);
    let b = Number::new(5);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 20);
}

#[test]
fn test_division() {
    let a = Number::new(2).with_operator(ArithOperator::Divide);
    let b = Number::new(10);
    let result = a.apply_to(&b).unwrap();
    assert_eq!(result.numerator(), 5);
}

#[test]
fn test_division_by_zero() {
    let a = Number::new(0).with_operator(ArithOperator::Divide);
    let b = Number::new(10);
    assert!(a.apply_to(&b).is_none());
}

// Tests for the mutating apply method (used in drag-drop)
#[test]
fn test_apply_addition_mutates() {
    let mut target = Number::new(10);
    let dropped = Number::new(5); // +5
    target.apply(&dropped);
    assert_eq!(target.numerator(), 15);
}

#[test]
fn test_apply_subtraction_mutates() {
    let mut target = Number::new(10);
    let dropped = Number::new(3).with_operator(ArithOperator::Subtract);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 7); // 10 - 3
}

#[test]
fn test_apply_multiplication_mutates() {
    let mut target = Number::new(4);
    let dropped = Number::new(5).with_operator(ArithOperator::Multiply);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 20); // 4 * 5
}

#[test]
fn test_apply_division_mutates() {
    let mut target = Number::new(20);
    let dropped = Number::new(4).with_operator(ArithOperator::Divide);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 5); // 20 / 4
}

#[test]
fn test_apply_division_by_zero_returns_none() {
    let mut target = Number::new(10);
    let dropped = Number::new(0).with_operator(ArithOperator::Divide);
    assert!(target.apply(&dropped).is_none());
    // Target should remain unchanged
    assert_eq!(target.numerator(), 10);
}

#[test]
fn test_apply_modulo() {
    let mut target = Number::new(17);
    let dropped = Number::new(10).with_operator(ArithOperator::Modulo);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 7); // 17 % 10 = 7
}

#[test]
fn test_apply_modulo_large_number() {
    // Simulate sensor output: 123456 % 10 = 6
    let mut target = Number::new(123456);
    let dropped = Number::new(10).with_operator(ArithOperator::Modulo);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 6);
}

#[test]
fn test_apply_modulo_by_zero_returns_none() {
    let mut target = Number::new(10);
    let dropped = Number::new(0).with_operator(ArithOperator::Modulo);
    assert!(target.apply(&dropped).is_none());
    // Target should remain unchanged
    assert_eq!(target.numerator(), 10);
}

#[test]
fn test_subtract_from_negative() {
    // Scenario: drop -1 tool on -1 value, should get -2
    let mut target = Number::new(-1);
    let dropped = Number::new(1).with_operator(ArithOperator::Subtract);
    target.apply(&dropped);
    assert_eq!(target.numerator(), -2); // -1 - 1 = -2
}

#[test]
fn test_subtract_from_zero() {
    // Drop -1 on 0 should give -1
    let mut target = Number::new(0);
    let dropped = Number::new(1).with_operator(ArithOperator::Subtract);
    target.apply(&dropped);
    assert_eq!(target.numerator(), -1); // 0 - 1 = -1
}

#[test]
fn test_chained_subtractions() {
    // Drop -1 on 0, then drop -1 on result, should give -2
    let mut target = Number::new(0);
    let dropped = Number::new(1).with_operator(ArithOperator::Subtract);

    target.apply(&dropped);
    assert_eq!(target.numerator(), -1, "First subtraction failed");

    // Create a fresh -1 tool and apply again
    let dropped2 = Number::new(1).with_operator(ArithOperator::Subtract);
    target.apply(&dropped2);
    assert_eq!(target.numerator(), -2, "Second subtraction failed");
}

#[test]
fn test_display_value_negative_zero() {
    // -0 should display as 0, not -0
    let n = Number::new(0);
    assert_eq!(n.display_value(), "0");

    // After subtraction resulting in -0 (which is 0)
    let mut target = Number::new(1);
    let dropped = Number::new(1).with_operator(ArithOperator::Subtract);
    target.apply(&dropped);
    assert_eq!(target.numerator(), 0);
    assert_eq!(target.display_value(), "0");
}

#[test]
fn test_negative_number_display() {
    let n = Number::new(-1);
    assert_eq!(n.display_value(), "-1");
    assert!(!n.is_tool()); // Add operator, so not a tool
    assert!(!n.is_copy_source());
}
