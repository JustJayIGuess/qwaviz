use std::ops::{Add, Mul};

pub trait Domain: PartialOrd + Clone + Add<Output = Self> {
    fn first() -> Self;
    fn last() -> Self;
    fn zero() -> Self;
}
pub trait DomainSection<D: Domain>:
    Clone + Add<Output = Self> + Mul<Output = Self>
{
    type Iter: Iterator<Item = D>;

    fn contains(&self, x: D) -> bool;
    fn all() -> Self;
    fn none() -> Self;
    fn iter(&self) -> Self::Iter;
    fn step_size(&self) -> D;
}

pub struct DomainSection1D<D: Domain> {
    pub lower: D,
    pub upper: D,
    pub step_size: D,
}

pub struct Domain1DIter<D: Domain> {
    upper: D,
    step_size: D,
    value: D,
}

impl<D: Domain> Domain1DIter<D> {
    fn new(domain: &DomainSection1D<D>, step_size: D) -> Domain1DIter<D> {
        Domain1DIter {
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

// impl<D: Domain> IntoIterator for DomainSection1D<D> {
//     type Item = D;

//     type IntoIter = Domain1DIter<D>;

//     fn into_iter(self) -> Self::IntoIter {
//         Domain1DIter::new(&self, self.step_size.clone())
//     }
// }

// impl<'a, D: Domain> IntoIterator for &'a DomainSection1D<D> {
//     type Item = D;
//     type IntoIter = Domain1DIter<D>;

//     fn into_iter(self) -> Self::IntoIter {
//         Domain1DIter::new(self, self.step_size.clone())
//     }
// }

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
    type Iter = Domain1DIter<D>;

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
    
    
    fn iter(&self) -> Self::Iter {
        Domain1DIter::new(self, self.step_size.clone())
    }
    
    fn step_size(&self) -> D {
        self.step_size.clone()
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
