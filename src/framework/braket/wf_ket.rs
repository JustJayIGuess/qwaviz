//! Generalised functionality for wavefunction kets in Dirac's Bra-Ket formalism.

use std::{
    ops::{Add, Neg, Sub},
    sync::Arc,
};

use super::super::{
    core::{domain::SubDomain, field::Field, vectorspace::VectorSpace},
    wavefunction::{Wavefunction, signature::WFSignature},
};
use super::{AbstractBra, AbstractKet, Bra, WFFunc, WFOperation};

/// A ket (vector) holding a wavefunction
#[derive(Clone)]
pub struct Ket<S>
where
    S: WFSignature,
{
    /// The wavefunction underlying this ket
    pub(super) wavefunction: WFOperation<S>,
    /// The subset of the domain where this ket is defined.
    pub subdomain: S::SubDom,
}

impl<S: WFSignature> Ket<S> {
    /// Return a new ket with the given wavefunction and subdomain
    pub fn new(f: Arc<WFFunc<S>>, subdomain: S::SubDom) -> Ket<S> {
        Ket {
            wavefunction: WFOperation::func(f),
            subdomain,
        }
    }

    /// Return a new ket with the given ('static) wavefunction and subdomain
    pub fn new_static<F: Fn(S::Space, S::Time) -> S::Out + 'static + Send + Sync>(
        f: F,
        subdomain: S::SubDom,
    ) -> Ket<S> {
        Self::new(Arc::new(f), subdomain)
    }

    /// Iterate over the domain of the ket with the given `step_size`
    pub fn iter_with_step_size(
        &self,
        step_size: S::Space,
    ) -> impl Iterator<Item = S::Space> + Sized + Send + Sync {
        self.subdomain.iter_with_step_size(step_size)
    }
}

impl<S: WFSignature> Default for Ket<S> {
    fn default() -> Self {
        Self {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::none(),
        }
    }
}

impl<S> VectorSpace<S::Out> for Ket<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        Ket {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        Ket {
            wavefunction: WFOperation::scale(c, self.wavefunction),
            subdomain: self.subdomain,
        }
    }

    fn sum(vectors: Vec<Self>) -> Self {
        Ket {
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
        Ket {
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

impl<S: WFSignature> Wavefunction<S> for Ket<S> {
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

impl<S> Add for Ket<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ket {
            wavefunction: self.wavefunction + rhs.wavefunction,
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Sub for Ket<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Ket {
            wavefunction: self.wavefunction - rhs.wavefunction,
            #[allow(clippy::suspicious_arithmetic_impl)]
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Neg for Ket<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Ket {
            wavefunction: -self.wavefunction,
            subdomain: self.subdomain,
        }
    }
}

impl<S> AbstractKet<S> for Ket<S>
where
    S: WFSignature,
{
    type Bra = Bra<S>;

    fn to_adjoint(self) -> Self::Bra {
        Self::Bra {
            wavefunction: WFOperation::adjoint(self.wavefunction),
            subdomain: self.subdomain,
        }
    }

    fn norm_sqr(&self, t: S::Time, step_size: S::Space) -> S::Out {
        Self::adjoint(self).apply(self, t, step_size)
    }

    fn adjoint(ket: &Self) -> Self::Bra {
        Self::Bra {
            wavefunction: WFOperation::adjoint(ket.wavefunction.clone()),
            subdomain: ket.subdomain.clone(),
        }
    }
}
