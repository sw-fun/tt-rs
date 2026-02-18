//! Builder methods and display functions for Number.

use crate::number::{ErasureLevel, Number};
use crate::{operator, ArithOperator};
use tt_rs_core::WidgetId;

impl Number {
    /// Sets the arithmetic operator (builder pattern).
    pub fn with_operator(mut self, op: ArithOperator) -> Self {
        self.operator = op;
        self
    }

    /// Sets this number as a copy source (builder pattern).
    pub fn as_copy_source(mut self) -> Self {
        self.is_copy_source = true;
        self
    }

    /// Returns the display value as a string.
    pub fn display_value(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "?".to_string()
        } else if self.is_integer() {
            self.numerator.to_string()
        } else {
            format!("{}/{}", self.numerator, self.denominator)
        }
    }

    /// Creates a copy with a new ID.
    pub fn copy_number(&self) -> Number {
        Number {
            id: WidgetId::new(),
            numerator: self.numerator,
            denominator: self.denominator,
            operator: self.operator,
            erasure: self.erasure,
            is_copy_source: false,
        }
    }

    /// Applies operator to create a new number.
    pub fn apply_to(&self, other: &Number) -> Option<Number> {
        let (n, d) = apply_op(self.operator, self, other)?;
        Some(Number::rational(n, d))
    }

    /// Apply another number using the dropped number's operator.
    /// After arithmetic, the target becomes a plain number (operator reset to Add).
    pub fn apply(&mut self, dropped: &Number) -> Option<()> {
        let (new_num, new_den) = apply_op(dropped.operator, dropped, self)?;
        self.numerator = new_num;
        self.denominator = new_den;
        // Reset operator to Add - the result is a plain number, not a tool
        self.operator = ArithOperator::Add;
        Some(())
    }
}

fn apply_op(op: ArithOperator, dropped: &Number, target: &Number) -> Option<(i64, u64)> {
    // The dropped number's operator determines the operation.
    // The dropped number's raw numerator is the operand value.
    // The target's effective value is used (accounts for Subtract tools).
    let dropped_num = dropped.numerator;
    let target_num = target.effective_numerator();

    match op {
        ArithOperator::Add => Some(operator::add(
            dropped_num,
            dropped.denominator,
            target_num,
            target.denominator,
        )),
        ArithOperator::Subtract => Some(operator::subtract(
            dropped_num,
            dropped.denominator,
            target_num,
            target.denominator,
        )),
        ArithOperator::Multiply => Some(operator::multiply(
            dropped_num,
            dropped.denominator,
            target_num,
            target.denominator,
        )),
        ArithOperator::Divide => operator::divide(
            dropped_num,
            dropped.denominator,
            target_num,
            target.denominator,
        ),
        ArithOperator::Modulo => operator::modulo(
            dropped_num,
            dropped.denominator,
            target_num,
            target.denominator,
        ),
    }
}
