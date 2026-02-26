use std::{f32::consts::PI, rc::Rc};

use num_complex::{Complex32, ComplexFloat};

use crate::{
    braket::{Bra, Ket, WFKet},
    domains::DomainSection1D,
    signatures::{WFSignature, WFSignature1D},
    vectorspace::Field,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspace;

fn get_isw_eigenstate(
    width: &'static f32,
    mass: &'static f32,
    hbar: &'static f32,
    n: &'static u32,
    t: &'static f32,
) -> WFKet<WFSignature1D> {
    WFKet {
        f: Rc::new(|x| {
            let (width, mass, hbar, n, t) = (*width, *mass, *hbar, *n as f32, *t);
            let energy = (n * PI * hbar / width).powi(2) / (2.0 * mass);
            let coef = (2.0 / width).sqrt();
            let phase_x = n * PI * x / width;
            coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
        }),
        domain: DomainSection1D {
            lower: 0.0,
            upper: *width,
            step_size: *width / 100.0,
        },
    }
}

fn main() {
    const width: f32 = 1.0;
    const mass: f32 = 1.0;
    const hbar: f32 = 1.0;
    const n: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    const t: f32 = 1.0;

    let kets: [WFKet<WFSignature1D>; 8] =
        std::array::from_fn(|i| get_isw_eigenstate(&width, &mass, &hbar, &n[i], &t));

    for i in 0..8 {
        for j in 0..8 {
            let iprod = kets[i].clone().adjoint().apply(&kets[j]);
            println!("{},{}: {}", i+1, j+1, iprod.norm());
        }
    }
}
