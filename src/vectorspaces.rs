use std::{
    ops::{Add, Neg, Sub},
    sync::Arc,
};

use crate::{
    braket::{KetOperation, WFBra, WFKet},
    domains::DomainSection,
    fields::Field,
    signatures::WFSignature,
};

pub trait VectorSpace<F: Field>:
    Clone + Add<Output = Self> + Sub<Output = Self> + Neg<Output = Self>
{
    fn zero() -> Self;
    fn scale(self, c: F) -> Self;
}

impl<S> VectorSpace<S::Out> for WFKet<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFKet {
            operation: KetOperation::Function {
                a: Arc::new(|_, _| S::Out::zero()),
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

impl<S> VectorSpace<S::Out> for WFBra<S>
where
    S: WFSignature,
{
    fn zero() -> Self {
        WFBra {
            operation: KetOperation::Function {
                a: Arc::new(|_, _| S::Out::zero()),
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
