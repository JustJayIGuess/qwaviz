use num_complex::Complex32;

use super::super::super::core::domain::DomainSection1D;
use super::WFSignature;

/// Standard wavefunction signature for 1 spatial dimension and 1 temporal dimension.
#[derive(Clone)]
pub struct WF1Space1Time;

impl WFSignature for WF1Space1Time {
    type Space = f32;
    type Time = f32;
    type Out = Complex32;
    type SubDom = DomainSection1D<Self::Space>;

    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out {
        a * b
    }
}
