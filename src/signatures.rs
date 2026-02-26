use num_complex::Complex32;

use crate::{
    domains::{Domain, DomainSection, DomainSection1D},
    vectorspace::Field,
};

pub trait WFSignature {
    type In: Domain + 'static;
    type Out: Field + 'static;
    type Dom: DomainSection<Self::In>;
    fn mul_to_codomain(a: Self::In, b: Self::Out) -> Self::Out;
}

pub struct WFSignature1D;

impl WFSignature for WFSignature1D {
    type In = f32;

    type Out = Complex32;

    type Dom = DomainSection1D<Self::In>;

    fn mul_to_codomain(a: Self::In, b: Self::Out) -> Self::Out {
        a * b
    }
}
