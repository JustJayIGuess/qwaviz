//! Implementation of finite subdomains.
//! E.g., a two-state system has subdomain {A, B}, represented here by the range (0..=1).

use std::ops::{Add, Mul};

use super::{Domain, SubDomain, SubDomain1DIter};

// pub enum BinaryDomain {
//     A,
//     B
// }

/// A subdomain with finitely many coordinates
#[derive(Clone)]
pub struct FiniteSubDomain {
    /// The smallest domain index in the subdomain
    pub min_idx: i32,
    /// The biggest domain index in the subdomain
    pub max_idx: i32,
}

impl SubDomain<i32> for FiniteSubDomain {
    fn contains(&self, x: i32) -> bool {
        self.min_idx <= x && x <= self.max_idx
    }

    fn all() -> Self {
        Self {
            min_idx: i32::MIN,
            max_idx: i32::MAX,
        }
    }

    fn none() -> Self {
        Self {
            min_idx: 0,
            max_idx: 0,
        }
    }

    fn iter(&self) -> impl Iterator<Item = i32> + Sized + Send + Sync {
        SubDomain1DIter::<i32> {
            upper: self.max_idx,
            step_size: 1,
            value: self.min_idx,
        }
    }

    fn step_size(&self) -> i32 {
        1
    }

    fn translate(self, offset: i32) -> Self {
        Self {
            min_idx: self.min_idx + offset,
            max_idx: self.max_idx + offset,
        }
    }

    fn with_step_size(self, step_size: i32) -> Self {
        Self {
            min_idx: self.min_idx,
            max_idx: self.max_idx,
        }
    }

    fn into_iter(self) -> impl Iterator<Item = i32> + Sized + Send + Sync {
        SubDomain1DIter::<i32> {
            upper: self.max_idx,
            step_size: 1,
            value: self.min_idx,
        }
    }
}

impl Mul for FiniteSubDomain {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            min_idx: if self.min_idx > rhs.min_idx {
                self.min_idx
            } else {
                rhs.min_idx
            },
            max_idx: if self.max_idx < rhs.max_idx {
                self.max_idx
            } else {
                rhs.max_idx
            },
        }
    }
}

impl Add for FiniteSubDomain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min_idx: if self.min_idx < rhs.min_idx {
                self.min_idx
            } else {
                rhs.min_idx
            },
            max_idx: if self.max_idx > rhs.max_idx {
                self.max_idx
            } else {
                rhs.max_idx
            },
        }
    }
}
