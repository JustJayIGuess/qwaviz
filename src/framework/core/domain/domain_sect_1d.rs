use std::ops::{Add, Mul};

use super::{Domain, SubDomain};

#[derive(Clone, Debug)]
/// A subdomain in one dimension for an arbitrary domain D
pub struct DomainSection1D<D: Domain> {
    /// The lower bound of the subdomain
    pub lower: D,
    /// The upper bound of the subdomain
    pub upper: D,
    /// The step size in this subdomain
    pub step_size: D,
}

/// An iterator over a 1D subdomain.
pub struct Domain1DIter<D: Domain> {
    upper: D,
    step_size: D,
    value: D,
}

impl<D: Domain> Domain1DIter<D> {
    fn new(domain: &DomainSection1D<D>, step_size: D) -> Domain1DIter<D> {
        Domain1DIter {
            upper: domain.upper,
            step_size,
            value: domain.lower,
        }
    }
}

impl<D: Domain> Iterator for Domain1DIter<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value >= self.upper {
            return None;
        }

        let res = self.value;
        self.value = self.value + self.step_size;
        Some(res)
    }
}

impl<D: Domain> SubDomain<D> for DomainSection1D<D> {
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

    fn iter(&self) -> impl Iterator<Item = D> {
        Domain1DIter::new(self, self.step_size)
    }

    fn step_size(&self) -> D {
        self.step_size
    }

    fn translate(self, offset: D) -> Self {
        Self {
            lower: self.lower + offset,
            upper: self.upper + offset,
            step_size: self.step_size,
        }
    }

    fn with_step_size(self, step_size: D) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper,
            step_size,
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
