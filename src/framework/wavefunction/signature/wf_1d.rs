//! Implementation of wavefunction signature for 1D domains.

use num_complex::Complex32;

use super::super::super::core::domain::SubDomain1D;
use super::WFSignature;

/// Standard wavefunction signature for 1 spatial dimension and 1 temporal dimension.
#[derive(Clone)]
pub struct Sign1D;

impl WFSignature for Sign1D {
    type Space = f32;
    type Time = f32;
    type Out = Complex32;
    type SubDom = SubDomain1D<Self::Space>;

    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out {
        a * b
    }
}
