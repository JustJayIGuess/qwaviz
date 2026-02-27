//! Functionality for representing mathematical fields

use std::ops::{Add, Div, Mul, Neg, Sub};

use num_complex::Complex32;

/// Trait requiring properties of a field (the mathematical object) with an involution for conjugation.
pub trait Field:
    Sized
    + Clone
    + Copy
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    /// The additive identity of the field
    #[must_use]
    fn zero() -> Self;
    /// The multiplicative identity of the field
    #[must_use]
    fn one() -> Self;
    /// The multiplicative inverse of this element
    fn inv(&self) -> Option<Self>;
    /// Check if element is the zero of the field
    fn is_zero(&self) -> bool;
    /// Take conjugate of the element.
    #[must_use]
    fn conjugate(self) -> Self;
}

impl Field for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(1.0 / *self)
        }
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn conjugate(self) -> Self {
        self
    }
}

impl Field for Complex32 {
    fn zero() -> Self {
        Complex32::new(0.0, 0.0)
    }

    fn one() -> Self {
        Complex32::new(1.0, 0.0)
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(self.inv())
        }
    }

    fn is_zero(&self) -> bool {
        *self == Complex32::ZERO
    }

    fn conjugate(self) -> Self {
        self.conj()
    }
}
