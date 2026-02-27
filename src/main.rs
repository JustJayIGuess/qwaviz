//! A test program decomposing a quantum state into an eigenbasis.
#![deny(missing_docs)]

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspaces;

use std::{f32::consts::PI, sync::Arc, time::Instant};

use num_complex::Complex32;

use crate::{
    braket::{Ket, WFKet, WFOperation, Wavefunction},
    signatures::{WF1Space1Time, WFSignature},
    vectorspaces::VectorSpace,
};


type Ket1D = WFKet<WF1Space1Time>;
type SubDom = <WF1Space1Time as WFSignature>::SubDom;

fn get_isw_eigenstate(width: f32, mass: f32, hbar: f32, n: usize) -> Ket1D {
    Ket1D {
        wavefunction: WFOperation::Function(Arc::new(move |x, t| {
            let energy = (n as f32 * PI * hbar / width).powi(2) / (2.0 * mass);
            let coef = (2.0 / width).sqrt();
            let phase_x: f32 = (n as f32) * PI * x / width;
            coef * phase_x.sin() * Complex32::cis(-energy * t / hbar)
        })),
        subdomain: SubDom {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn get_expansion_state(width: f32) -> Ket1D {
    let psi0 = get_isw_eigenstate(width / 2.0, 1.0, 1.0, 1);
    Ket1D {
        wavefunction: WFOperation::Function(Arc::new(move |x, t| psi0.f(x, t))),
        subdomain: SubDom {
            lower: 0.0,
            upper: width,
            step_size: width / 1000.0,
        },
    }
}

fn main() {
    const MAX_N: usize = 128;

    let ket_0 = get_expansion_state(1.0);
    let eigenkets: [Ket1D; MAX_N] =
        std::array::from_fn(|i| get_isw_eigenstate(1.0, 1.0, 1.0, i + 1));

    let time_find_coefs = Instant::now();
    let coefs: Vec<Complex32> = eigenkets
        .iter()
        .map(|ket| ket.clone().adjoint() * ket_0.clone())
        .enumerate()
        .inspect(|(i, c)| println!("C_{}: {}", i + 1, c))
        .map(|(_, c)| c)
        .collect();
    println!("\nFind coefs: {:?}\n", time_find_coefs.elapsed());

    let time_construct_eigenket = Instant::now();
    let eigenbasis_ket = eigenkets
        .iter()
        .zip(coefs)
        .map(|(ket, c)| ket.clone().scale(c))
        .reduce(|a, b| a + b)
        .unwrap();
    println!(
        "Construct from eigenkets: {:?}\n",
        time_construct_eigenket.elapsed()
    );

    let time_recompute_coefs = Instant::now();
    eigenkets.iter().enumerate().for_each(|(i, ket)| {
        println!(
            "C_{}: {}",
            i + 1,
            ket.clone().adjoint() * eigenbasis_ket.clone()
        )
    });
    println!("\nRecompute coefs: {:?}\n", time_recompute_coefs.elapsed());
}
