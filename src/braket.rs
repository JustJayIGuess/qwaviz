use std::{
    ops::{Add, Mul, Neg, Sub},
    sync::Arc,
};

#[cfg(feature = "par_braket")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    domains::{Domain, DomainSection},
    fields::Field,
    signatures::WFSignature,
    vectorspaces::VectorSpace,
};

pub trait Wavefunction<S: WFSignature> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out;
}

pub trait Ket<S: WFSignature>: VectorSpace<S::Out> {
    type Bra: Bra<S>;
    fn adjoint(self) -> Self::Bra;
    fn norm_sqr(&self, t: S::Time) -> S::Out;
}

pub trait Bra<S: WFSignature>: VectorSpace<S::Out> {
    type Ket: Ket<S>;
    fn apply(self, ket: Self::Ket, t: S::Time) -> S::Out;
}

#[derive(Clone)]
pub enum KetOperation<S: WFSignature> {
    Function {
        a: Arc<dyn Fn(S::Space, S::Time) -> S::Out + Send + Sync>,
    },
    Add {
        a: Box<KetOperation<S>>,
        b: Box<KetOperation<S>>,
    },
    Sub {
        a: Box<KetOperation<S>>,
        b: Box<KetOperation<S>>,
    },
    Mul {
        a: S::Out,
        b: Box<KetOperation<S>>,
    },
    Neg {
        a: Box<KetOperation<S>>,
    },
    Adjoint {
        a: Box<KetOperation<S>>,
    },
}

impl<S: WFSignature> KetOperation<S> {
    fn eval(&self, x: S::Space, t: S::Time) -> S::Out {
        match self {
            KetOperation::Function { a } => a(x, t),
            KetOperation::Add { a, b } => a.eval(x.clone(), t.clone()) + b.eval(x, t),
            KetOperation::Sub { a, b } => a.eval(x.clone(), t.clone()) - b.eval(x, t),
            KetOperation::Mul { a, b } => a.clone() * b.eval(x, t),
            KetOperation::Neg { a } => -a.eval(x, t),
            KetOperation::Adjoint { a } => a.eval(x, t).conjugate(),
        }
    }
}

#[derive(Clone)]
pub struct WFKet<S>
where
    S: WFSignature,
{
    pub operation: KetOperation<S>,
    pub domain: S::Dom,
}

#[derive(Clone)]
pub struct WFBra<S>
where
    S: WFSignature,
{
    pub operation: KetOperation<S>,
    pub domain: S::Dom,
}

impl<S: WFSignature> Wavefunction<S> for WFKet<S> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out {
        if self.domain.contains(x.clone()) {
            self.operation.eval(x, t)
        } else {
            S::Out::zero()
        }
    }
}

impl<S: WFSignature> Wavefunction<S> for WFBra<S> {
    fn f(&self, x: S::Space, t: S::Time) -> S::Out {
        if self.domain.contains(x.clone()) {
            self.operation.eval(x, t)
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
            operation: KetOperation::Add {
                a: Box::new(self.operation),
                b: Box::new(rhs.operation),
            },
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
            operation: KetOperation::Sub {
                a: Box::new(self.operation),
                b: Box::new(rhs.operation),
            },
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
            operation: KetOperation::Neg {
                a: Box::new(self.operation),
            },
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
            operation: KetOperation::Add {
                a: Box::new(self.operation),
                b: Box::new(rhs.operation),
            },
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
            operation: KetOperation::Sub {
                a: Box::new(self.operation),
                b: Box::new(rhs.operation),
            },
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
            operation: KetOperation::Neg {
                a: Box::new(self.operation),
            },
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
            .unwrap_or_else(|| S::Out::zero())
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
            operation: KetOperation::Adjoint {
                a: Box::new(self.operation),
            },
            domain: self.domain,
        }
    }

    fn norm_sqr(&self, t: S::Time) -> S::Out {
        self.clone().adjoint().apply(self.clone(), t)
    }
}
