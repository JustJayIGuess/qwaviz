use core::f32;
use num_complex::{Complex, Complex32};
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
};

trait Field:
    Sized
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn inv(&self) -> Option<Self>;
    fn is_zero(&self) -> bool;
    fn conjugate(self) -> Self;
}

trait Domain: PartialOrd + Clone + Add<Output = Self> {
    fn first() -> Self;
    fn last() -> Self;
    fn zero() -> Self;
}
trait DomainSection<D: Domain>: Clone + IntoIterator + Add<Output = Self> + Mul<Output = Self> {
    fn contains(&self, x: D) -> bool;
    fn all() -> Self;
    fn none() -> Self;
}

trait VectorSpace<F: Field>:
    Clone
    + Add<Output = Self>
    + Add<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn scale(self, c: F) -> Self;
}

trait Ket<F: Field>: VectorSpace<F> {
    type Bra: Bra<F>;
    fn adjoint(self) -> Self::Bra;
    fn norm_sqr(&self) -> F;
}

trait Bra<F: Field>: VectorSpace<F> {
    type Ket: Ket<F>;
    fn apply(&self, ket: &Self::Ket) -> F;
}

struct DomainSection1D<D: Domain> {
    lower: D,
    upper: D,
    step_size: D,
}

struct Domain1DIter<D: Domain> {
    lower: D,
    upper: D,
    step_size: D,
    value: D,
}

impl<D: Domain> Domain1DIter<D> {
    fn new(domain: &DomainSection1D<D>, step_size: D) -> Domain1DIter<D> {
        Domain1DIter {
            lower: domain.lower.clone(),
            upper: domain.upper.clone(),
            step_size,
            value: domain.lower.clone(),
        }
    }
}

impl<D: Domain> Iterator for Domain1DIter<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value >= self.upper {
            return None;
        }

        let res = self.value.clone();
        self.value = self.value.clone() + self.step_size.clone();
        Some(res)
    }
}

impl<D: Domain> IntoIterator for DomainSection1D<D> {
    type Item = D;

    type IntoIter = Domain1DIter<D>;

    fn into_iter(self) -> Self::IntoIter {
        Domain1DIter::<D>::new(&self, self.step_size.clone())
    }
}

impl<D: Domain> Clone for DomainSection1D<D> {
    fn clone(&self) -> Self {
        Self {
            lower: self.lower.clone(),
            upper: self.upper.clone(),
            step_size: self.step_size.clone(),
        }
    }
}

impl<D: Domain> DomainSection<D> for DomainSection1D<D> {
    fn contains(&self, x: D) -> bool {
        self.lower <= x && x <= self.upper
    }

    fn all() -> Self {
        Self {
            lower: D::first(),
            upper: D::last(),
            step_size: D::last(),
        }
    }

    fn none() -> Self {
        Self {
            lower: D::zero(),
            upper: D::zero(),
            step_size: D::last(),
        }
    }
}

impl<D: Domain> Add for DomainSection1D<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DomainSection1D {
            lower: if self.lower < rhs.lower {
                self.lower
            } else {
                rhs.lower
            },
            upper: if self.upper > rhs.upper {
                self.upper
            } else {
                rhs.upper
            },
            step_size: if self.step_size < rhs.step_size {
                self.step_size
            } else {
                rhs.step_size
            },
        }
    }
}

impl<D: Domain> Mul for DomainSection1D<D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        DomainSection1D {
            lower: if self.lower > rhs.lower {
                self.lower
            } else {
                rhs.lower
            },
            upper: if self.upper < rhs.upper {
                self.upper
            } else {
                rhs.upper
            },
            step_size: if self.step_size < rhs.step_size {
                self.step_size
            } else {
                rhs.step_size
            },
        }
    }
}

struct WFKet<In: Domain + 'static, Out: Field + 'static, D: DomainSection<In>> {
    f: Rc<dyn Fn(In) -> Out>,
    domain: D,
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

struct WFBra<In, Out, D>
where
    In: Domain + 'static,
    Out: Field + 'static,
    D: DomainSection<In>,
{
    f: Rc<dyn Fn(In) -> Out>,
    domain: D,
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
        todo!()
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

impl Field for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(1.0 / *self)
        }
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn conjugate(self) -> Self {
        self
    }
}

impl Field for Complex32 {
    fn zero() -> Self {
        Complex32::new(0.0, 0.0)
    }

    fn one() -> Self {
        Complex32::new(1.0, 0.0)
    }

    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(self.inv())
        }
    }

    fn is_zero(&self) -> bool {
        *self == Complex32::ZERO
    }

    fn conjugate(self) -> Self {
        self.conj()
    }
}

impl Domain for f32 {
    fn first() -> Self {
        f32::NEG_INFINITY
    }

    fn last() -> Self {
        f32::INFINITY
    }

    fn zero() -> Self {
        0.0
    }
}

fn main() {
    let ket: WFKet<f32, Complex32, DomainSection1D<f32>> = WFKet {
        f: Rc::new(|x| Complex32::new(x, 0.0)),
        domain: DomainSection1D {
            lower: 0.0,
            upper: 1.0,
            step_size: 0.1,
        },
    };

    println!("Hello, world!");
}
