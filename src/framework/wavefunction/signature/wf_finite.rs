use num_complex::Complex32;

use crate::framework::core::domain::finite_domains::FiniteSubDomain;

use super::super::super::core::domain::SubDomain1D;
use super::WFSignature;

/// Standard wavefunction signature for finite coordinates and 1 temporal dimension.
#[derive(Clone)]
pub struct WFFinite;

impl WFSignature for WFFinite {
    type Space = i32;
    type Time = f32;
    type Out = Complex32;
    type SubDom = FiniteSubDomain;

    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out {
        (a as f32) * b
    }
}
