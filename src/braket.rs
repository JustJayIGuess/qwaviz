use std::{
    ops::{Add, Mul, Neg, Sub},
    sync::Arc,
};

#[cfg(feature = "par_braket")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    domains::DomainSection,
    signatures::WFSignature,
    vectorspace::{Field, VectorSpace},
};

pub trait Wavefunction<S: WFSignature> {
    fn f(&self, x: S::In) -> S::Out;
}

pub trait Ket<F: Field>: VectorSpace<F> {
    type Bra: Bra<F>;
    fn adjoint(self) -> Self::Bra;
    fn norm_sqr(&self) -> F;
}

pub trait Bra<F: Field>: VectorSpace<F> {
    type Ket: Ket<F>;
    fn apply(self, ket: Self::Ket) -> F;
}

#[derive(Clone)]
pub enum KetOperation<S: WFSignature> {
    Function {
        a: Arc<dyn Fn(S::In) -> S::Out + Send + Sync>,
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
    fn eval(&self, x: S::In) -> S::Out {
        match self {
            KetOperation::Function { a } => a(x),
            KetOperation::Add { a, b } => a.eval(x.clone()) + b.eval(x),
            KetOperation::Sub { a, b } => a.eval(x.clone()) - b.eval(x),
            KetOperation::Mul { a, b } => a.clone() * b.eval(x),
            KetOperation::Neg { a } => -a.eval(x),
            KetOperation::Adjoint { a } => a.eval(x).conjugate(),
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
    fn f(&self, x: S::In) -> <S as WFSignature>::Out {
        self.operation.eval(x)
    }
}

impl<S: WFSignature> Wavefunction<S> for WFBra<S> {
    fn f(&self, x: S::In) -> <S as WFSignature>::Out {
        self.operation.eval(x)
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

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            operation: KetOperation::Function {
                a: Arc::new(|_| S::Out::zero()),
            },
            domain: S::Dom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFKet {
            operation: KetOperation::Mul {
                a: c,
                b: Box::new(self.operation),
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

impl<S> VectorSpace<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFBra {
            operation: KetOperation::Function {
                a: Arc::new(|_| S::Out::zero()),
            },
            domain: S::Dom::none(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFBra {
            operation: KetOperation::Mul {
                a: c,
                b: Box::new(self.operation),
            },
            domain: self.domain,
        }
    }
}

impl<S: WFSignature> Mul<WFKet<S>> for WFBra<S> {
    type Output = S::Out;

    fn mul(self, rhs: WFKet<S>) -> Self::Output {
        self.apply(rhs)
    }
}

impl<S> Bra<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    type Ket = WFKet<S>;

    #[cfg(not(feature = "par_braket"))]
    fn apply(self, ket: Self::Ket) -> S::Out {
        let domain = ket.domain.clone() * self.domain.clone();
        domain
            .iter()
            .map(|x| S::mul_to_codomain(domain.step_size(), self.f(x.clone()) * ket.f(x)))
            .reduce(|a, b| a + b)
            .unwrap_or_else(|| S::Out::zero())
    }

    #[cfg(feature = "par_braket")]
    fn apply(self, ket: Self::Ket) -> S::Out {
        let domain = ket.domain.clone() * self.domain.clone();
        domain
            .iter()
            .par_bridge()
            .map(|x| S::mul_to_codomain(domain.step_size(), self.f(x.clone())) * ket.f(x))
            .reduce(|| S::Out::zero(), |a, b| a + b)
    }
}

impl<S> Ket<S::Out> for WFKet<S>
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

    fn norm_sqr(&self) -> S::Out {
        self.clone().adjoint().apply(self.clone())
    }
}
