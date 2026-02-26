use std::{f32::consts::PI, sync::Arc, time::Instant};

use num_complex::Complex32;

use crate::{
    braket::{Bra, Ket, KetOperation, WFKet},
    domains::DomainSection1D,
    signatures::WFSignature1D,
};

pub mod braket;
pub mod domains;
pub mod fields;
pub mod signatures;
pub mod vectorspace;

fn get_isw_eigenstate(width: f32, mass: f32, hbar: f32, n: u32) -> WFKet<WFSignature1D> {
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
            step_size: width / 10000.0,
        },
    }
}

fn main() {
    let start = Instant::now();

    let kets: [WFKet<WFSignature1D>; 8] =
        std::array::from_fn(|i| get_isw_eigenstate(1.0, 1.0, 1.0, (i + 1) as u32));

    let middle = Instant::now();

    for i in 0..8 {
        for j in 0..8 {
            let inner_prod = kets[i].clone().adjoint().apply(kets[j].clone(), 0.0);
            println!("{},{}: {}", i + 1, j + 1, inner_prod.norm());
        }
    }

    let finish = Instant::now();
    println!(
        "Alloc: {:?}, Compute: {:?}, Total: {:?}",
        middle - start,
        finish - middle,
        finish - start
    );
}
