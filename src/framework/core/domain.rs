//! Functionality for domains (input types to wavefunctions), and subdomains (subsets of domains where wavefunctions are defined)

mod domain_sect_1d;

pub use domain_sect_1d::{Domain1DIter, DomainSection1D};

use std::ops::{Add, Mul, Sub};

/// Trait describing properties of the domain of a wavefunction.
/// Note that partial ordering and addition are needed to iterate over the domain;
/// for finite domains, these can be defined by assigning an arbitrary ordering.
pub trait Domain:
    Sized + PartialOrd + Clone + Copy + Add<Output = Self> + Sub<Output = Self> + Send + Sync
{
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
    /// Translate this subdomain
    #[must_use]
    fn translate(self, offset: D) -> Self;
    /// Change step size.
    #[must_use]
    fn with_step_size(self, step_size: D) -> Self;
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
