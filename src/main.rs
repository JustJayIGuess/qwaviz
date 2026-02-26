use std::{f32::consts::PI, sync::Arc, time::Instant};

use num_complex::Complex32;

use crate::{
    braket::{Ket, KetOperation, WFKet, Wavefunction},
    domains::DomainSection1D,
    signatures::WF1Space1Time,
    vectorspace::VectorSpace,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspace;

fn get_isw_eigenstate(width: f32, mass: f32, hbar: f32, n: u32) -> WFKet<WF1Space1Time> {
    WFKet {
        operation: KetOperation::Function {
            a: Arc::new(move |x, t| {
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

fn get_expansion_state(width: f32) -> WFKet<WF1Space1Time> {
    let psi0 = get_isw_eigenstate(width / 2.0, 1.0, 1.0, 1);
    WFKet {
        operation: KetOperation::Function {
            a: Arc::new(move |x, t| psi0.f(x, t)),
        },
        domain: DomainSection1D {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn main() {
    const MAX_EIGENSTATES: usize = 64;

    let ket_0 = get_expansion_state(1.0);
    let eigenkets: [WFKet<WF1Space1Time>; MAX_EIGENSTATES] =
        std::array::from_fn(|i| get_isw_eigenstate(1.0, 1.0, 1.0, (i + 1) as u32));
    let mut coefs: [Complex32; MAX_EIGENSTATES] = [Complex32::ZERO; MAX_EIGENSTATES];

    let time_find_coefs = Instant::now();
    for (i, ket) in eigenkets.iter().enumerate() {
        coefs[i] = ket.clone().adjoint() * ket_0.clone();
        println!("C_{}: {}", i + 1, coefs[i]);
    }
    println!("Find coefs: {:?}", Instant::now() - time_find_coefs);

    let eigenbasis_ket = eigenkets
        .iter()
        .zip(coefs)
        .map(|(ket, c)| ket.clone().scale(c))
        .reduce(|a, b| a + b)
        .unwrap();

    let time_recompute_coefs = Instant::now();
    for (i, ket) in eigenkets.iter().enumerate() {
        let coef = ket.clone().adjoint() * eigenbasis_ket.clone();
        println!("C_{}: {}", i + 1, coef);
    }
    println!(
        "Recompute coefs: {:?}",
        Instant::now() - time_recompute_coefs
    );
}
