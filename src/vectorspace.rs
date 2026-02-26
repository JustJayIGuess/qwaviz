use std::ops::{Add, Div, Mul, Neg, Sub};

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

pub trait VectorSpace<F: Field>:
    Clone
    + Add<Output = Self>
    + Add<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn scale(self, c: F) -> Self;
}
