//! Signatures are used to specify typing and type interactions of function vector spaces

use num_complex::Complex32;

use crate::{
    domains::{Domain, DomainSection1D, SubDomain},
    fields::Field,
};

/// The type signature for a wavefunction.
///
pub trait WFSignature: Clone {
    /// The type associated with the spatial domain of the wavefunction
    type Space: Domain + Send + Sync;
    /// The type associated with the time input to the wavefunction
    type Time: Domain + Send + Sync;
    /// The output type of the wavefunction. This is also the field over which wavefunctions will form a vectorspace
    type Out: Field + Send + Sync;
    /// The type implementing functionality for handling subsets of the domain.
    type SubDom: SubDomain<Self::Space> + Send + Sync;
    /// Combine elements in space with wavefunction output.
    /// This defines how to multiply integrands by d(space) when computing inner products.
    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out;
}

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
