use num_complex::Complex32;

use crate::{
    domains::{Domain, DomainSection, DomainSection1D},
    vectorspace::Field,
};

pub trait WFSignature: Clone {
    type Space: Domain + Send + Sync;
    type Time: Domain + Send + Sync;
    type Out: Field + Send + Sync;
    type Dom: DomainSection<Self::Space> + Send + Sync;
    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out;
}

#[derive(Clone)]
pub struct WFSignature1D;

impl WFSignature for WFSignature1D {
    type Space = f32;
    type Time = f32;
    type Out = Complex32;
    type Dom = DomainSection1D<Self::Space>;

    fn mul_to_codomain(a: Self::Space, b: Self::Out) -> Self::Out {
        a * b
    }
}
