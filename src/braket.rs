use std::{
    ops::{Add, Neg, Sub},
    rc::Rc,
};

use crate::{
    domains::{Domain, DomainSection},
    vectorspace::{Field, VectorSpace},
};

pub trait WFSignature<In, Out>
where
    In: Domain,
    Out: Field,
{
    fn mul_to_codomain(a: In, b: Out) -> Out;
}

pub struct WFKet<In, Out, D>
where
    In: Domain + 'static,
    Out: Field + 'static,
    D: DomainSection<In>,
{
    pub f: Rc<dyn Fn(In) -> Out>,
    pub domain: D,
}

pub struct WFBra<In, Out, D>
where
    In: Domain + 'static,
    Out: Field + 'static,
    D: DomainSection<In>,
{
    f: Rc<dyn Fn(In) -> Out>,
    domain: D,
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

impl<In, Out, D> Add for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let f = self.f;
        let g = rhs.f;
        WFKet {
            f: Rc::new(move |x: In| f(x.clone()) + g(x)),
            domain: self.domain + rhs.domain,
        }
    }
}

impl<In, Out, D> Sub for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let f = self.f;
        let g = rhs.f;
        WFKet {
            f: Rc::new(move |x: In| f(x.clone()) - g(x)),
            domain: self.domain + rhs.domain,
        }
    }
}

impl<In, Out, D> Neg for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let f = self.f;
        WFKet {
            f: Rc::new(move |x: In| -f(x)),
            domain: self.domain,
        }
    }
}

impl<In, Out, D> Clone for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    fn clone(&self) -> Self {
        WFKet {
            f: self.f.clone(),
            domain: self.domain.clone(),
        }
    }
}

impl<In, Out, D> VectorSpace<Out> for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    fn zero() -> Self {
        WFKet {
            f: Rc::new(|_| Out::zero()),
            domain: D::all(),
        }
    }

    fn scale(self, c: Out) -> Self {
        WFKet {
            f: Rc::new(move |x| c.clone() * (self.f)(x)),
            domain: self.domain,
        }
    }
}

impl<In, Out, D> Add for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let f = self.f;
        let g = rhs.f;
        WFBra {
            f: Rc::new(move |x: In| f(x.clone()) + g(x)),
            domain: self.domain + rhs.domain,
        }
    }
}

impl<In, Out, D> Sub for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let f = self.f;
        let g = rhs.f;
        WFBra {
            f: Rc::new(move |x: In| f(x.clone()) - g(x)),
            domain: self.domain + rhs.domain,
        }
    }
}

impl<In, Out, D> Neg for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let f = self.f;
        WFBra {
            f: Rc::new(move |x: In| -f(x)),
            domain: self.domain,
        }
    }
}

impl<In, Out, D> Clone for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    fn clone(&self) -> Self {
        WFBra {
            f: self.f.clone(),
            domain: self.domain.clone(),
        }
    }
}

impl<In, Out, D> VectorSpace<Out> for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    fn zero() -> Self {
        WFBra {
            f: Rc::new(|_| Out::zero()),
            domain: D::none(),
        }
    }

    fn scale(self, c: Out) -> Self {
        WFBra {
            f: Rc::new(move |x| c.clone() * (self.f)(x)),
            domain: self.domain,
        }
    }
}

impl<In, Out, D> Bra<Out> for WFBra<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Ket = WFKet<In, Out, D>;

    fn apply(&self, ket: &Self::Ket) -> Out {
        todo!();
        let mut res = Out::zero();
        let domain = ket.domain.clone() * self.domain.clone();
        for x in domain.iter() {
            res = res + domain.step_size() * ((self.f)(x.clone()) * (ket.f)(x));
        }
        res
    }
}

impl<In, Out, D> Ket<Out> for WFKet<In, Out, D>
where
    In: Domain,
    Out: Field,
    D: DomainSection<In>,
{
    type Bra = WFBra<In, Out, D>;

    fn adjoint(self) -> Self::Bra {
        Self::Bra {
            f: Rc::new(move |x| (self.f)(x).conjugate()),
            domain: self.domain,
        }
    }

    fn norm_sqr(&self) -> Out {
        self.clone().adjoint().apply(&self)
    }
}
