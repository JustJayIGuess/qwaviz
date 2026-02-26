use std::{f32::consts::PI, rc::Rc};

use num_complex::Complex32;

use crate::{
    braket::{Ket, KetOperation, WFKet},
    domains::DomainSection1D,
    signatures::WFSignature1D,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspace;

fn get_isw_eigenstate(width: f32, mass: f32, hbar: f32, n: u32, t: f32) -> WFKet<WFSignature1D> {
    WFKet {
        operation: KetOperation::Function {
            a: Rc::new(move |x| {
                let energy = (n as f32 * PI * hbar / width).powi(2) / (2.0 * mass);
                let coef = (2.0 / width).sqrt();
                let phase_x = n as f32 * PI * x / width;
                coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
            }),
        },
        domain: DomainSection1D {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn main() {
    let kets: [WFKet<WFSignature1D>; 8] =
        std::array::from_fn(|i| get_isw_eigenstate(1.0, 1.0, 1.0, i as u32, 0.0));

    for i in 0..8 {
        for j in 0..8 {
            let inner_prod = kets[i].clone().adjoint() * kets[j].clone();
            println!("{},{}: {}", i + 1, j + 1, inner_prod.norm());
        }
    }
}
