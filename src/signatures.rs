use num_complex::Complex32;

use crate::{
    domains::{Domain, DomainSection, DomainSection1D},
    vectorspace::Field,
};

pub trait WFSignature: Clone {
    type In: Domain + Send + Sync;
    type Out: Field + Send + Sync;
    type Dom: DomainSection<Self::In> + Send + Sync;
    fn mul_to_codomain(a: Self::In, b: Self::Out) -> Self::Out;
}

#[derive(Clone)]
pub struct WFSignature1D;

impl WFSignature for WFSignature1D {
    type In = f32;

    type Out = Complex32;

    type Dom = DomainSection1D<Self::In>;

    fn mul_to_codomain(a: Self::In, b: Self::Out) -> Self::Out {
        a * b
    }
}
