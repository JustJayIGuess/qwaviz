use std::{ops::{Add, Neg, Sub}, sync::Arc};

use super::super::{wavefunction::{Wavefunction, signature::WFSignature}, core::{vectorspace::VectorSpace, field::Field, domain::{SubDomain}}};
use super::{WFOperation, Ket, WFBra, Bra};

/// A ket (vector) holding a wavefunction
#[derive(Clone)]
pub struct WFKet<S>
where
    S: WFSignature,
{
    /// The wavefunction underlying this ket
    pub wavefunction: WFOperation<S>,
    /// The subset of the domain where this ket is defined.
    pub subdomain: S::SubDom,
}

impl<S: WFSignature> Default for WFKet<S> {
    fn default() -> Self {
        Self {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::none(),
        }
    }
}

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            wavefunction: WFOperation::func(Arc::new(|_, _| S::Out::zero())),
            subdomain: S::SubDom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFKet {
            wavefunction: WFOperation::scale(c, self.wavefunction),
            subdomain: self.subdomain,
        }
    }

    fn sum(vectors: Vec<Self>) -> Self {
        WFKet {
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
        WFKet {
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


impl<S: WFSignature> Wavefunction<S> for WFKet<S> {
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

impl<S> Add for WFKet<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WFKet {
            wavefunction: self.wavefunction + rhs.wavefunction,
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Sub for WFKet<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        WFKet {
            wavefunction: self.wavefunction - rhs.wavefunction,
            #[allow(clippy::suspicious_arithmetic_impl)]
            subdomain: self.subdomain + rhs.subdomain,
        }
    }
}

impl<S> Neg for WFKet<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        WFKet {
            wavefunction: -self.wavefunction,
            subdomain: self.subdomain,
        }
    }
}

impl<S> Ket<S> for WFKet<S>
where
    S: WFSignature,
{
    type Bra = WFBra<S>;

    fn to_adjoint(self) -> Self::Bra {
        Self::Bra {
            wavefunction: WFOperation::adjoint(self.wavefunction),
            subdomain: self.subdomain,
        }
    }

    fn norm_sqr(&self, t: S::Time) -> S::Out {
        Self::adjoint(self).apply(self, t)
    }

    fn adjoint(ket: &Self) -> Self::Bra {
        Self::Bra {
            wavefunction: WFOperation::adjoint(ket.wavefunction.clone()),
            subdomain: ket.subdomain.clone(),
        }
    }
}