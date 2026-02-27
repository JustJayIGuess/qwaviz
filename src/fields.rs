use std::ops::{Add, Div, Mul, Neg, Sub};

use num_complex::Complex32;

pub trait Field:
    Sized
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn inv(&self) -> Option<Self>;
    fn is_zero(&self) -> bool;
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
