use std::{
    ops::{Add, Neg, Sub},
    rc::Rc,
};

use crate::{
    domains::DomainSection,
    signatures::WFSignature,
    vectorspace::{Field, VectorSpace},
};

pub struct WFKet<S>
where
    S: WFSignature,
{
    pub f: Rc<dyn Fn(S::In) -> S::Out>,
    pub domain: S::Dom,
}

pub struct WFBra<S>
where
    S: WFSignature,
{
    f: Rc<dyn Fn(S::In) -> S::Out>,
    domain: S::Dom,
}

pub trait Ket<F: Field>: VectorSpace<F> {
    type Bra: Bra<F>;
    fn adjoint(self) -> Self::Bra;
    fn norm_sqr(&self) -> F;
}

pub trait Bra<F: Field>: VectorSpace<F> {
    type Ket: Ket<F>;
    fn apply(&self, ket: &Self::Ket) -> F;
}

impl<S> Add for WFKet<S>
where
    S: WFSignature,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let f = self.f;
        let g = rhs.f;
        WFKet {
            f: Rc::new(move |x: S::In| f(x.clone()) + g(x)),
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
        let f = self.f;
        let g = rhs.f;
        WFKet {
            f: Rc::new(move |x: S::In| f(x.clone()) - g(x)),
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
        let f = self.f;
        WFKet {
            f: Rc::new(move |x: S::In| -f(x)),
            domain: self.domain,
        }
    }
}

impl<S> Clone for WFKet<S>
where
    S: WFSignature,
{
    fn clone(&self) -> Self {
        WFKet {
            f: self.f.clone(),
            domain: self.domain.clone(),
        }
    }
}

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            f: Rc::new(|_| S::Out::zero()),
            domain: S::Dom::all(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFKet {
            f: Rc::new(move |x| c.clone() * (self.f)(x)),
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
        let f = self.f;
        let g = rhs.f;
        WFBra {
            f: Rc::new(move |x: S::In| f(x.clone()) + g(x)),
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
        let f = self.f;
        let g = rhs.f;
        WFBra {
            f: Rc::new(move |x: S::In| f(x.clone()) - g(x)),
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
        let f = self.f;
        WFBra {
            f: Rc::new(move |x: S::In| -f(x)),
            domain: self.domain,
        }
    }
}

impl<S> Clone for WFBra<S>
where
    S: WFSignature,
{
    fn clone(&self) -> Self {
        WFBra {
            f: self.f.clone(),
            domain: self.domain.clone(),
        }
    }
}

impl<S> VectorSpace<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFBra {
            f: Rc::new(|_| S::Out::zero()),
            domain: S::Dom::none(),
        }
    }

    fn scale(self, c: S::Out) -> Self {
        WFBra {
            f: Rc::new(move |x: S::In| c.clone() * (self.f)(x)),
            domain: self.domain,
        }
    }
}

impl<S> Bra<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    type Ket = WFKet<S>;

    fn apply(&self, ket: &Self::Ket) -> S::Out {
        let mut res = S::Out::zero();
        let domain = ket.domain.clone() * self.domain.clone();
        for x in domain.iter() {
            res = res + S::mul_to_codomain(domain.step_size(), (self.f)(x.clone()) * (ket.f)(x));
        }
        res
    }
}

impl<S> Ket<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    type Bra = WFBra<S>;

    fn adjoint(self) -> Self::Bra {
        Self::Bra {
            f: Rc::new(move |x| (self.f)(x).conjugate()),
            domain: self.domain,
        }
    }

    fn norm_sqr(&self) -> S::Out {
        self.clone().adjoint().apply(&self)
    }
}
