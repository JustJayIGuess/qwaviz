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
    fn zero() -> Self;
    /// Scale this vector by a scalar
    #[must_use]
    fn scale(self, c: F) -> Self;
    /// Sum many vectors together
    fn sum(vectors: Vec<Self>) -> Self;
    /// Sum many vectors together with given weights
    fn weighted_sum(summands: Vec<(F, Self)>) -> Self;
}

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            wavefunction: WFOperation::Function(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFKet {
            wavefunction: WFOperation::Mul(c, Box::new(self.wavefunction)),
            subdomain: self.subdomain,
        }
    }

    fn sum(vectors: Vec<Self>) -> Self {
        WFKet {
            wavefunction: WFOperation::Sum(
                vectors.iter().map(|v| v.wavefunction.clone()).collect(),
            ),
            subdomain: vectors
                .iter()
                .map(|v| v.subdomain.clone())
                .reduce(|a, b| a + b)
                .unwrap_or_else(S::SubDom::none),
        }
    }

    fn weighted_sum(summands: Vec<(S::Out, Self)>) -> Self {
        WFKet {
            wavefunction: WFOperation::WeightedSum(
                summands
                    .iter()
                    .map(|(c, v)| (c.clone(), v.wavefunction.clone()))
                    .collect(),
            ),
            subdomain: summands
                .iter()
                .map(|(_, v)| v.subdomain.clone())
                .reduce(|a, b| a + b)
                .unwrap_or_else(S::SubDom::none),
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
            subdomain: S::SubDom::none(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFBra {
            wavefunction: WFOperation::Mul(c, Box::new(self.wavefunction)),
            subdomain: self.subdomain,
        }
    }

    fn sum(vectors: Vec<Self>) -> Self {
        WFBra {
            wavefunction: WFOperation::Sum(
                vectors.iter().map(|v| v.wavefunction.clone()).collect(),
            ),
            subdomain: vectors
                .iter()
                .map(|v| v.subdomain.clone())
                .reduce(|a, b| a + b)
                .unwrap_or_else(S::SubDom::none),
        }
    }

    fn weighted_sum(summands: Vec<(S::Out, Self)>) -> Self {
        WFBra {
            wavefunction: WFOperation::WeightedSum(
                summands
                    .iter()
                    .map(|(c, v)| (c.clone(), v.wavefunction.clone()))
                    .collect(),
            ),
            subdomain: summands
                .iter()
                .map(|(_, v)| v.subdomain.clone())
                .reduce(|a, b| a + b)
                .unwrap_or_else(S::SubDom::none),
        }
    }
}
