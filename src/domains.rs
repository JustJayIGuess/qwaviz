//! Functionality for domains (input types to wavefunctions), and subdomains (subsets of domains where wavefunctions are defined)
use std::ops::{Add, Mul};

/// Trait describing properties of the domain of a wavefunction.
/// Note that partial ordering and addition are needed to iterate over the domain;
/// for finite domains, these can be defined by assigning an arbitrary ordering.
pub trait Domain: Sized + PartialOrd + Clone + Copy + Add<Output = Self> + Send + Sync {
    /// The lower bound of the domain
    fn first() -> Self;
    /// The upper bound of the domain
    fn last() -> Self;
    /// The zero of the domain
    fn zero() -> Self;
}

/// Trait describing properties of a subset of a domain. Used largely for integration.
pub trait SubDomain<D: Domain>: Clone + Add<Output = Self> + Mul<Output = Self> {
    /// Check if a point is contained in this subset.
    fn contains(&self, x: D) -> bool;
    /// The entire domain
    fn all() -> Self;
    /// An empty subdomain
    fn none() -> Self;
    /// Return an iterator over this subdomain
    fn iter(&self) -> impl Iterator<Item = D> + Sized + Send + Sync;
    /// The step size in this domain. Used as volume element in integration.
    fn step_size(&self) -> D;
}

#[derive(Clone)]
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
