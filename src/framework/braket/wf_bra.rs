//! Generalised functionality for wavefunction bras in Dirac's Bra-Ket formalism.

use std::{
    ops::{Add, Neg, Sub},
    sync::Arc,
};

#[cfg(feature = "par_braket")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use super::super::{
    core::{domain::SubDomain, field::Field, vectorspace::VectorSpace},
    wavefunction::{Wavefunction, signature::WFSignature},
};
use super::{AbstractBra, Ket, WFFunc, WFOperation};

/// A bra (covector) holding a wavefunction
#[derive(Clone)]
pub struct Bra<S>
where
    S: WFSignature,
{
    /// The wavefunction underlying this bra
    pub(super) wavefunction: WFOperation<S>,
    /// The subset of the domain where this bra is defined
    pub(super) subdomain: S::SubDom,
}

impl<S: WFSignature> Bra<S> {
    /// Return a new ket with the given wavefunction and subdomain
    pub fn new(f: Arc<WFFunc<S>>, subdomain: S::SubDom) -> Bra<S> {
        Bra {
            wavefunction: WFOperation::func(f),
            subdomain,
        }
    }
}

impl<S: WFSignature> Default for Bra<S> {
    fn default() -> Self {
        Self {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::none(),
        }
    }
}

impl<S> VectorSpace<S::Out> for Bra<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        Bra {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::none(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        Bra {
            wavefunction: WFOperation::scale(c, self.wavefunction),
            subdomain: self.subdomain,
        }
    }

    fn sum(vectors: Vec<Self>) -> Self {
        Bra {
            wavefunction: WFOperation::sum(
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
        Bra {
            wavefunction: WFOperation::weighted_sum(
                summands
                    .iter()
                    .map(|(c, v)| (*c, v.wavefunction.clone()))
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

impl<S: WFSignature> Wavefunction<S> for Bra<S> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out {
        if self.subdomain.contains(x) {
            self.wavefunction.eval(x, t)
        } else {
            S::Out::zero()
        }
    }

    fn p(
        &self,
        x: <S as WFSignature>::Space,
        t: <S as WFSignature>::Time,
    ) -> <S as WFSignature>::Out {
        if self.subdomain.contains(x) {
            let value = self.f(x, t);
            value.conjugate() * value
        } else {
            S::Out::zero()
        }
    }

    fn translate_space(self, offset: <S as WFSignature>::Space) -> Self {
        Self {
            wavefunction: WFOperation::translate_space(offset, self.wavefunction),
            subdomain: self.subdomain.translate(offset),
        }
    }

    fn translate_time(self, offset: <S as WFSignature>::Time) -> Self {
        Self {
            wavefunction: WFOperation::translate_time(offset, self.wavefunction),
            subdomain: self.subdomain,
        }
    }
}

impl<S> Add for Bra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Bra {
            wavefunction: self.wavefunction + rhs.wavefunction,
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Sub for Bra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Bra {
            wavefunction: self.wavefunction - rhs.wavefunction,
            #[allow(clippy::suspicious_arithmetic_impl)]
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Neg for Bra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Bra {
            wavefunction: -self.wavefunction,
            subdomain: self.subdomain,
        }
    }
}

impl<S> AbstractBra<S> for Bra<S>
where
    S: WFSignature,
{
    type Ket = Ket<S>;

    #[cfg(not(feature = "par_braket"))]
    fn apply(&self, ket: &Self::Ket, t: S::Time, step_size: S::Space) -> S::Out {
        let domain = ket.subdomain.clone() * self.subdomain.clone();
        domain
            .iter_with_step_size(step_size)
            .map(|x| S::mul_to_codomain(step_size, self.f(x, t) * ket.f(x, t)))
            .reduce(|a, b| a + b)
            .unwrap_or_else(S::Out::zero)
    }

    #[cfg(feature = "par_braket")]
    fn apply(&self, ket: &Self::Ket, t: S::Time, step_size: S::Space) -> S::Out {
        let domain = ket.subdomain.clone() * self.subdomain.clone();
        domain
            .iter_with_step_size(step_size)
            .par_bridge()
            .map(|x| S::mul_to_codomain(step_size, self.f(x, t)) * ket.f(x, t))
            .reduce(S::Out::zero, |a, b| a + b)
    }
}
