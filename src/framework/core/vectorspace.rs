//! Functionality for representing abstract vector spaces

use std::ops::{Add, Neg, Sub};

use super::field::Field;

/// Trait implementing properties of a vectorspace over a field
pub trait VectorSpace<F: Field>:
    Clone + Add<Output = Self> + Sub<Output = Self> + Neg<Output = Self>
{
    /// The additive identity of the vectorspace
    fn zero() -> Self;
    /// Scale this vector by a scalar
    #[must_use]
    fn scale(self, c: F) -> Self;
    /// Sum many vectors together
    fn sum(vectors: Vec<Self>) -> Self;
    /// Sum many vectors together with given weights
    fn weighted_sum(summands: Vec<(F, Self)>) -> Self;
}
