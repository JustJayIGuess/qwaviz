//! Bra-ket notation, where bras are adjoints of kets and vice versa. Each form a vector space, with
//! bras living in the dual space of kets. Applying a bra to a ket results in an inner product.

mod operations;
mod wf_bra;
mod wf_ket;

pub use operations::WFFunc;
pub use operations::WFOperation;
pub use wf_bra::Bra;
pub use wf_ket::Ket;

use super::{core::vectorspace::VectorSpace, wavefunction::signature::WFSignature};

/// A ket (vector) in a function vectorspace
pub trait AbstractKet<S: WFSignature>: VectorSpace<S::Out> {
    /// The corresponding bra (covector) type
    type Bra: AbstractBra<S>;
    /// Convert to corresponding bra (covector)
    fn to_adjoint(self) -> Self::Bra;
    /// Create corresponding bra (covector) of a ket (vector)
    fn adjoint(ket: &Self) -> Self::Bra;
    /// Compute the squared norm using the standard inner product
    fn norm_sqr(&self, t: S::Time, step_size: S::Space) -> S::Out;
}

/// A bra (covector) in the dual of a function vectorspace
pub trait AbstractBra<S: WFSignature>: VectorSpace<S::Out> {
    /// The corresponding ket (vector) type
    type Ket: AbstractKet<S>;
    /// Apply this bra (covector) to a ket (vector) to produce an element of the field.
    fn apply(&self, ket: &Self::Ket, t: S::Time, step_size: S::Space) -> S::Out;
}
