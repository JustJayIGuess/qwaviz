//! Functionality for representing abstract vector spaces

use std::{
    ops::{Add, Neg, Sub},
    sync::Arc,
};

use crate::{
    braket::{WFBra, WFKet, WFOperation},
    domains::SubDomain,
    fields::Field,
    signatures::WFSignature,
};

/// Trait implementing properties of a vectorspace over a field
pub trait VectorSpace<F: Field>:
    Clone + Add<Output = Self> + Sub<Output = Self> + Neg<Output = Self>
{
    /// The additive identity of the vectorspace
    #[must_use]
    fn zero() -> Self;
    /// Scale this vector by a scalar
    #[must_use]
    fn scale(self, c: F) -> Self;
}

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            wavefunction: WFOperation::Function(Arc::new(|_, _| S::Out::zero())),
            domain: S::SubDom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFKet {
            wavefunction: WFOperation::Mul(c, Box::new(self.wavefunction)),
            domain: self.domain,
        }
    }
}

impl<S> VectorSpace<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFBra {
            wavefunction: WFOperation::Function(Arc::new(|_, _| S::Out::zero())),
            domain: S::SubDom::none(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFBra {
            wavefunction: WFOperation::Mul(c, Box::new(self.wavefunction)),
            domain: self.domain,
        }
    }
}
