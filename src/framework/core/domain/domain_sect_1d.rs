use std::ops::{Add, Mul};

use super::{Domain, SubDomain};

#[derive(Clone, Debug)]
/// A subdomain in one dimension for an arbitrary domain D
pub struct SubDomain1D<D: Domain> {
    /// The lower bound of the subdomain
    pub lower: D,
    /// The upper bound of the subdomain
    pub upper: D,
}

/// An iterator over a 1D subdomain.
pub struct SubDomain1DIter<D: Domain> {
    pub(super) upper: D,
    pub(super) step_size: D,
    pub(super) value: D,
}

impl<D: Domain> SubDomain1DIter<D> {
    fn new(domain: &SubDomain1D<D>, step_size: D) -> SubDomain1DIter<D> {
        SubDomain1DIter {
            upper: domain.upper,
            step_size,
            value: domain.lower,
        }
    }
}

impl<D: Domain> Iterator for SubDomain1DIter<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value > self.upper {
            return None;
        }

        let res = self.value;
        self.value = self.value + self.step_size;
        Some(res)
    }
}

impl<D: Domain> SubDomain<D> for SubDomain1D<D> {
    fn contains(&self, x: D) -> bool {
        self.lower <= x && x <= self.upper
    }

    fn all() -> Self {
        Self {
            lower: D::first(),
            upper: D::last(),
        }
    }

    fn none() -> Self {
        Self {
            lower: D::zero(),
            upper: D::zero(),
        }
    }

    fn iter_with_step_size(&self, step_size: D) -> impl Iterator<Item = D> {
        SubDomain1DIter::new(self, step_size)
    }

    fn translate(self, offset: D) -> Self {
        Self {
            lower: self.lower + offset,
            upper: self.upper + offset,
        }
    }
}

impl<D: Domain> Add for SubDomain1D<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SubDomain1D {
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
        }
    }
}

impl<D: Domain> Mul for SubDomain1D<D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SubDomain1D {
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
        }
    }
}
