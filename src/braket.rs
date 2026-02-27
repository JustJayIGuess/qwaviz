//! Functionality for representing Bras and Kets (covectors and vectors).
//! Includes implementation of Bras and Kets in 1(+1)-D function spaces.

use std::{
    ops::{Add, Mul, Neg, Sub},
    sync::Arc,
};

#[cfg(feature = "par_braket")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    domains::{Domain, SubDomain},
    fields::Field,
    signatures::WFSignature,
    vectorspaces::VectorSpace,
};

type WFFunc<S> = dyn Fn(<S as WFSignature>::Space, <S as WFSignature>::Time) -> <S as WFSignature>::Out
    + Send
    + Sync;

/// Require ability to evaluate a wavefunction at points in domain
pub trait Wavefunction<S: WFSignature> {
    /// Evaluate the wavefunction at a point in space and time
    fn f(&self, x: S::Space, t: S::Time) -> S::Out;
}

/// A ket (vector) in a function vectorspace
pub trait Ket<S: WFSignature>: VectorSpace<S::Out> {
    /// The corresponding bra (covector) type
    type Bra: Bra<S>;
    /// Convert to corresponding bra (covector)
    fn adjoint(self) -> Self::Bra;
    /// Compute the squared norm using the standard inner product
    fn norm_sqr(&self, t: S::Time) -> S::Out;
}

/// A bra (covector) in the dual of a function vectorspace
pub trait Bra<S: WFSignature>: VectorSpace<S::Out> {
    /// The corresponding ket (vector) type
    type Ket: Ket<S>;
    /// Apply this bra (covector) to a ket (vector) to produce an element of the field.
    fn apply(self, ket: Self::Ket, t: S::Time) -> S::Out;
}

/// Operations that can be done on the wavefunctions underlying bras (covectors) and kets (vectors)
#[derive(Clone)]
pub enum WFOperation<S: WFSignature> {
    /// A constant in the function space (i.e., a function from (Space x Time) --> Out)
    Function(Arc<WFFunc<S>>),
    /// Add two wavefunctions pointwise
    Add(Box<WFOperation<S>>, Box<WFOperation<S>>),
    /// Subtract two wavefunctions pointwise
    Sub(Box<WFOperation<S>>, Box<WFOperation<S>>),
    /// Scale a wavefunction by a scalar
    Mul(S::Out, Box<WFOperation<S>>),
    /// Negate a wavefunction pointwise
    Neg(Box<WFOperation<S>>),
    /// Take the adjoin of a wavefunction (conjugate pointwise)
    Adjoint(Box<WFOperation<S>>),
}

impl<S: WFSignature> WFOperation<S> {
    fn eval(&self, x: S::Space, t: S::Time) -> S::Out {
        match self {
            WFOperation::Function(f) => f(x, t),
            WFOperation::Add(f, g) => f.eval(x.clone(), t.clone()) + g.eval(x, t),
            WFOperation::Sub(a, b) => a.eval(x.clone(), t.clone()) - b.eval(x, t),
            WFOperation::Mul(a, b) => a.clone() * b.eval(x, t),
            WFOperation::Neg(a) => -a.eval(x, t),
            WFOperation::Adjoint(a) => a.eval(x, t).conjugate(),
        }
    }
}

/// A ket (vector) holding a wavefunction
#[derive(Clone)]
pub struct WFKet<S>
where
    S: WFSignature,
{
    /// The wavefunction underlying this ket
    pub wavefunction: WFOperation<S>,
    /// The subset of the domain where this ket is defined.
    pub domain: S::SubDom,
}

/// A bra (covector) holding a wavefunction
#[derive(Clone)]
pub struct WFBra<S>
where
    S: WFSignature,
{
    /// The wavefunction underlying this bra
    pub wavefunction: WFOperation<S>,
    /// The subset of the domain where this bra is defined
    pub domain: S::SubDom,
}

impl<S: WFSignature> Wavefunction<S> for WFKet<S> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out {
        if self.domain.contains(x.clone()) {
            self.wavefunction.eval(x, t)
        } else {
            S::Out::zero()
        }
    }
}

impl<S: WFSignature> Wavefunction<S> for WFBra<S> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out {
        if self.domain.contains(x.clone()) {
            self.wavefunction.eval(x, t)
        } else {
            S::Out::zero()
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
            wavefunction: WFOperation::Add(Box::new(self.wavefunction), Box::new(rhs.wavefunction)),
            domain: self.domain + rhs.domain,
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
            wavefunction: WFOperation::Sub(Box::new(self.wavefunction), Box::new(rhs.wavefunction)),
            #[allow(clippy::suspicious_arithmetic_impl)]
            domain: self.domain + rhs.domain,
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
            wavefunction: WFOperation::Neg(Box::new(self.wavefunction)),
            domain: self.domain,
        }
    }
}

impl<S> Add for WFBra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WFBra {
            wavefunction: WFOperation::Add(Box::new(self.wavefunction), Box::new(rhs.wavefunction)),
            domain: self.domain + rhs.domain,
        }
    }
}

impl<S> Sub for WFBra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        WFBra {
            wavefunction: WFOperation::Sub(Box::new(self.wavefunction), Box::new(rhs.wavefunction)),
            #[allow(clippy::suspicious_arithmetic_impl)]
            domain: self.domain + rhs.domain,
        }
    }
}

impl<S> Neg for WFBra<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        WFBra {
            wavefunction: WFOperation::Neg(Box::new(self.wavefunction)),
            domain: self.domain,
        }
    }
}

impl<S: WFSignature> Mul<WFKet<S>> for WFBra<S> {
    type Output = S::Out;

    fn mul(self, rhs: WFKet<S>) -> Self::Output {
        self.apply(rhs, S::Time::zero())
    }
}

impl<S> Bra<S> for WFBra<S>
where
    S: WFSignature,
{
    type Ket = WFKet<S>;

    #[cfg(not(feature = "par_braket"))]
    fn apply(self, ket: Self::Ket, t: S::Time) -> S::Out {
        let domain = ket.domain.clone() * self.domain.clone();
        domain
            .iter()
            .map(|x| {
                S::mul_to_codomain(
                    domain.step_size(),
                    self.f(x.clone(), t.clone()) * ket.f(x, t.clone()),
                )
            })
            .reduce(|a, b| a + b)
            .unwrap_or_else(S::Out::zero)
    }

    #[cfg(feature = "par_braket")]
    fn apply(self, ket: Self::Ket, t: S::Time) -> S::Out {
        let domain = ket.domain.clone() * self.domain.clone();
        domain
            .iter()
            .par_bridge()
            .map(|x| {
                S::mul_to_codomain(domain.step_size(), self.f(x.clone(), t.clone()))
                    * ket.f(x, t.clone())
            })
            .reduce(|| S::Out::zero(), |a, b| a + b)
    }
}

impl<S> Ket<S> for WFKet<S>
where
    S: WFSignature,
{
    type Bra = WFBra<S>;

    fn adjoint(self) -> Self::Bra {
        Self::Bra {
            wavefunction: WFOperation::Adjoint(Box::new(self.wavefunction)),
            domain: self.domain,
        }
    }

    fn norm_sqr(&self, t: S::Time) -> S::Out {
        self.clone().adjoint().apply(self.clone(), t)
    }
}
