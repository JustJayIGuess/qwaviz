use std::rc::Rc;

use num_complex::Complex32;

use crate::{
    braket::{Bra, Ket, WFKet}, domains::DomainSection1D,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod vectorspace;

fn main() {
    let ket: WFKet<f32, Complex32, DomainSection1D<f32>> = WFKet {
        f: Rc::new(|x| Complex32::new(x, 0.0)),
        domain: DomainSection1D {
            lower: 0.0,
            upper: 1.0,
            step_size: 0.01,
        },
    };

    let norm = ket.clone().adjoint().apply(&ket).sqrt();

    println!("Norm: {}", norm);
}
