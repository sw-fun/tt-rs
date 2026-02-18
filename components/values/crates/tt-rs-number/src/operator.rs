//! Arithmetic operators and rational arithmetic for Number widgets.

/// The arithmetic operator displayed on a number widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArithOperator {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl ArithOperator {
    /// Returns the symbol for this operator.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
        }
    }
}

// === Rational Arithmetic ===

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Reduces a fraction to lowest terms.
pub(crate) fn reduce(num: i64, den: u64) -> (i64, u64) {
    let g = gcd(num.unsigned_abs(), den);
    (num / g as i64, den / g)
}

pub(crate) fn add(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    let num = n1 * d2 as i64 + n2 * d1 as i64;
    reduce(num, d1 * d2)
}

pub(crate) fn subtract(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    let num = n2 * d1 as i64 - n1 * d2 as i64;
    reduce(num, d1 * d2)
}

pub(crate) fn multiply(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    reduce(n1 * n2, d1 * d2)
}

pub(crate) fn divide(n1: i64, d1: u64, n2: i64, d2: u64) -> Option<(i64, u64)> {
    if n1 == 0 {
        return None;
    }
    let sign = if n1 < 0 { -1 } else { 1 };
    let num = n2 * d1 as i64 * sign;
    let den = d2 * n1.unsigned_abs();
    Some(reduce(num, den))
}

/// Modulo operation: n2 % n1 (target % dropped)
pub(crate) fn modulo(n1: i64, d1: u64, n2: i64, d2: u64) -> Option<(i64, u64)> {
    if n1 == 0 {
        return None; // Cannot mod by zero
    }
    // For integers (denominators are 1): n2 % n1
    // For rationals: convert to common denominator, then mod numerators
    let common_den = d1 * d2;
    let num1 = n1 * d2 as i64;
    let num2 = n2 * d1 as i64;
    let result_num = num2 % num1;
    Some(reduce(result_num, common_den))
}
